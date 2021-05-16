use crate::config::SaltFile;
use crate::node::Node;
use anyhow::Context;
use dirs_next::home_dir;
use std::fs;
use std::io::prelude::*;
use std::path::{Path, PathBuf};

pub fn parse() -> anyhow::Result<SaltFile> {
	let dir = folder_path();
	fs::create_dir_all(dir)?;
	let file = file_path();
	let does_exist = Path::new(file.as_path()).exists();
	if does_exist {
		read(file)
	} else {
		let config = SaltFile {
			nodes: vec![Node {
				name: "Default".to_owned(),
				tasks: vec![],
				next_id: 1,
			}],
		};
		write(config)
	}
}

pub fn folder_path() -> PathBuf {
	let mut dir = PathBuf::new();
	dir.push(home_dir().unwrap());
	dir.push(".salt");
	dir
}

pub fn file_path() -> PathBuf {
	let mut file = folder_path();
	file.push("salt.json");
	file
}

pub fn write(config: SaltFile) -> anyhow::Result<SaltFile> {
	let dir = folder_path();
	fs::create_dir_all(dir)?;
	let file = file_path();
	let json = serde_json::to_string(&config).unwrap();
	let mut physical_file = fs::File::create(file.as_path())?;
	physical_file.write_all(json.as_bytes())?;
	Ok(config)
}

pub fn read(path: PathBuf) -> anyhow::Result<SaltFile> {
	let file = fs::File::open(path.as_path());
	match serde_json::from_reader(file.unwrap()) {
		Ok(json) => Ok(json),
		Err(err) => Err(anyhow::Error::new(err))
			.with_context(|| "Error parsing saltfile. Try deleting the file."),
	}
}
