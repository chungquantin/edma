use crate::err::Error;

use super::StepExecutor;
use solomon_gremlin::{process::traversal::TerminatorToken, FromGValue, GValue, List, Vertex};

pub struct StepCollector<'a, T: FromGValue + Clone> {
	executor: StepExecutor<'a, T>,
}

impl<'a, T: FromGValue + Clone> StepCollector<'a, T> {
	pub fn new(executor: StepExecutor<'a, T>) -> Self {
		Self {
			executor,
		}
	}

	fn collect_vertex_list(&self) -> List {
		let mut list = self.executor.source_value::<List>("V").unwrap();
		let new_vertices = self.executor.source_value::<List>("addV").unwrap();
		list.append(&mut new_vertices.core());
		list
	}

	fn collect_vertex(&self) -> GValue {
		GValue::List(self.collect_vertex_list())
	}

	fn collect_vertex_properties(&self) -> GValue {
		let list = self.collect_vertex_list();
		let mut result: Vec<GValue> = vec![];

		for item in list.iter() {
			let vertex = item.get::<Vertex>().unwrap();
			let properties = vertex.properties();
			for (_, property) in properties.iter() {
				for item in property {
					result.push(GValue::VertexProperty(item.clone()));
				}
			}
		}

		GValue::List(List::new(result))
	}

	fn collect_count(&self) -> GValue {
		let value = &self.executor.result.other.value;
		let terminator = value.get::<TerminatorToken>().unwrap();
		let result = self.collect(&terminator.clone()).unwrap();
		let list = result.get::<List>().unwrap();
		GValue::Int64(list.len() as i64)
	}

	pub fn collect(&self, terminator: &TerminatorToken) -> Result<GValue, Error>
	where
		T: FromGValue,
	{
		match terminator {
			TerminatorToken::Vertex => Ok(self.collect_vertex()),
			TerminatorToken::VertexProperty => Ok(self.collect_vertex_properties()),
			TerminatorToken::Int64 => Ok(self.collect_count()),
			_ => unimplemented!(),
		}
	}
}
