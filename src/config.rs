use crate::node;
use serde::{Deserialize, Serialize};
#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct SaltFile {
	pub nodes: Vec<node::Node>,
}
