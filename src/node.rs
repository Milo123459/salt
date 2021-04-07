use crate::task;
use chrono::{DateTime, Datelike, Local};
use colored::*;

pub struct Node {
    pub name: String,
	pub tasks: Vec<task::Task>,
	pub created_at: DateTime<Local>,
}

pub fn display_node(node: Node) -> String {
	let date = format!(
        "{}{}{:02}{}{:02}{}{:02}{}",
        "[".yellow().to_string(),
        "Created at: ".bold().to_string(),
        node.created_at.day().to_string().blue().to_string(),
        "/".yellow().to_string(),
        node.created_at.month().to_string().green().to_string(),
        "/".yellow().to_string(),
        node.created_at.year().to_string().red().to_string(),
        "]".yellow().to_string()
    );
    let name = node.name.green().to_string();
    let tasks_size = format!("{}{}{}", "[".yellow().to_string(), node.tasks.len().to_string().blue().to_string(), "]".yellow().to_string());
	format!("{} {} {}", name, date, tasks_size)
}
