// use std::collections::HashSet;

// use uuid::Uuid;

// use crate::{storage::DatastoreManager, Error, Vertex};

// /// T: Database transaction type
// pub struct VertexController {}

// impl VertexController {
// 	pub fn new(ds: DatastoreManager) -> Result<Self, Error> {
// 		Ok(VertexController {
// 			ds: ds.default(),
// 		})
// 	}

// 	pub fn create_vertex(&self, labels: Vec<Uuid>, props: HashSet<Uuid, Vec<u8>>) {
// 		let vertex = Vertex::new(labels, props);
// 	}
// }
