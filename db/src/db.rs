use std::collections::{HashMap, LinkedList, VecDeque};

use gremlin::{
	process::traversal::{GraphTraversal, GraphTraversalSource, Instruction, MockTerminator},
	GValue, Vertex,
};

use crate::{
	err::Error, storage::DatastoreRef, IxResult, IxValue, PropertyRepository, SimpleTransaction,
	VertexRepository,
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

trait Dedup<T: PartialEq + Clone> {
	fn clear_duplicates(&mut self);
}

impl<T: PartialEq + Clone> Dedup<T> for Vec<T> {
	fn clear_duplicates(&mut self) {
		let mut already_seen = Vec::new();
		self.retain(|item| match already_seen.contains(item) {
			true => false,
			_ => {
				already_seen.push(item.clone());
				true
			}
		})
	}
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

	pub fn collect(&mut self) -> Result<IxValue, Error> {
		let mut visited = HashMap::<Vec<u8>, bool>::new();
		let mut result: VecDeque<Vertex> = VecDeque::new();
		let mut mutate_vertex = |v: &Vertex| {
			let key = v.id().bytes();
			if !visited.contains_key(&key) && v.has_label() {
				visited.insert(key.to_vec(), true);
				result.push_front(v.clone());
			}
		};
		while !self.steps.is_empty() {
			let ix: IxResult = self.steps.pop_back().unwrap();
			match ix.source.as_str() {
				"V" => {
					let vertices = ix.value.get::<Vec<Vertex>>().unwrap();
					for vertex in vertices {
						mutate_vertex(vertex);
					}
				}
				"addV" => {
					let vertex = ix.value.get::<Vertex>().unwrap();
					mutate_vertex(vertex);
				}
				_ => {}
			}
		}

		println!("Result: {:?}", result);
		println!("-----------------");
		Ok(IxValue::VertexSeq(result.into_iter().collect()))
	}

	pub async fn execute(
		&mut self,
		traversal: GraphTraversal<Vertex, Vertex, MockTerminator>,
	) -> Result<IxValue, Error> {
		let bytecode = traversal.bytecode();
		println!("Bytecode: {:?}", bytecode);
		println!("-----------------");
		for step in bytecode.steps() {
			match contain_source(step) {
				true => self.process_streaming_step(step).await,
				false => self.process_step(step).await,
			}
		}

		self.collect()
	}

	/// The V()-step is meant to read vertices from the graph and is usually
	/// used to start a GraphTraversal, but can also be used mid-traversal.
	async fn v(&mut self, args: &Vec<GValue>) -> IxResult<'a> {
		let tx = &mut self.v.mut_tx();
		let result = self.v.v(tx, args).await.unwrap();

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
		let tx = &mut self.v.mut_tx();
		let vertex = self.v.new_v(tx, args).await.unwrap();
		tx.commit().await.unwrap();

		IxResult::new("addV", IxValue::Vertex(vertex))
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
			mut v => {
				for cur in v.iter_mut() {
					let vertex_result = self.v.property(cur, tx, args).await.unwrap();
					result.push(vertex_result);
				}
			}
		}
		tx.commit().await.unwrap();

		IxResult::new("vertex_property", IxValue::VertexSeq(result))
	}

	async fn add_vertex_property(&mut self, args: &Vec<GValue>) -> IxResult<'a> {
		let tx = &mut self.v.mut_tx();
		let vertex = &*self.top_step().value.get::<Vertex>().unwrap();
		let result = self.v.property(&mut vertex.clone(), tx, args).await.unwrap();

		IxResult::new("vertex_property", IxValue::Vertex(result))
	}

	/// The property()-step is used to add properties to the elements of the graph (sideEffect).
	/// Unlike addV() and addE(), property() is a full sideEffect step in that it does not return
	/// the property it created, but the element that streamed into it. Moreover, if property()
	/// follows an addV() or addE(), then it is "folded" into the previous step to enable vertex
	/// and edge creation with all its properties in one creation operation.
	/// [Documentation](https://tinkerpop.apache.org/docs/current/reference/#property-step)
	async fn property(&mut self, args: &Vec<GValue>) -> IxResult<'a> {
		match args.first().unwrap().is_cardinality() {
			true => self.property_with_cardinality(args).await,
			false => {
				let stream = self.top_step();
				match stream.source.as_str() {
					"V" => self.vertex_property(args).await,
					"addV" => self.add_vertex_property(args).await,
					_ => unimplemented!(),
				}
			}
		}
	}

	fn get_streamed_vertices(&mut self) -> Vec<Vertex> {
		let stream = self.top_step();
		let default = Vec::default();

		let vertices = stream.value.get::<Vec<Vertex>>().unwrap_or(&default);
		vertices.to_vec()
	}

	fn top_step(&self) -> &IxResult<'_> {
		self.steps.back().unwrap()
	}
}

#[cfg(test)]
mod test {
	use crate::{storage::Datastore, util::generate_path, Database};
	use gremlin::{
		process::traversal::{GraphTraversalSource, MockTerminator},
		GValue, Vertex,
	};

	#[tokio::test]
	async fn vertex_with_property() {
		let path = &generate_path(None);
		let datastore = Datastore::new(path);
		let g = GraphTraversalSource::<MockTerminator>::empty();
		let mut db = Database::new(datastore.borrow(), &g);

		let command = db.traverse().v(1).add_v("person").property("github", "chungquantin");
		let result = db.execute(command).await.unwrap();

		let vertices = result.get::<Vec<Vertex>>().unwrap();
		assert_eq!(vertices.len(), 1);

		let vertex = vertices.first().unwrap();
		let vertex_property = vertex.property("github").unwrap();
		assert_eq!(vertex.label(), "person");
		assert_eq!(
			vertex_property.first().unwrap().value(),
			&GValue::String("chungquantin".to_string())
		);
	}

	#[tokio::test]
	async fn vertex_with_many_property() {
		let path = &generate_path(None);
		let datastore = Datastore::new(path);
		let g = GraphTraversalSource::<MockTerminator>::empty();
		let mut db = Database::new(datastore.borrow(), &g);

		let command = db.traverse().add_v("person").property_many(vec![
			("birthday", "1/11/2001"),
			("github", "chungquantin"),
			("name", "Tin Chung"),
		]);

		let result = db.execute(command).await.unwrap();
		let vertices = result.get::<Vec<Vertex>>().unwrap();
		assert_eq!(vertices.len(), 1);

		let vertex = vertices.first().unwrap();
		assert_eq!(vertex.label(), "person");
		let name = vertex.property("name").unwrap();
		assert_eq!(name[0].value(), &GValue::String("Tin Chung".to_string()));
		let birthday = vertex.property("birthday").unwrap();
		assert_eq!(birthday[0].value(), &GValue::String("1/11/2001".to_string()));
		let github = vertex.property("github").unwrap();
		assert_eq!(github[0].value(), &GValue::String("chungquantin".to_string()));
	}

	#[tokio::test]
	async fn vertex_property() {
		let path = &generate_path(None);
		let datastore = Datastore::new(path);
		let g = GraphTraversalSource::<MockTerminator>::empty();
		let mut db = Database::new(datastore.borrow(), &g);

		let command = db.traverse().add_v("person").property_many(vec![
			("github", "chungquantin"),
			("github", "tin-snowflake"),
			("name", "Tin Chung"),
		]);

		let result = db.execute(command).await.unwrap();
		let vertices = result.get::<Vec<Vertex>>().unwrap();
		assert_eq!(vertices.len(), 1);

		let vertex = vertices.first().unwrap();
		println!("Vertex: {:?}", vertex);
		assert_eq!(vertex.label(), "person");
		let name = vertex.property("name").unwrap();
		assert_eq!(name[0].value(), &GValue::String("Tin Chung".to_string()));
		let github = vertex.property("github").unwrap();
		assert_eq!(github[0].value(), &GValue::String("chungquantin".to_string()));
		assert_eq!(github[1].value(), &GValue::String("tin-snowflake".to_string()));
	}

	#[tokio::test]
	async fn multiple_new_vertex() {
		let path = &generate_path(None);
		let datastore = Datastore::new(path);
		let g = GraphTraversalSource::<MockTerminator>::empty();
		let mut db = Database::new(datastore.borrow(), &g);

		let command =
			db.traverse().v(1).add_v("person").add_v("coder").property("github", "chungquantin");
		let t1 = db.execute(command).await.unwrap();
		let vertices = t1.get::<Vec<Vertex>>().unwrap();
		assert_eq!(vertices.len(), 2);

		let t2 = db.execute(db.traverse().v(())).await.unwrap();
		assert_eq!(t2.get::<Vec<Vertex>>().unwrap().len(), 2);

		let mut iter = vertices.iter();
		let person_vertex = iter.next().unwrap();
		assert_eq!(person_vertex.label(), "person");
		let coder_vertex = iter.next().unwrap();
		let github = coder_vertex.property("github").unwrap();
		assert_eq!(github[0].value(), &GValue::String("chungquantin".to_string()));
	}
}
