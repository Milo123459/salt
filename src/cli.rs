use crate::args;
use crate::config;
use crate::match_patterns;
use crate::node;
use colored::*;
use std::io::Error;

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
	node: String,
) -> anyhow::Result<()> {
	if args.arguments.first().is_some() {
		match_patterns! { &*args.arguments.first().unwrap().to_lowercase(), patterns,
			"help" => action(patterns)?,
			"list" => {
				let nodes = config.nodes.into_iter().map(|x| x.name).collect::<Vec<_>>();
				println!("Available nodes:\n{}", nodes.join(", ").underline().bold())
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

pub fn match_cmds(args: args::Arguments, config: config::SaltFile) -> anyhow::Result<()> {
	let cmd = &args.action;
	let node = args.clone().node;
	let checked = args.clone().checked();
	match_patterns! { &*cmd.to_lowercase(), patterns,
		"action" => action(patterns)?,
		"actions" => action(patterns)?,
		"node" => node_sub_command(config, args, node)?,
		_ => return Err(anyhow::Error::new(Error::new(
			std::io::ErrorKind::InvalidInput,
			"Invalid action. Try the command `action`",
		)))
	};
	Ok(())
}
