use crate::config::SaltFile;
use crate::node::Node;
use anyhow::Context;
use dirs_next::home_dir;
use serde_json;
use std::fs;
use std::io::prelude::*;
use std::path::{Path, PathBuf};
//use crate::config::SaltFile;

pub fn parse() -> anyhow::Result<SaltFile> {
	let mut dir = PathBuf::new();
	dir.push(home_dir().unwrap());
	dir.push(".salt");
	fs::create_dir_all(dir)?;
	let mut file = PathBuf::new();
	file.push(home_dir().unwrap());
	file.push(".salt");
	file.push("salt.json");
	let does_exist = Path::new(file.as_path()).exists();
	if does_exist {
		let file = fs::File::open(file.as_path());
		match serde_json::from_reader(file.unwrap()) {
			Ok(json) => Ok(json),
			Err(err) => Err(anyhow::Error::new(err))
				.with_context(|| "Error parsing saltfile. Try deleting the file."),
		}
	} else {
		let config = SaltFile {
			nodes: vec![Node {
				name: "Default".to_owned(),
				tasks: vec![],
			}],
		};
		let json = serde_json::to_string(&config.clone()).unwrap();
		let mut physical_file = fs::File::create(file.as_path())?;
		physical_file.write_all(json.as_bytes())?;
		Ok(config)
	}
}
