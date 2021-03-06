use crate::args;
use crate::config;
use crate::get_and_parse;
use crate::match_patterns;
use crate::node;
use crate::task;
use colored::*;
use std::io::Error;
use std::str::FromStr;

pub fn action(input: Vec<&str>) -> anyhow::Result<()> {
	// this will sanitize the vec in a sense
	// the input has \" \" around the value we want so we remove it
	// we also filter out _ from the vec
	let actions = input
		.into_iter()
		.filter_map(|x| x.strip_prefix('"')?.strip_suffix('"'))
		.collect::<Vec<_>>();
	// log a nice message displaying all the actions
	println!(
		"Actions available:\n{}",
		actions.join(", ").underline().bold()
	);
	Ok(())
}

pub fn node_sub_command(
	config: config::SaltFile,
	args: args::Arguments,
	supplied_node: String,
	checked: bool,
) -> anyhow::Result<()> {
	if args.arguments.first().is_some() {
		match_patterns! { &*args.arguments.first().unwrap().to_lowercase(), patterns,
			"help" => action(patterns)?,
			"new" => {
				if config.nodes.iter().any(|i| i.name.to_lowercase() == supplied_node.to_lowercase()) {
					return Err(anyhow::Error::new(Error::new(
						std::io::ErrorKind::InvalidInput,
						format!("You can't create a node that already exists (supplied node was `{}` - please use --node <name> to specify the node name)", supplied_node)
					)))
				};
				let mut new_config = config;
				new_config.nodes.push(node::Node {
					name: supplied_node.to_owned(),
					tasks: vec![],
					next_id: 1
				});
				get_and_parse::write(new_config)?;
				println!("{} `{}`", "Created new node".bold().to_string(), &supplied_node.red().to_string())
			},
			"list" => {
				let nodes = config.nodes.into_iter().map(|x| x.name).collect::<Vec<_>>();
				println!("Available nodes:\n{}", nodes.join(", ").underline().bold())
			},
			"tasks" => {
				let possible_node = node::get_node(&supplied_node)?;
				println!("{}", node::display_node(possible_node, checked));
			},
			"convert" => {
				if args.arguments.get(1).is_some() {

				match_patterns! { &*args.arguments.get(1).unwrap().to_lowercase(), patterns_for_convert,
					"help" => action(patterns_for_convert)?,
					"md" => {
						let possible_node = node::get_node(&supplied_node)?;
						let mut markdown: Vec<String> = vec![];
						markdown.push(format!("**{}**\n", possible_node.name));
						for task in possible_node.tasks {
							markdown.push(format!("- [{}] **{}** ({})", if task.checked { "x" } else { " " }, task.task, task.id));
						}
						println!("{}", markdown.join("\n"))
					},
					_ => return Err(anyhow::Error::new(Error::new(
						std::io::ErrorKind::InvalidInput,
						"Invalid sub-action. Try the command `convert help`",
					)))
				}
			} else {
				println!("Invalid sub-action. Try the command `convert help`")
			}
			},
			_ => return Err(anyhow::Error::new(Error::new(
				std::io::ErrorKind::InvalidInput,
				"Invalid sub-action. Try the command `node help`",
			)))
		}
	} else {
		println!("Try `node help`")
	}
	Ok(())
}

pub fn add(
	config: config::SaltFile,
	args: args::Arguments,
	supplied_node: String,
) -> anyhow::Result<()> {
	if args.arguments.first().is_some() {
		let task = args.arguments.join(" ");
		let other_task = &task.clone();
		let mut possible_node = node::get_node(&supplied_node)?;
		let mut new_config = config;
		possible_node.tasks.push(task::Task {
			id: possible_node.next_id,
			task,
			checked: false,
		});

		possible_node.next_id = &possible_node.next_id + 1;
		for node_in_config in &mut new_config.nodes {
			if *node_in_config.name.to_lowercase() == supplied_node.to_lowercase() {
				*node_in_config = (&possible_node).to_owned();
			}
		}

		get_and_parse::write(new_config)?;

		println!(
			"Created task `{}` in node `{}`",
			other_task,
			supplied_node.red()
		);
	} else {
		println!("Please specify a task description!")
	}
	Ok(())
}

pub fn check(
	config: config::SaltFile,
	args: args::Arguments,
	supplied_node: String,
) -> anyhow::Result<()> {
	if args.arguments.first().is_some() {
		let mut possible_node = node::get_node(&supplied_node)?;
		let mut new_config = config;
		let mut status = false;
		for task_node in &mut possible_node.tasks {
			if task_node.id.to_string() == *args.arguments.first().unwrap() {
				status = !task_node.checked;
				task_node.checked = status;
				*task_node = task_node.to_owned();
			}
		}
		for node_in_config in &mut new_config.nodes {
			if *node_in_config.name.to_lowercase() == supplied_node.to_lowercase() {
				*node_in_config = (&possible_node).to_owned();
			}
		}
		get_and_parse::write(new_config)?;
		println!(
			"Task with ID of `{}` was {}",
			args.arguments.first().unwrap().to_string().red(),
			if status { "checked" } else { "unchecked" }
		)
	} else {
		println!("Please specify the task ID")
	}
	Ok(())
}

pub fn tasks(checked: bool, supplied_node: String) -> anyhow::Result<()> {
	let possible_node = node::get_node(&supplied_node)?;
	println!("{}", node::display_node(possible_node, checked));
	Ok(())
}

pub fn edit(
	config: config::SaltFile,
	args: args::Arguments,
	supplied_node: String,
) -> anyhow::Result<()> {
	if args.arguments.first().is_some() && args.arguments.get(1).is_some() {
		let joined_args = args.arguments.join(" ");
		let parsed_arguments = joined_args.split_once(" ").unwrap();
		let id = FromStr::from_str(parsed_arguments.0)?;
		let new_description = parsed_arguments.1;
		let mut possible_node = node::get_node(&supplied_node)?;
		let mut new_config = config;
		if task::task_exists(possible_node.clone(), id).unwrap() {
			if let Some(task) = possible_node.tasks.iter_mut().find(|t| t.id == id) {
				task.task = String::from(new_description)
			}

			for node_in_config in &mut new_config.nodes {
				if *node_in_config.name.to_lowercase() == supplied_node.to_lowercase() {
					*node_in_config = (&possible_node).to_owned();
				}
			}

			get_and_parse::write(new_config)?;

			println!(
				"Edited task with ID `{}` in node `{}`",
				id,
				supplied_node.red()
			);
		} else {
			println!("Couldn't find task.")
		}
	} else {
		println!("Please specify a task id, and the new description of the task.")
	}
	Ok(())
}

pub fn all(config: config::SaltFile, checked: bool) -> anyhow::Result<()> {
	for supplied_node in config.nodes {
		println!("{}", node::display_node(supplied_node, checked));
	}
	Ok(())
}

pub fn match_cmds(args: args::Arguments, config: config::SaltFile) -> anyhow::Result<()> {
	let cmd = &args.action;
	let supplied_node = args.clone().node;
	let checked = args.clone().checked();
	match_patterns! { &*cmd.to_lowercase(), patterns,
		"action" => action(patterns)?,
		"actions" => action(patterns)?,
		"node" => node_sub_command(config, args, supplied_node, checked)?,
		"add" => add(config, args, supplied_node)?,
		"check" => check(config, args, supplied_node)?,
		"uncheck" => check(config, args, supplied_node)?,
		"done" => check(config, args, supplied_node)?,
		"tasks" => tasks(checked, supplied_node)?,
		"all" => all(config, checked)?,
		"edit" => edit(config, args, supplied_node)?,
		_ => return Err(anyhow::Error::new(Error::new(
			std::io::ErrorKind::InvalidInput,
			"Invalid action. Try the command `action`",
		)))
	};
	Ok(())
}
