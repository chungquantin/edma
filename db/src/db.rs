use std::collections::LinkedList;

use gremlin::{
	process::traversal::{GraphTraversal, GraphTraversalSource, Instruction, MockTerminator},
	GValue, Vertex,
};

use crate::{
	err::Error, storage::DBRef, IxResult, IxValue, SimpleTransaction, VertexRepository,
	VertexResult,
};

type TraversalSource = GraphTraversalSource<MockTerminator>;
pub struct Database<'a> {
	traversal: &'a TraversalSource,
	pub vertex: VertexRepository<'a>,
	steps: LinkedList<IxResult<'a>>,
	// pub vertex_property: VertexPropertyRepository<'a>,
	// pub edge: EdgeRepository<'a>,
	// pub edge_property: EdgePropertyRepository<'a>,
	// pub label: LabelRepository<'a>,
}

impl<'a> IxResult<'a> {
	pub fn new(operator: &'a str, value: IxValue) -> Self {
		IxResult {
			operator,
			value,
			source_operator: "".to_string(),
		}
	}

	pub fn add_source(&mut self, operator: String) -> Result<(), Error> {
		self.source_operator = operator;
		Ok(())
	}
}

fn is_source_step(step: &Instruction) -> bool {
	let s = step.operator().as_str();
	s == "V" || s == "E" || s == "addV" || s == "addE"
}

impl<'a> Database<'a> {
	pub fn new(ds_ref: DBRef<'a>, traversal: &'a TraversalSource) -> Self {
		Database {
			traversal,
			vertex: VertexRepository::new(ds_ref),
			steps: LinkedList::<IxResult>::new(),
			// vertex_property: VertexPropertyRepository::new(ds_ref),
			// edge: EdgeRepository::new(ds_ref),
			// edge_property: EdgePropertyRepository::new(ds_ref),
			// label: LabelRepository::new(ds_ref),
		}
	}

	pub fn traverse(&self) -> &TraversalSource {
		self.traversal
	}

	async fn process_source_step(&mut self, step: &Instruction) {
		let args = step.args();
		let operator = step.operator().as_str();
		let mut step_result = match operator {
			"V" => self.v(args).await,
			"addV" => self.add_v(args).await,
			"E" => self.e(args),
			"addE" => self.add_e(args),
			_ => unimplemented!(),
		};

		let source = step.operator().to_string();
		step_result.add_source(source).unwrap();

		self.steps.push_back(step_result);
	}

	pub async fn execute(
		&mut self,
		traversal: GraphTraversal<Vertex, Vertex, MockTerminator>,
	) -> Result<IxResult, Error> {
		let bytecode = traversal.bytecode();
		println!("Bytecode: {:?}", bytecode);

		for step in bytecode.steps() {
			if is_source_step(step) {
				self.process_source_step(step).await;
			}
		}

		let last_result = self.steps.back().unwrap().clone();
		Ok(last_result)
	}

	/// The V()-step is meant to read vertices from the graph and is usually
	/// used to start a GraphTraversal, but can also be used mid-traversal.
	async fn v(&mut self, ids: &Vec<GValue>) -> IxResult<'a> {
		let tx = &mut self.vertex.mut_tx();
		let result = self.vertex.v(tx, ids).await.unwrap();
		tx.commit().await.unwrap();

		IxResult::new("V", IxValue::VertexSeq(result))
	}

	fn e(&mut self, ids: &Vec<GValue>) -> IxResult<'a> {
		println!("Edge {:?}", ids);

		IxResult::new("E", IxValue::None)
	}

	/// The addV()-step is used to add vertices to the graph (map/sideEffect).
	/// For every incoming object, a vertex is created. Moreover, GraphTraversalSource maintains an addV() method.
	/// [Documentation](https://tinkerpop.apache.org/docs/current/reference/#addvertex-step)
	async fn add_v(&mut self, labels: &Vec<GValue>) -> IxResult<'a> {
		let tx = &mut self.vertex.mut_tx();
		let source = self.steps.back().unwrap();
		let mut result = vec![];
		let vertices = source.value.get::<Vec<VertexResult>>().unwrap();

		if vertices.is_empty() {
			// If there is no vertices defined, initialized with default option
			let new_v = &mut Vertex::default();
			let vertex = self.vertex.add_v(tx, new_v, labels, false).await.unwrap();
			result.push(vertex);
		} else {
			// If there are vertices found, filter out the initialized vertex
			for cur in vertices {
				let v = &mut cur.v();
				let scoped_result = self.vertex.add_v(tx, v, labels, true).await.unwrap();
				result.push(scoped_result.clone());
			}
		}

		tx.commit().await.unwrap();
		IxResult::new("addV", IxValue::VertexSeq(result))
	}

	fn add_e(&mut self, labels: &Vec<GValue>) -> IxResult<'a> {
		println!("Add edge {:?}", labels);

		IxResult::new("addE", IxValue::None)
	}
}

#[cfg(test)]
mod test {
	use crate::{storage::Datastore, util::generate_path, Database};
	use gremlin::process::traversal::{GraphTraversalSource, MockTerminator};

	#[tokio::test]
	async fn database_test() {
		let path = &generate_path(None);
		let datastore = Datastore::new(path);
		let g = GraphTraversalSource::<MockTerminator>::empty();
		let mut db = Database::new(datastore.borrow(), &g);

		let traversal = db.traverse().v(1).add_v("person").v(2).add_v("hero").v(());
		let result = db.execute(traversal).await.unwrap();

		println!("Result: {:?}", result);
		unimplemented!();
	}
}
