use serde::{Deserialize, Serialize};
use structopt::StructOpt;

#[derive(Serialize, Deserialize, Debug, StructOpt, PartialEq, Clone)]
pub struct Arguments {
	/// type of action. run the `action` / `actions` action to see available actions.
	pub action: String,

	/// arguments to action
	pub arguments: Vec<String>,

	/// todo node name
	#[structopt(long, short, default_value = "Default")]
	pub node: String,

	/// display done tasks (only applies on certain commands)
	#[structopt(long, short)]
	pub checked: bool,
}
