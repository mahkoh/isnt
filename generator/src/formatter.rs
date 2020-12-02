use std::fs::{File, OpenOptions};
use anyhow::Result;
use std::path::{Path};
use std::io::{Write, BufWriter};
use crate::collector::Mod;
use std::borrow::Cow;
use crate::parser::{Spec, Method};
use std::fmt::{Display, Formatter, Write as FmtWrite};
use std::fmt;
use isnt::std_1::primitive::IsntSliceExt;

pub fn format(mod_: &Mod, out_dir: &Path) -> Result<()> {
    std::fs::create_dir_all(out_dir)?;
    let out_file = out_dir.join("lib_generated.rs");
    let file = open(&out_file)?;
    format_inside_mod(mod_, out_dir, file)
}

fn open(path: &Path) -> Result<BufWriter<File>> {
    Ok(BufWriter::new(OpenOptions::new().create(true).write(true).truncate(true).open(path)?))
}

fn format_inside_mod(mod_: &Mod, out_dir: &Path, mut file: BufWriter<File>) -> Result<()> {
    let mut dirty = true;
    writeln!(file, "// This file was generated")?;
    for &(ref name, ref mod_) in &mod_.mods {
        if dirty {
            writeln!(file)?;
            dirty = false;
        }
        writeln!(file, "pub mod {};", name)?;
        format_outside_mod(name, mod_, out_dir.into())?;
    }
    for &(ref name, ref ext) in &mod_.extensions {
        if dirty {
            writeln!(file)?;
        }
        create_trait(ext, name, &mut file)?;
        dirty = true;
    }
    Ok(())
}

fn format_outside_mod(name: &str, mod_: &Mod, mut containing_dir: Cow<Path>) -> Result<()> {
    let path = if !mod_.mods.is_empty() {
        containing_dir = Cow::Owned(containing_dir.join(name));
        std::fs::create_dir_all(&containing_dir)?;
        containing_dir.join("mod.rs")
    } else {
        containing_dir.join(format!("{}.rs", name))
    };
    format_inside_mod(mod_, &containing_dir, open(&path)?)?;
    Ok(())
}

fn write_ty_params(parameters: &[(String, Option<String>)], out: &mut BufWriter<File>) -> Result<()> {
    if parameters.is_empty() {
        return Ok(());
    }
    write!(out, "<")?;
    write_parameters(parameters, out)?;
    write!(out, ">")?;
    Ok(())
}

fn write_parameters(parameters: &[(String, Option<String>)], out: &mut BufWriter<File>) -> Result<()> {
    for (n, parameter) in parameters.iter().enumerate() {
        if n > 0 {
            write!(out, ", ")?;
        }
        write!(out, "{}", parameter.0)?;
        if let Some(ty) = &parameter.1 {
            write!(out, ": {}", ty)?;
        }
    }
    Ok(())
}

fn write_signature(method: &Method, out: &mut BufWriter<File>) -> Result<()> {
    let (prefix, suffix) = if method.name == "is" {
        ("is_not", "")
    } else if method.name.starts_with("is_") {
        ("is_not", &method.name["is".len()..])
    } else {
        ("not_", &method.name[..])
    };
    write!(out, "    fn {}{}", prefix, suffix)?;
    write_ty_params(&method.ty_params, out)?;
    write!(out, "(")?;
    write_parameters(&method.params, out)?;
    write!(out, ")")?;
    write!(out, " -> bool")?;
    if !method.where_clause.is_empty() {
        write!(out, " {}", method.where_clause)?;
    }
    Ok(())
}

fn write_ty_args(params: &[(String, Option<String>)], out: &mut BufWriter<File>) -> Result<()> {
    if params.is_not_empty() {
        write!(out, "<")?;
        for (pos, (name, _)) in params.iter().enumerate() {
            if pos > 0 {
                write!(out, ", ")?;
            }
            write!(out, "{}", name)?;
        }
        write!(out, ">")?;
    }
    Ok(())
}

struct ToSnakeCase<'a>(&'a str);

impl<'a> Display for ToSnakeCase<'a> {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        for (pos, c) in self.0.chars().enumerate() {
            if c.is_ascii_uppercase() {
                if pos > 0 {
                    f.write_char('_')?;
                }
                f.write_char(c.to_ascii_lowercase())?;
            } else {
                f.write_char(c)?;
            }
        }
        Ok(())
    }
}

fn write_cfg(spec: &Spec, out: &mut BufWriter<File>) -> Result<()> {
    if let Some(cfg) = &spec.cfg {
        writeln!(out, "#[cfg({})]", cfg)?;
    }
    Ok(())
}

fn extract_type_name(spec: &Spec) -> (String, String, bool) {
    if let Some(name) = &spec.target_name {
        return (name.clone(), name.clone(), false);
    }
    let target = match &spec.extends {
        Some(e) => e,
        _ => &spec.imp.ty,
    };
    let mut long = String::new();
    let mut short = String::new();
    for c in target.chars() {
        if matches!(c, '<' | ' ') {
            break;
        }
        long.push(c);
        short.push(c);
        if c == ':' {
            short.clear();
        }
    }
    assert!(!short.is_empty());
    (short, long, true)
}

fn create_trait(spec: &Spec, name: &str, out: &mut BufWriter<File>) -> Result<()> {
    let (short_target, long_target, link) = extract_type_name(spec);
    write_cfg(spec, out)?;
    write!(out, "mod {}_private {{ pub trait Sealed", ToSnakeCase(name))?;
    write_ty_params(&spec.imp.ty_params, out)?;
    writeln!(out, " {{ }} }}\n")?;
    write!(out, "/// Extension for ")?;
    if link {
        writeln!(out, "[`{}`]({})", short_target, long_target)?;
    } else {
        writeln!(out, "{}", short_target)?;
    }
    write_cfg(spec, out)?;
    write!(out, "pub trait Isnt{}Ext", name)?;
    write_ty_params(&spec.imp.ty_params, out)?;
    write!(out, ": {}_private::Sealed", ToSnakeCase(name))?;
    write_ty_args(&spec.imp.ty_params, out)?;
    if let Some(ext) = &spec.extends {
        write!(out, "+{}", ext)?;
    }
    writeln!(out, " {{")?;
    for method in &spec.methods {
        write!(out, "    /// The negation of ")?;
        if link {
            writeln!(out, "[`{}`]({}::{})", method.name, long_target, method.name)?;
        } else {
            writeln!(out, "`{}`", method.name)?;
        }
        writeln!(out, "    #[must_use]")?;
        write_signature(&method, out)?;
        writeln!(out, ";")?;
    }
    writeln!(out, "}}\n")?;
    write_cfg(spec, out)?;
    write!(out, "impl")?;
    write_ty_params(&spec.imp.ty_params, out)?;
    write!(out, " {}_private::Sealed", ToSnakeCase(name))?;
    write_ty_args(&spec.imp.ty_params, out)?;
    writeln!(out, " for {} {{ }}\n", spec.imp.ty)?;
    write_cfg(spec, out)?;
    write!(out, "impl")?;
    write_ty_params(&spec.imp.ty_params, out)?;
    write!(out, " Isnt{}Ext", name)?;
    write_ty_args(&spec.imp.ty_params, out)?;
    writeln!(out, " for {} {{", spec.imp.ty)?;
    for (n, method) in spec.methods.iter().enumerate() {
        if n > 0 {
            writeln!(out)?;
        }
        writeln!(out, "    #[inline]")?;
        write_signature(&method, out)?;
        writeln!(out, " {{")?;
        write!(out, "        !")?;
        if method.params[0].1.is_some() {
            write!(out, "{}", method.params[0].0)?;
        } else {
            write!(out, "self")?;
        }
        write!(out, ".{}", method.name)?;
        if method.ty_params.is_not_empty() {
            write!(out, "::")?;
            write_ty_args(&method.ty_params, out)?;
        }
        write!(out, "(")?;
        for (n, parameter) in method.params.iter().skip(1).enumerate() {
            if n > 0 {
                write!(out, ", ")?;
            }
            write!(out, "{}", parameter.0)?;
        }
        writeln!(out, ")")?;
        writeln!(out, "    }}")?;
    }
    writeln!(out, "}}")?;
    Ok(())
}
