use structopt::StructOpt;
use serde::{Deserialize, Serialize};

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
	pub(crate) checked: Option<Option<bool>>,
}

// for the usage of --checked
impl Arguments {
	pub fn checked(&self) -> bool {
		match self.checked {
			None => false,
			Some(None) => true,
			Some(Some(a)) => a,
		}
	}
}
