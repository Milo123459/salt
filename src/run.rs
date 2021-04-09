use crate::args;
use crate::cli;
use crate::get_and_parse;

pub fn run(args: args::Arguments) -> anyhow::Result<()> {
    let config = get_and_parse::parse().unwrap();
    cli::match_cmds(args, config).expect("Couldn't match command");
    Ok(())
}