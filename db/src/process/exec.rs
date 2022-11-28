use std::marker::PhantomData;

use crate::{err::Error, storage::DatastoreRef, IxResult, SimpleTransaction, VertexRepository};
use gremlin::process::traversal::{GraphTraversal, Terminator};
use gremlin::GremlinError;
use gremlin::{
	process::traversal::{Bytecode, Instruction},
	FromGValue, GValue, List, Vertex,
};

#[derive(Clone, Debug)]
pub struct ExecutionResult {
	edges: IxResult,
	vertices: IxResult,
	new_vertex: IxResult,
	new_edge: IxResult,
}

impl Default for ExecutionResult {
	fn default() -> Self {
		let default_list = IxResult::new("", GValue::List(List::default()));
		println!("Default: {:?}", default_list);
		Self {
			edges: default_list.clone(),
			vertices: default_list,
			new_vertex: Default::default(),
			new_edge: Default::default(),
		}
	}
}

impl ExecutionResult {
	fn get_from_source(&self, source: &str) -> IxResult {
		match source {
			"E" => self.edges.clone(),
			"V" => self.vertices.clone(),
			"addV" => self.new_vertex.clone(),
			"addE" => self.new_edge.clone(),
			_ => unimplemented!(),
		}
	}
}

#[derive(Clone)]
pub struct StepExecutor<'a, T: FromGValue + Clone> {
	bytecode: Bytecode,
	result: ExecutionResult,
	terminator: String,
	source: String,
	v: VertexRepository<'a>,
	phantom: PhantomData<T>,
	iter_index: usize,
}

fn is_streaming_vertex_step(s: &str) -> bool {
	s == "V" || s == "addV"
}

fn is_streaming_edge_step(s: &str) -> bool {
	s == "E" || s == "addE"
}

fn is_streaming_source_step(s: &str) -> bool {
	is_streaming_edge_step(s) || is_streaming_vertex_step(s)
}

/// # ReducingBarrierStep()
/// All of the traversers prior to the step are processed by a reduce function and once
/// all the previous traversers are processed, a single "reduced value" traverser is emitted
/// to the next step. Note that the path history leading up to a reducing barrier step is
/// destroyed given its many-to-one nature.
fn is_reducing_barrier_step(s: &str) -> bool {
	s == "fold" || s == "count" || s == "sum" || s == "max" || s == "min"
}

impl<'a, T: FromGValue + Clone> StepExecutor<'a, T> {
	fn collect_vertex_list(&mut self) -> List {
		let mut list = self.get_from_source::<List>("V").unwrap();
		let new_vertex = self.result.get_from_source("addV").value;
		if !new_vertex.is_null() {
			list.push(new_vertex);
		}

		list
	}

	fn collect_vertex(&mut self) -> GValue {
		GValue::List(self.collect_vertex_list())
	}

	fn collect_vertex_properties(&mut self) -> GValue {
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

	fn collect(&mut self) -> Result<GValue, Error>
	where
		T: FromGValue,
	{
		match self.terminator.as_str() {
			"Vertex" => Ok(self.collect_vertex()),
			"VertexProperty" => Ok(self.collect_vertex_properties()),
			_ => unimplemented!(),
		}
	}

	pub fn new<S, E>(traversal: &GraphTraversal<S, T, E>, ds_ref: DatastoreRef<'a>) -> Self
	where
		T: FromGValue,
		E: Terminator<T>,
	{
		StepExecutor {
			bytecode: traversal.bytecode().clone(),
			result: ExecutionResult::default(),
			terminator: String::default(),
			source: String::default(),
			v: VertexRepository::new(ds_ref),
			phantom: PhantomData,
			iter_index: 0,
		}
	}

	async fn process_streaming_step(&mut self, step: &Instruction) {
		let args = step.args();
		let operator = step.operator().as_str();
		let source = operator.to_string();
		match operator {
			"V" => {
				let result = self.v(args).await;
				self.result.vertices = result;
			}
			"E" => {
				let result = self.e(args).await;
				self.result.edges = result;
			}
			"addV" => {
				let result = self.add_v(args).await;
				self.result.new_vertex = result;
			}
			"addE" => {
				let result = self.add_e(args).await;
				self.result.new_edge = result;
			}
			_ => unimplemented!(),
		};

		self.source = source;
	}

	async fn process_reducing_barrier_step(&mut self, step: &Instruction) {
		let args = step.args();
		let operator = step.operator().as_str();
		match operator {
			"count" => self.count(args).await,
			_ => unimplemented!(),
		};
	}

	async fn process_step(&mut self, step: &Instruction) {
		let args = step.args();
		let operator = step.operator().as_str();
		match operator {
			"property" => self.property(args).await,
			"properties" => self.properties(args).await,
			"count" => self.count(args).await,
			"hasLabels" => self.has_labels(args).await,
			"hasIds" => self.has_ids(args).await,
			_ => unimplemented!(),
		};
	}

	async fn execute(&mut self) -> Result<GValue, GremlinError>
	where
		T: FromGValue,
	{
		self.bytecode_debug();

		for step in self.bytecode.clone().steps() {
			println!("==> result: {:?}", self.result);
			match step.operator().as_str() {
				s if is_streaming_source_step(s) => self.process_streaming_step(step).await,
				s if is_reducing_barrier_step(s) => self.process_reducing_barrier_step(step).await,
				_ => self.process_step(step).await,
			}
		}
		let result = self.collect().unwrap();
		self.collect_debug(result.clone());
		Ok(result)
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

		if !self.result.new_vertex.is_empty() {
			// Push new vertex to the end of vertices
			let committed_vertex = self.result.new_vertex.clone();
			println!("Commited vertex: {:?}", committed_vertex);
			let mut vertices = self.get_from_source::<List>("V").unwrap();
			vertices.push(committed_vertex.value);
			self.result.vertices.value = GValue::List(vertices);
		}

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
		let source = &self.source.clone();
		let vertices = self.get_list_from_source::<Vertex>(source).unwrap();
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
		let stream = self.result.get_from_source(&self.source);
		let default = Vertex::default();
		let vertex = stream.value.get::<Vertex>().unwrap_or(&default);

		let result = self.v.property(&mut vertex.clone(), tx, args).await.unwrap();
		let value = GValue::Vertex(result);
		self.result.new_vertex.value = value.clone();
		tx.commit().await.unwrap();

		IxResult::new("vertex_property", value)
	}

	async fn vertices_properties(&mut self, args: &Vec<GValue>) -> IxResult {
		let mut result = vec![];
		let source = &self.source.clone();
		let mut vertices = self.get_list_from_source::<Vertex>(source).unwrap();
		if !vertices.is_empty() {
			for cur in vertices.iter_mut() {
				let vertex = self.v.properties(cur, args).await.unwrap();
				result.push(GValue::Vertex(vertex));
			}
			self.result.vertices.value = GValue::List(List::new(result.clone()));
		}
		self.set_terminator("VertexProperty");
		let list = GValue::List(List::new(result));
		IxResult::new("properties", list)
	}

	async fn new_vertex_properties(&mut self, args: &Vec<GValue>) -> IxResult {
		let source = &self.source.clone();
		let mut vertex = self.get_from_source::<Vertex>(source).unwrap();
		let vertex = self.v.properties(&mut vertex, args).await.unwrap();
		let result = GValue::Vertex(vertex);
		self.result.vertices.value = result.clone();

		self.set_terminator("VertexProperty");
		IxResult::new("properties", result)
	}

	async fn properties(&mut self, args: &Vec<GValue>) -> IxResult {
		println!("Source: {:?}", self.result);
		match self.source.as_str() {
			"V" => self.vertices_properties(args).await,
			"addV" => self.new_vertex_properties(args).await,
			_ => unimplemented!(),
		}
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
			false => match self.source.as_str() {
				"V" => self.vertex_property(args).await,
				"addV" => self.add_vertex_property(args).await,
				_ => unimplemented!(),
			},
		}
	}

	async fn count(&mut self, _args: &Vec<GValue>) -> IxResult
	where
		T: FromGValue + Clone,
	{
		self.set_terminator("Int32");
		IxResult::new("count", GValue::Null)
	}

	async fn has_labels(&mut self, _args: &Vec<GValue>) -> IxResult {
		unimplemented!()
	}

	async fn has_ids(&mut self, _args: &Vec<GValue>) -> IxResult {
		unimplemented!()
	}

	fn get_from_source<E>(&mut self, source: &str) -> Result<E, Error>
	where
		E: FromGValue,
	{
		let stream = self.result.get_from_source(source);
		let item = E::from_gvalue(stream.value).unwrap();
		Ok(item)
	}

	fn get_list_from_source<E>(&mut self, source: &str) -> Result<Vec<E>, Error>
	where
		E: FromGValue,
	{
		let stream = self.result.get_from_source(source);
		let list = stream.value.get::<List>().unwrap();
		let mut result = vec![];
		for item in list.iter() {
			let value = E::from_gvalue(item.clone()).unwrap();
			result.push(value);
		}
		Ok(result)
	}

	fn collect_debug(&self, result: GValue) {
		println!("==> Result");
		println!("{:?}", result);
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
