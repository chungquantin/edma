use std::collections::LinkedList;

use gremlin::{
	process::traversal::{GraphTraversal, GraphTraversalSource, Instruction, MockTerminator},
	GValue, Vertex,
};

use crate::{
	err::Error, storage::DatastoreRef, IxResult, IxValue, PropertyRepository, SimpleTransaction,
	VertexRepository, VertexResult,
};

type TraversalSource = GraphTraversalSource<MockTerminator>;
pub struct Database<'a> {
	traversal: &'a TraversalSource,
	steps: LinkedList<IxResult<'a>>,
	pub v: VertexRepository<'a>,
	pub p: PropertyRepository<'a>,
	// pub vertex_property: VertexPropertyRepository<'a>,
	// pub edge: EdgeRepository<'a>,
}

fn contain_source(step: &Instruction) -> bool {
	let s = step.operator().as_str();
	s == "V" || s == "E" || s == "addV" || s == "addE"
}

impl<'a> Database<'a> {
	pub fn new(ds_ref: DatastoreRef<'a>, traversal: &'a TraversalSource) -> Self {
		let mut steps = LinkedList::<IxResult>::new();
		steps.push_back(IxResult::empty());

		Database {
			traversal,
			steps,
			v: VertexRepository::new(ds_ref),
			p: PropertyRepository::new(ds_ref),
			// vertex_property: VertexPropertyRepository::new(ds_ref),
			// edge: EdgeRepository::new(ds_ref),
		}
	}

	pub fn traverse(&self) -> &TraversalSource {
		self.traversal
	}

	async fn process_streaming_step(&mut self, step: &Instruction) {
		let args = step.args();
		let operator = step.operator().as_str();
		let mut step_result = match operator {
			"V" => self.v(args).await,
			"E" => self.e(args).await,
			"addV" => self.add_v(args).await,
			"addE" => self.add_e(args).await,
			_ => unimplemented!(),
		};

		let source = step.operator().to_string();
		step_result.set_source(source).unwrap();

		self.steps.push_back(step_result);
	}

	async fn process_step(&mut self, step: &Instruction) {
		let args = step.args();
		let operator = step.operator().as_str();
		let mut step_result = match operator {
			"property" => self.property(args).await,
			_ => unimplemented!(),
		};

		let stream = self.steps.back().unwrap();
		let source = &stream.source;
		step_result.set_source(source.to_string()).unwrap();
		self.steps.push_back(step_result);
	}

	pub async fn execute(
		&mut self,
		traversal: GraphTraversal<Vertex, Vertex, MockTerminator>,
	) -> Result<IxResult, Error> {
		let bytecode = traversal.bytecode();
		println!("Bytecode: {:?}", bytecode);
		for step in bytecode.steps() {
			match contain_source(step) {
				true => self.process_streaming_step(step).await,
				false => self.process_step(step).await,
			}
		}
		Ok(self.top_step().clone())
	}

	/// The V()-step is meant to read vertices from the graph and is usually
	/// used to start a GraphTraversal, but can also be used mid-traversal.
	async fn v(&mut self, args: &Vec<GValue>) -> IxResult<'a> {
		let tx = &mut self.v.mut_tx();
		let result = self.v.v(tx, args).await.unwrap();
		tx.commit().await.unwrap();

		IxResult::new("V", IxValue::VertexSeq(result))
	}

	async fn e(&mut self, ids: &Vec<GValue>) -> IxResult<'a> {
		println!("Edge {:?}", ids);

		IxResult::new("E", IxValue::Null)
	}

	/// The addV()-step is used to add vertices to the graph (map/sideEffect).
	/// For every incoming object, a vertex is created. Moreover, GraphTraversalSource maintains an addV() method.
	/// [Documentation](https://tinkerpop.apache.org/docs/current/reference/#addvertex-step)
	async fn add_v(&mut self, args: &Vec<GValue>) -> IxResult<'a> {
		let mut result = vec![];
		let tx = &mut self.v.mut_tx();
		let vertices = self.get_streamed_vertices();
		match vertices {
			v if v.is_empty() => result.push(self.v.new_v(tx, args).await.unwrap()),
			v => {
				for cur in v.iter() {
					let v = &mut cur.v();
					let vertex_result = self.v.add_v(tx, v, args, true).await.unwrap();
					result.push(vertex_result);
				}
			}
		}

		tx.commit().await.unwrap();
		IxResult::new("addV", IxValue::VertexSeq(result))
	}

	async fn add_e(&mut self, labels: &Vec<GValue>) -> IxResult<'a> {
		println!("Add edge {:?}", labels);

		IxResult::new("addE", IxValue::Null)
	}

	async fn property_with_cardinality(&mut self, _args: &Vec<GValue>) -> IxResult<'a> {
		IxResult::new("property", IxValue::Null)
	}

	async fn vertex_property(&mut self, args: &Vec<GValue>) -> IxResult<'a> {
		let tx = &mut self.v.mut_tx();
		let mut result = vec![];
		let vertices = self.get_streamed_vertices();
		match vertices {
			v if v.is_empty() => result.push(self.v.new_property(tx, args).await.unwrap()),
			v => {
				for cur in v.iter() {
					let v = &mut cur.v();
					let vertex_result = self.v.property(v, tx, args, true).await.unwrap();
					result.push(vertex_result);
				}
			}
		}
		tx.commit().await.unwrap();
		IxResult::new("vertex_property", IxValue::VertexSeq(result))
	}

	/// The property()-step is used to add properties to the elements of the graph (sideEffect).
	/// Unlike addV() and addE(), property() is a full sideEffect step in that it does not return
	/// the property it created, but the element that streamed into it. Moreover, if property()
	/// follows an addV() or addE(), then it is "folded" into the previous step to enable vertex
	/// and edge creation with all its properties in one creation operation.
	/// [Documentation](https://tinkerpop.apache.org/docs/current/reference/#property-step)
	async fn property(&mut self, args: &Vec<GValue>) -> IxResult<'a> {
		println!("Property {:?}", args);

		match args.first().unwrap().is_cardinality() {
			true => self.property_with_cardinality(args).await,
			false => {
				let stream = self.top_step();
				println!("Steps: {:?}", self.steps);
				match stream.source.as_str() {
					"V" | "addV" => self.vertex_property(args).await,
					_ => unimplemented!(),
				}
			}
		}
	}

	fn get_streamed_vertices(&mut self) -> Vec<VertexResult> {
		let stream = self.top_step();
		let default = Vec::default();

		let vertices = stream.value.get::<Vec<VertexResult>>().unwrap_or(&default);
		vertices.to_vec()
	}

	fn top_step(&self) -> &IxResult<'_> {
		self.steps.back().unwrap()
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

		let command = db.traverse().add_v("person").property_many(vec![
			("birthday".to_string(), "1/11/2001"),
			("github".to_string(), "chungquantin"),
			("name".to_string(), "Tin Chung"),
		]);
		// db.traverse().add_v("person");

		let result = db.execute(command).await.unwrap();

		println!("Result: {:?}", result);
		// unimplemented!();
	}
}
