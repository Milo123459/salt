#![allow(dead_code)]
use colored::*;
use serde::{Deserialize, Serialize};
use std::io::Error;

use crate::node::Node;

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct Task {
	pub checked: bool,
	pub task: String,
	pub id: i32,
}

pub fn get_task(node: Node, id: i32) -> anyhow::Result<Task> {
	let task = node.tasks.iter().find(|task| task.id == id);
	if let Some(t) = task {
		Ok(t.clone())
	} else {
		Err(anyhow::Error::new(Error::new(
			std::io::ErrorKind::InvalidInput,
			format!("No task with id `{}` was found in node `{}`", id, node.name),
		)))
	}
}

pub fn display_task(task: &Task) -> String {
	let mut status = "[]".yellow().to_string();
	let id = format!(
		"{}{}{}",
		"(".blue().to_string(),
		task.id.to_string().red().to_string(),
		")".blue().to_string()
	);
	if task.checked {
		status = format!(
			"{}{}{}",
			"[".yellow().to_string(),
			"x".blue().to_string(),
			"]".yellow().to_string()
		);
	};
	format!("{} {} {} ", status, task.task, id)
}
