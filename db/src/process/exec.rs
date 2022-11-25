use std::collections::{HashMap, LinkedList, VecDeque};
use std::marker::PhantomData;

use crate::{err::Error, storage::DatastoreRef, IxResult, SimpleTransaction, VertexRepository};
use gremlin::process::traversal::{GraphTraversal, Terminator};
use gremlin::GremlinError;
use gremlin::{
	process::traversal::{Bytecode, Instruction},
	FromGValue, GValue, List, Vertex,
};

#[derive(Clone)]
pub struct StepExecutor<'a, T> {
	bytecode: Bytecode,
	steps: LinkedList<IxResult>,
	terminator: String,
	v: VertexRepository<'a>,
	phantom: PhantomData<T>,
	iter_index: usize,
}

fn contain_source(step: &Instruction) -> bool {
	let s = step.operator().as_str();
	s == "V" || s == "E" || s == "addV" || s == "addE"
}

impl<'a, T> StepExecutor<'a, T> {
	fn collect_vertex_property(&mut self) -> Result<GValue, Error>
	where
		T: FromGValue,
	{
		let mut result = vec![];
		let vertices = self.collect_vertex().unwrap();
		let list = vertices.get::<List>().unwrap();
		for item in list.iter() {
			let vertex = item.get::<Vertex>().unwrap();
			let properties = vertex.properties();
			for (_, property) in properties.iter() {
				for item in property {
					result.push(GValue::VertexProperty(item.clone()));
				}
			}
		}

		let list = List::new(result);
		Ok(GValue::List(list))
	}

	fn collect_vertex(&mut self) -> Result<GValue, Error>
	where
		T: FromGValue,
	{
		let mut iter: VecDeque<GValue> = VecDeque::new();
		let mut visited = HashMap::<Vec<u8>, bool>::new();
		let mut mutate_vertex = |v: &Vertex| {
			let key = v.id().bytes();
			if !visited.contains_key(&key) && v.has_label() {
				visited.insert(key.to_vec(), true);
				iter.push_front(GValue::Vertex(v.clone()));
			}
		};
		while !self.steps.is_empty() {
			let ix: IxResult = self.steps.pop_back().unwrap();
			match ix.source.as_str() {
				"V" => {
					let list = ix.value.get::<List>().unwrap();
					for item in list.iter() {
						let vertex = item.get::<Vertex>().unwrap();
						mutate_vertex(vertex);
					}
				}
				"addV" => {
					let vertex = ix.value.get::<Vertex>().unwrap();
					mutate_vertex(vertex);
				}
				_ => unimplemented!(),
			}
		}

		self.collect_debug(iter.clone());
		let vec: Vec<GValue> = iter.into_iter().collect();
		let list = List::new(vec);
		Ok(GValue::List(list))
	}

	fn collect(&mut self) -> Result<GValue, Error>
	where
		T: FromGValue,
	{
		match self.terminator.as_str() {
			"Vertex" => self.collect_vertex(),
			"VertexProperty" => self.collect_vertex_property(),
			_ => unimplemented!(),
		}
	}
}

impl<'a, T> StepExecutor<'a, T> {
	pub fn new<S, E>(traversal: &GraphTraversal<S, T, E>, ds_ref: DatastoreRef<'a>) -> Self
	where
		T: FromGValue,
		E: Terminator<T>,
	{
		StepExecutor {
			bytecode: traversal.bytecode().clone(),
			steps: LinkedList::<IxResult>::default(),
			terminator: String::default(),
			v: VertexRepository::new(ds_ref),
			phantom: PhantomData,
			iter_index: 0,
		}
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
			"properties" => self.properties(args).await,
			_ => unimplemented!(),
		};

		let stream = self.steps.back().unwrap();
		let source = &stream.source;
		step_result.set_source(source.to_string()).unwrap();
		self.steps.push_back(step_result);
	}

	async fn execute(&mut self) -> Result<GValue, GremlinError>
	where
		T: FromGValue,
	{
		self.bytecode_debug();

		for step in self.bytecode.clone().steps() {
			match contain_source(step) {
				true => self.process_streaming_step(step).await,
				false => self.process_step(step).await,
			}
		}
		Ok(self.collect().unwrap())
	}

	pub async fn to_list(&mut self) -> Result<Vec<T>, Error>
	where
		T: FromGValue + Clone,
	{
		let mut result = vec![];
		let exec = self.execute().await.unwrap();
		let list = exec.get::<List>().unwrap();
		for item in list.iter() {
			let value = T::from_gvalue(item.clone()).unwrap();
			result.push(value);
		}

		Ok(result)
	}

	pub async fn next(&mut self) -> Result<Option<T>, Error>
	where
		T: FromGValue + Clone,
	{
		let list = self.to_list().await.unwrap();
		Ok(if self.iter_index < list.len() {
			let result = list[self.iter_index].clone();
			let option = Some(result);
			self.iter_index += 1;
			option
		} else {
			None
		})
	}

	pub async fn has_next(&mut self) -> Result<bool, Error>
	where
		T: FromGValue + Clone,
	{
		let list = self.to_list().await.unwrap();
		Ok(self.iter_index + 1 < list.len())
	}

	/// The V()-step is meant to read vertices from the graph and is usually
	/// used to start a GraphTraversal, but can also be used mid-traversal.
	async fn v(&mut self, args: &Vec<GValue>) -> IxResult {
		let tx = &mut self.v.mut_tx();
		let result = self.v.v(tx, args).await.unwrap();

		self.set_terminator("Vertex");
		IxResult::new("V", GValue::List(result))
	}

	async fn e(&mut self, _ids: &Vec<GValue>) -> IxResult {
		IxResult::new("E", GValue::Null)
	}

	/// The addV()-step is used to add vertices to the graph (map/sideEffect).
	/// For every incoming object, a vertex is created. Moreover, GraphTraversalSource maintains an addV() method.
	/// [Documentation](https://tinkerpop.apache.org/docs/current/reference/#addvertex-step)
	async fn add_v(&mut self, args: &Vec<GValue>) -> IxResult {
		let tx = &mut self.v.mut_tx();
		let vertex = self.v.new_v(tx, args).await.unwrap();
		tx.commit().await.unwrap();

		self.set_terminator("Vertex");
		IxResult::new("addV", GValue::Vertex(vertex))
	}

	async fn add_e(&mut self, _labels: &Vec<GValue>) -> IxResult {
		self.set_terminator("Edge");
		IxResult::new("addE", GValue::Null)
	}

	async fn property_with_cardinality(&mut self, _args: &Vec<GValue>) -> IxResult {
		IxResult::new("property", GValue::Null)
	}

	async fn vertex_property(&mut self, args: &Vec<GValue>) -> IxResult {
		let tx = &mut self.v.mut_tx();
		let mut result: Vec<GValue> = vec![];
		let vertices = self.get_streamed_vertices();
		match vertices {
			v if v.is_empty() => {
				let vertex = self.v.new_property(tx, args).await.unwrap();
				result.push(GValue::Vertex(vertex))
			}
			mut v => {
				for cur in v.iter_mut() {
					let vertex = self.v.property(cur, tx, args).await.unwrap();
					result.push(GValue::Vertex(vertex));
				}
			}
		}
		tx.commit().await.unwrap();

		let list = GValue::List(List::new(result));
		IxResult::new("vertex_property", list)
	}

	async fn add_vertex_property(&mut self, args: &Vec<GValue>) -> IxResult {
		let tx = &mut self.v.mut_tx();
		let vertex = self.top_step().value.get::<Vertex>().unwrap();
		let result = self.v.property(&mut vertex.clone(), tx, args).await.unwrap();
		tx.commit().await.unwrap();

		IxResult::new("vertex_property", GValue::Vertex(result))
	}

	async fn properties(&mut self, _args: &Vec<GValue>) -> IxResult {
		let mut result = vec![];
		let stream = self.top_step();
		match stream.source.as_str() {
			"V" | "addV" => {
				let mut vertices = self.get_streamed_vertices();
				if !vertices.is_empty() {
					for cur in vertices.iter_mut() {
						let vertex = self.v.properties(cur).await.unwrap();
						result.push(GValue::Vertex(vertex));
					}
				}
			}
			_ => unimplemented!(),
		}

		self.set_terminator("VertexProperty");
		let list = GValue::List(List::new(result));
		IxResult::new("properties", list)
	}

	/// The property()-step is used to add properties to the elements of the graph (sideEffect).
	/// Unlike addV() and addE(), property() is a full sideEffect step in that it does not return
	/// the property it created, but the element that streamed into it. Moreover, if property()
	/// follows an addV() or addE(), then it is "folded" into the previous step to enable vertex
	/// and edge creation with all its properties in one creation operation.
	/// [Documentation](https://tinkerpop.apache.org/docs/current/reference/#property-step)
	async fn property(&mut self, args: &Vec<GValue>) -> IxResult {
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

		let mut result = vec![];
		match &stream.value {
			GValue::Vertex(v) => result.push(v.clone()),
			GValue::List(v) => {
				let iter = v;
				for v in iter.iter() {
					let vertex = v.get::<Vertex>().unwrap().clone();
					result.push(vertex);
				}
			}
			_ => unimplemented!(),
		}

		result
	}

	fn top_step(&self) -> &IxResult {
		self.steps.back().unwrap()
	}

	fn collect_debug(&self, result: VecDeque<GValue>) {
		println!("==> Result");
		for (index, ix) in result.iter().enumerate() {
			println!("{:?} --- {:?}", index, ix);
		}
		println!("-----------------");
	}

	fn bytecode_debug(&self) {
		println!("==> Bytecode");
		for (index, ix) in self.bytecode.steps().iter().enumerate() {
			println!("Instruction {:?}: {:?}", index, ix);
		}
		println!("-----------------");
	}

	fn set_terminator(&mut self, token: &str) {
		self.terminator = token.to_string();
	}
}
