mod args;
mod cli;
mod config;
mod get_and_parse;
mod macros;
mod node;
mod run;
mod task;
pub use anyhow::Context;
pub use structopt::StructOpt;

fn main() -> anyhow::Result<()> {
	run::run(args::Arguments::from_args())?;
	Ok(())
}
