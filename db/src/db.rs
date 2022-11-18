use gremlin::{
	process::traversal::{Bytecode, GraphTraversalSource, MockTerminator},
	GValue,
};

use crate::{
	err::Error, storage::DBRef, EdgePropertyRepository, EdgeRepository, LabelRepository,
	VertexPropertyRepository, VertexRepository,
};

type TraversalSource = GraphTraversalSource<MockTerminator>;
pub struct Database<'a> {
	traversal: &'a TraversalSource,
	pub vertex: VertexRepository<'a>,
	pub vertex_property: VertexPropertyRepository<'a>,
	pub edge: EdgeRepository<'a>,
	pub edge_property: EdgePropertyRepository<'a>,
	pub label: LabelRepository<'a>,
}

impl<'a> Database<'a> {
	pub fn new(ds_ref: DBRef<'a>, traversal: &'a TraversalSource) -> Self {
		Database {
			traversal,
			vertex: VertexRepository::new(ds_ref),
			vertex_property: VertexPropertyRepository::new(ds_ref),
			edge: EdgeRepository::new(ds_ref),
			edge_property: EdgePropertyRepository::new(ds_ref),
			label: LabelRepository::new(ds_ref),
		}
	}

	pub fn traverse(&self) -> &TraversalSource {
		self.traversal
	}

	pub async fn execute(&self, bytecode: &Bytecode) -> Result<(), Error> {
		for step in bytecode.steps() {
			let args = step.args();
			match step.operator().as_str() {
				"V" => self.v(args).await,
				"addV" => self.add_v(args).await,
				"E" => self.e(args),
				"addE" => self.add_e(args),
				_ => todo!(),
			}
		}
		Ok(())
	}

	/// The V()-step is meant to read vertices from the graph and is usually
	/// used to start a GraphTraversal, but can also be used mid-traversal.
	async fn v(&self, ids: &Vec<GValue>) {
		println!("Vertex: {:?}", ids);

		let result = vec![self.vertex.v(ids).await.unwrap()];

		println!("=> Result: {:?}", result);
	}

	fn e(&self, ids: &Vec<GValue>) {
		println!("Edge {:?}", ids);
	}

	async fn add_v(&self, labels: &Vec<GValue>) {
		println!("Add vertex {:?}", labels);

		let res = self.vertex.add_v(labels).await.unwrap();
		let result = vec![Some(res)];

		println!("=> Add vertex {:?}", result);
	}

	fn add_e(&self, labels: &Vec<GValue>) {
		println!("Add edge {:?}", labels);
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
		let db = Database::new(datastore.borrow(), &g);

		let t = db.traverse().v(()).add_v("person");
		let bytecode = t.bytecode();
		db.execute(bytecode).await.unwrap();
		println!("Bytecode: {:?}", bytecode);
	}
}
