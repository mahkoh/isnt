use anyhow::Result;
use std::path::{Path, PathBuf};
use std::fs;
use crate::parser::Spec;

pub fn collect(root: &Path) -> Result<Mod> {
    walk_inside_mod(root)
}

pub struct Mod {
    pub mods: Vec<(String, Mod)>,
    pub extensions: Vec<(String, Spec)>,
}

fn walk_inside_mod(dir: &Path) -> Result<Mod> {
    let mut mods = vec!();
    let mut extensions = vec!();
    for file in fs::read_dir(dir)? {
        let file = file?;
        let file_name: PathBuf = file.file_name().into();
        let file_stem = file_name.file_stem().unwrap().to_str().unwrap().to_owned();
        let file_path = dir.join(&file_name);
        let ty = file.file_type()?;
        if ty.is_file() {
            extensions.push((file_stem, crate::parser::parse(&file_path)?));
        } else if ty.is_dir() {
            mods.push((file_stem, walk_inside_mod(&file_path)?));
        }
    }
    mods.sort_by(|a, b| a.0.cmp(&b.0));
    extensions.sort_by(|a, b| a.0.cmp(&b.0));
    Ok(Mod {
        mods,
        extensions,
    })
}
