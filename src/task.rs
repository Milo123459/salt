use colored::*;
use serde::{Serialize, Deserialize};

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct Task {
	pub checked: bool,
	pub task: String,
}

pub fn display_task(task: &Task) -> String {
	let mut status = "[]".yellow().to_string();
	if task.checked == true {
		status = format!(
			"{}{}{}",
			"[".yellow().to_string(),
			"x".blue().to_string(),
			"]".yellow().to_string()
		);
	};
	format!("{} {}", status, task.task)
}
