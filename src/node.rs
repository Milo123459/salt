use crate::task;
use colored::*;
use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct Node {
	pub name: String,
	pub tasks: Vec<task::Task>,
}

pub fn display_node(node: Node, show_hidden: bool) -> String {
	let name = node.name.truecolor(246, 105, 194).bold().to_string();
	let tasks_size = format!(
		"{}{}{}",
		"[".yellow().to_string(),
		node.tasks
			.iter()
			.filter(|task| if show_hidden == true {
				true
			} else {
				task.checked == false
			})
			.collect::<Vec<_>>()
			.len()
			.to_string()
			.blue()
			.to_string(),
		"]".yellow().to_string()
	);
	let tasks = node
		.tasks
		.iter()
		.filter(|task| {
			if show_hidden == true {
				true
			} else {
				task.checked == false
			}
		})
		.map(|task| task::display_task(task))
		.collect::<Vec<_>>()
		.join("\n  └─>");
	format!(
		"{} {}\n {}\n  └─>{}",
		name,
		tasks_size,
		"Tasks".bold().truecolor(113, 247, 159).to_string(),
		tasks
	)
}
