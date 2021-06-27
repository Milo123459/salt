#![allow(dead_code)]
use colored::*;
use serde::{Deserialize, Serialize};

use crate::node::Node;

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct Task {
	pub checked: bool,
	pub task: String,
	pub id: i32,
}

pub fn task_exists(node: Node, id: i32) -> anyhow::Result<bool> {
	let task = node.tasks.iter().find(|task| task.id == id);
	if task.is_some() {
		Ok(true)
	} else {
		Ok(false)
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
