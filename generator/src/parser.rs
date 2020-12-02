use std::path::Path;
use std::io::{BufReader, BufRead};
use std::fs::File;
use std::iter::Peekable;
use std::str::Chars;
use anyhow::{Result, Context, anyhow};

pub struct Spec {
    pub imp: Impl,
    pub methods: Vec<Method>,
    pub extends: Option<String>,
    pub cfg: Option<String>,
    pub target_name: Option<String>
}

pub type Params = Vec<(String, Option<String>)>;

pub struct Method {
    pub name: String,
    pub ty_params: Params,
    pub params: Params,
    pub where_clause: String,
}

pub struct Impl {
    pub ty_params: Params,
    pub ty: String,
}

pub fn parse(file: &Path) -> Result<Spec> {
    let mut lines = vec!();
    for (n, line) in BufReader::new(File::open(file).unwrap()).lines().enumerate() {
        lines.push((n, line?));
    }
    let mut parser = LineParser {
        imp: None,
        extends: None,
        cfg: None,
        target_name: None,
        methods: vec!(),
    };
    parser.parse(&lines).with_context(|| anyhow!("cannot parse {}", file.display()))?;
    Ok(Spec {
        imp: parser.imp.unwrap(),
        methods: parser.methods,
        extends: parser.extends,
        cfg: parser.cfg,
        target_name: parser.target_name,
    })
}

struct LineParser {
    imp: Option<Impl>,
    cfg: Option<String>,
    target_name: Option<String>,
    methods: Vec<Method>,
    extends: Option<String>,
}

impl LineParser {
    fn parse(&mut self, lines: &[(usize, String)]) -> Result<()> {
        for &(ref line, ref content) in lines {
            self.parse_line(content).with_context(|| anyhow!("cannot parse line {}", line))?;
        }
        Ok(())
    }

    fn parse_line(&mut self, line: &str) -> Result<()> {
        let (directive, arg) = match line.find(' ') {
            Some(pos) => line.split_at(pos),
            _ => (line, ""),
        };
        match directive.trim() {
            "#" => { },
            "target_name" => self.target_name = Some(arg.trim().to_string()),
            "impl" => self.imp = Some(self.parse_impl(arg)?),
            "extends" => self.extends = Some(arg.trim().to_string()),
            "cfg" => self.cfg = Some(arg.trim().to_string()),
            "fn" => {
                let method = self.parse_method(arg)?;
                self.methods.push(method);
            },
            _ => return Err(anyhow!("unknown directive {}", directive)),
        }
        Ok(())
    }

    fn parse_method(&mut self, method: &str) -> Result<Method> {
        RustParser { chars: method.chars().peekable() }.parse_fn_decl()
    }

    fn parse_impl(&mut self, params: &str) -> Result<Impl> {
        RustParser { chars: params.chars().peekable() }.parse_impl_decl()
    }
}

struct RustParser<'a> {
    chars: Peekable<Chars<'a>>,
}

impl<'a> RustParser<'a> {
    fn slurp(&mut self) -> String {
        self.skip_whitespace();
        let mut res = self.chars.clone().collect();
        trim_whitespace(&mut res);
        res
    }

    fn parse_params(&mut self, start: char, stop: char) -> Result<Vec<(String, Option<String>)>> {
        self.skip_whitespace();
        assert_eq!(Some(start), self.chars.next());
        let mut param = vec!();
        loop {
            self.skip_whitespace();
            match self.chars.peek() {
                Some(&c) if c == stop => {
                    self.chars.next();
                    break;
                }
                None => return Err(anyhow!("missing '{}'", stop)),
                _ => { },
            }
            param.push(self.parse_param(stop)?);
            match self.chars.next() {
                Some(c) if c == stop => break,
                Some(',') => { },
                _ => return Err(anyhow!("unterminated parameter")),
            }
        }
        Ok(param)
    }

    fn parse_impl_decl(&mut self) -> Result<Impl> {
        self.skip_whitespace();
        let ty_params = match self.chars.peek() {
            Some('<') => self.parse_params('<', '>')?,
            _ => vec!(),
        };
        let ty = self.slurp();
        Ok(Impl {
            ty_params,
            ty,
        })
    }

    fn parse_fn_decl(&mut self) -> Result<Method> {
        let name = self.parse_method_name();
        self.skip_whitespace();
        let ty_params = match self.chars.peek() {
            Some('<') => self.parse_params('<', '>')?,
            _ => vec!(),
        };
        let params = self.parse_params('(', ')')?;
        let where_clause = self.slurp();
        Ok(Method {
            name,
            ty_params,
            params,
            where_clause,
        })
    }

    fn parse_method_name(&mut self) -> String {
        self.skip_whitespace();
        let mut res = String::new();
        while let Some(&c) = self.chars.peek() {
            match c {
                '(' | ' ' | '<' => break,
                _ => res.push(c),
            }
            self.chars.next();
        }
        res
    }

    fn skip_whitespace(&mut self) {
        while let Some(&' ') = self.chars.peek() {
            self.chars.next();
        }
    }

    fn parse_param(&mut self, terminator: char) -> Result<(String, Option<String>)> {
        let mut name = String::new();
        let mut ty = None;
        let mut has_type = true;
        let mut depth = 0;
        while let Some(&c) = self.chars.peek() {
            match c {
                '(' | '<' | '[' => depth += 1,
                c if (c == terminator || c == ',') && depth == 0 => {
                    has_type = false;
                    break;
                },
                ':' if depth == 0 => break,
                ')' | '>' | ']' => depth -= 1,
                _ => { },
            }
            self.chars.next();
            name.push(c);
        }
        if depth != 0 {
            return Err(anyhow!("unterminated parameter"));
        }
        trim_whitespace(&mut name);
        if has_type {
            assert_eq!(self.chars.next(), Some(':'));
            self.skip_whitespace();
            let mut ty_ = String::new();
            while let Some(&c) = self.chars.peek() {
                match c {
                    '(' | '<' | '[' => depth += 1,
                    c if (c == terminator || c == ',') && depth == 0 => break,
                    ')' | '>' | ']' => depth -= 1,
                    _ => { },
                }
                self.chars.next();
                ty_.push(c);
            }
            if depth != 0 {
                return Err(anyhow!("unterminated parameter"));
            }
            trim_whitespace(&mut ty_);
            ty = Some(ty_);
        }
        Ok((name, ty))
    }
}

fn trim_whitespace(s: &mut String) {
    loop {
        match s.pop() {
            Some(' ') => { },
            Some(c) => {
                s.push(c);
                return;
            }
            _ => return,
        }
    }
}
