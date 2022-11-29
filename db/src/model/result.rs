use gremlin::{GValue, List};

use crate::IxResult;

#[derive(Clone, Debug)]
pub struct ExecutionResult {
	pub edges: IxResult,
	pub vertices: IxResult,
	pub new_vertex: IxResult,
	pub new_edge: IxResult,
	pub other: IxResult,
}

impl Default for ExecutionResult {
	fn default() -> Self {
		let default_list = IxResult::new("", GValue::List(List::default()));
		Self {
			edges: default_list.clone(),
			vertices: default_list.clone(),
			new_vertex: Default::default(),
			new_edge: Default::default(),
			other: default_list,
		}
	}
}

impl ExecutionResult {
	pub fn get_from_source(&self, source: &str) -> IxResult {
		match source {
			"E" => self.edges.clone(),
			"V" => self.vertices.clone(),
			"addV" => self.new_vertex.clone(),
			"addE" => self.new_edge.clone(),
			_ => unimplemented!(),
		}
	}
}
