mod parser;
mod formatter;
mod collector;

use anyhow::Result;

fn main() -> Result<()> {
    let root = "../tpl".as_ref();
    let mod_ = collector::collect(root)?;
    formatter::format(&mod_, "../src".as_ref())
}
