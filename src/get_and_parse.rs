use dirs::home_dir;
use std::fs;
use std::path::{Path, PathBuf};
use anyhow;
//use crate::config::SaltFile;

pub fn parse() -> anyhow::Result<()> {
    let mut dir = PathBuf::new();
    dir.push(home_dir().unwrap());
    dir.push(".salt");
    fs::create_dir_all(dir)?;
    let mut file = PathBuf::new();
    file.push(home_dir().unwrap());
    file.push(".salt");
    file.push("salt.json");
    let does_exist = Path::new(file.as_path()).exists();
    println!("{}", does_exist);
    Ok(())
}