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

pub fn match_cmds(args: args::Arguments, config: config::SaltFile) -> anyhow::Result<()> {
	let cmd = &args.action;
	match_patterns! { &*cmd.to_lowercase(), patterns,
		"action" => action(patterns)?,
		"actions" => action(patterns)?,
		_ => return Err(anyhow::Error::new(Error::new(
			std::io::ErrorKind::InvalidInput,
			"Invalid action. Try the command `action`",
		)))
	};
	Ok(())
}
