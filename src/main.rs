mod args;
mod cli;
mod config;
mod get_and_parse;
mod macros;
mod node;
mod task;
mod run;
pub use anyhow::Context;
pub use structopt::StructOpt;


fn main() -> anyhow::Result<()> {
    run::run(args::Arguments::from_args_safe()?).expect("Couldn't run.");
    Ok(())
}
