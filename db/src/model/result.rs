use gremlin::{GValue, List};

use crate::IxResult;

#[derive(Clone, Debug)]
pub struct ExecutionResult {
	pub edges: IxResult,
	pub vertices: IxResult,
	pub new_vertices: IxResult,
	pub new_edges: IxResult,
	pub other: IxResult,
}

impl Default for ExecutionResult {
	fn default() -> Self {
		let default_list = IxResult::new("", GValue::List(List::default()));
		Self {
			edges: default_list.clone(),
			vertices: default_list.clone(),
			new_vertices: default_list.clone(),
			new_edges: default_list.clone(),
			other: default_list,
		}
	}
}

impl ExecutionResult {
	pub fn get_from_source(&self, source: &str) -> IxResult {
		match source {
			"E" => self.edges.clone(),
			"V" => self.vertices.clone(),
			"addV" => self.new_vertices.clone(),
			"addE" => self.new_edges.clone(),
			_ => unimplemented!(),
		}
	}
}
