use crate::{
	interface::KeyValuePair,
	storage::Transaction,
	util::{build_byte_map, build_sized, concat_bytes, Component},
	Error, PropertyRepository, SimpleTransaction,
};
use solomon_gremlin::{Edge, GValue, Labels, List, Vertex, GID};

impl_repository!(EdgeRepository(Edge));

type RepositoryResult<T> = Result<T, Error>;
const UUID_SIZE: usize = 16;

/// Byte layout:
/// ( size + edge id | size + incoming vertex id | size + outgoing vertex id )
fn build_edge_key<'a>(edge: &Edge) -> Vec<u8> {
	let vertex = |v: &Option<Vertex>| {
		let vec = vec![0; UUID_SIZE];
		let empty_uuid = vec.as_slice();
		let result = {
			match v {
				Some(v) => v.id().clone().bytes(),
				None => empty_uuid.to_vec(),
			}
		};

		result
	};
	let in_v = vertex(edge.in_v());
	let in_v_component = Component::Bytes(in_v.as_slice());
	let out_v = vertex(edge.out_v());
	let out_v_component = Component::Bytes(out_v.as_slice());
	let bytes = concat_bytes(vec![
		build_sized(Component::Gid(edge.id())),
		build_sized(in_v_component),
		build_sized(out_v_component),
	]);
	println!("bytes: {:?}", build_sized(Component::Gid(edge.id())));
	bytes
}

impl<'a> EdgeRepository<'a> {
	/// The V()-step is meant to read vertices from the graph and is usually
	/// used to start a GraphTraversal, but can also be used mid-traversal.
	/// [Documentation](https://tinkerpop.apache.org/docs/current/reference/#v-step)
	pub async fn e(&self, tx: &Transaction, ids: &[GValue]) -> RepositoryResult<List> {
		let cf = self.cf();

		match ids.first() {
			Some(id) => {
				let key = build_sized(Component::GValue(id));
				let get_vertex = tx.get(cf, key.to_vec()).await.unwrap();
				Ok(List::new(match get_vertex {
					Some(v) => {
						let value = GValue::Edge(self.from_pair(&(key, v)).unwrap());
						vec![value]
					}
					None => vec![],
				}))
			}
			_ => self.iterate_all(tx).await,
		}
	}

	pub async fn new_e(&self, tx: &mut Transaction, labels: &[GValue]) -> RepositoryResult<Edge> {
		let new_e = &mut Edge::default();
		self.add_e(tx, new_e, labels).await
	}

	/// Adds a Edge with an edge label determined by a Traversal.
	pub async fn add_e(
		&self,
		tx: &mut Transaction,
		e: &mut Edge,
		labels: &[GValue],
	) -> RepositoryResult<Edge> {
		let mut serialized_labels = Vec::<String>::new();
		for label in labels.iter() {
			let val = label.get::<String>().unwrap();
			serialized_labels.push(val.to_string());
			e.add_label(val);
		}
		let labels = Labels::from(serialized_labels);

		// build Label byte (length : usize, data: LabelType)
		let mut bytes = vec![];
		for label in labels.0.iter() {
			let byte = build_sized(Component::Label(label));
			bytes.push(byte);
		}

		let cf = self.cf();

		let key = build_edge_key(e);
		let val = bytes.concat();

		tx.put(cf, key, val).await.unwrap();

		Ok(e.clone())
	}

	// If there is no vertices defined, initialized with default option
	pub async fn new_property(
		&self,
		tx: &mut Transaction,
		args: &[GValue],
	) -> RepositoryResult<Edge> {
		let vertex = &mut Edge::default();
		self.property(vertex, tx, args).await
	}

	pub fn property_repo(&self) -> PropertyRepository {
		PropertyRepository::new(self.ds_ref)
	}

	pub async fn property(
		&self,
		e: &mut Edge,
		tx: &mut Transaction,
		args: &[GValue],
	) -> RepositoryResult<Edge> {
		let property_repo = self.property_repo();
		let (label, value) = (&args[0], &args[1]);
		let property = property_repo.property(tx, e.id(), label, value).await.unwrap();
		e.add_property(property);
		Ok(e.clone())
	}

	pub async fn in_v(
		&self,
		tx: &mut Transaction,
		e: &mut Edge,
		v: &Vertex,
	) -> RepositoryResult<Edge> {
		let cf = self.cf();
		let old_key = build_edge_key(e);

		if tx.exi(cf.clone(), old_key.clone()).await.unwrap() {
			tx.del(cf.clone(), old_key).await.unwrap();
			// Update edge in storage with new key
			e.set_in_v(v.clone());
			let new_key = build_edge_key(e);
			let val = build_sized(Component::FixedLengthString(e.label()));
			tx.put(cf, new_key, val).await.unwrap();
		}
		Ok(e.clone())
	}

	pub async fn out_v(
		&self,
		tx: &mut Transaction,
		e: &mut Edge,
		v: &Vertex,
	) -> RepositoryResult<Edge> {
		let cf = self.cf();
		let old_key = build_edge_key(e);

		if tx.exi(cf.clone(), old_key.clone()).await.unwrap() {
			tx.del(cf.clone(), old_key).await.unwrap();
			// Update edge in storage with new key
			e.set_out_v(v.clone());
			let new_key = build_edge_key(e);
			println!("New key: {:?}", new_key);
			let val = build_sized(Component::FixedLengthString(e.label()));
			tx.put(cf, new_key, val).await.unwrap();
		}
		Ok(e.clone())
	}

	pub async fn properties(
		&self,
		tx: &Transaction,
		e: &mut Edge,
		args: &[GValue],
	) -> RepositoryResult<Edge> {
		let property_repo = self.property_repo();
		let properties = match args.first() {
			Some(label) => property_repo.iterate_from_label(tx, e.id(), label).await.unwrap(),
			None => property_repo.iterate_from_edge(tx, e.id()).await.unwrap(),
		};
		e.add_properties(properties);
		Ok(e.clone())
	}

	fn from_pair(&self, p: &KeyValuePair) -> RepositoryResult<Edge> {
		let (k, v) = p;
		// Handle deserializing and rebuild edge stream
		let bytemap = &build_byte_map(vec!["gid", "in_v", "out_v"], k.to_vec());
		let gid = GID::Bytes(bytemap.get("gid").unwrap().to_vec());
		let mut edge = Edge::partial_new(gid);
		let in_v = GID::Bytes(bytemap.get("in_v").unwrap().to_vec());
		edge.set_partial_in_v(in_v);
		let out_v = GID::Bytes(bytemap.get("out_v").unwrap().to_vec());
		edge.set_partial_out_v(out_v);
		// handle deserializing label data of edge
		let mut i = 0;
		while i < v.len() {
			let len = *v[i..i + 1].first().unwrap();
			let usize_len = len as usize;
			let end = usize_len + i + 1;
			let data = &v[i + 1..end];
			let label = String::from_utf8(data.to_vec()).unwrap();
			edge.add_label(label);
			i += usize_len + 1;
		}

		Ok(edge)
	}

	pub async fn drop_v(&self, tx: &mut Transaction, id: GID) -> Result<(), Error> {
		let cf = self.cf();
		let value = id.get::<String>().unwrap();
		let gid = &GID::from(value.to_string());
		let key = build_sized(Component::Gid(gid));
		tx.del(cf, key).await.unwrap();
		Ok(())
	}

	fn iterate(&self, iterator: Vec<Result<KeyValuePair, Error>>) -> RepositoryResult<List> {
		let mut result: Vec<GValue> = vec![];
		for pair in iterator.iter() {
			let p_ref = pair.as_ref().unwrap();
			let edge = self.from_pair(p_ref).unwrap();
			result.push(GValue::Edge(edge));
		}

		Ok(List::new(result))
	}

	pub async fn iterate_all(&self, tx: &Transaction) -> RepositoryResult<List> {
		let cf = self.cf();

		let iterator = tx.iterate(cf).await.unwrap();
		self.iterate(iterator)
	}

	pub async fn iterate_from_edge(&self, tx: &Transaction, edge_id: &GID) -> Result<List, Error> {
		let cf = self.cf();
		let prefix = build_sized(Component::Gid(edge_id));
		let iterator = tx.prefix_iterate(cf, prefix).await.unwrap();
		self.iterate(iterator)
	}
}
