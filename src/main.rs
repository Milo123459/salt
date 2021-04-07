mod node;
mod task;
use chrono::Local;
fn main() -> () {
	println!(
		"{}",
		node::display_node(node::Node {
            name: "Pepper!".to_string(),
			created_at: Local::now(),
			tasks: vec![task::Task {
				checked: false,
				task: "he".to_string()
			}]
		})
	);
}
