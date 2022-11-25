use crate::{storage::DatastoreRef, GraphTerminator};
use gremlin::process::traversal::GraphTraversalSource;

type TraversalSource<'a> = GraphTraversalSource<GraphTerminator<'a>>;

pub struct Database<'a> {
	traversal: TraversalSource<'a>,
}

impl<'a> Database<'a> {
	pub fn new(ds_ref: DatastoreRef<'a>) -> Self {
		let terminator = GraphTerminator::new(ds_ref);
		let traversal = GraphTraversalSource::new(terminator);

		Database {
			traversal,
		}
	}

	pub fn traverse(&self) -> TraversalSource {
		self.traversal.clone()
	}
}

#[cfg(test)]
mod test {
	use crate::{storage::Datastore, util::generate_path, Database};
	use gremlin::GValue;

	#[tokio::test]
	async fn vertex_with_property() {
		let path = &generate_path(None);
		let datastore = Datastore::new(path);
		let db = Database::new(datastore.borrow());

		let result = db
			.traverse()
			.v(1)
			.add_v("person")
			.property("github", "chungquantin")
			.exec()
			.next()
			.await
			.unwrap();

		let vertex = result.unwrap();
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
		let db = Database::new(datastore.borrow());

		let exec = db
			.traverse()
			.add_v("person")
			.property_many(vec![
				("birthday", "1/11/2001"),
				("github", "chungquantin"),
				("name", "Tin Chung"),
			])
			.exec()
			.next()
			.await
			.unwrap();

		let vertex = exec.unwrap();
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
		let db = Database::new(datastore.borrow());

		let exec = db
			.traverse()
			.add_v("person")
			.property_many(vec![
				("github", "chungquantin"),
				("github", "tin-snowflake"),
				("name", "Tin Chung"),
			])
			.exec()
			.next()
			.await
			.unwrap();

		let vertex = exec.unwrap();
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
		let db = Database::new(datastore.borrow());

		let vertices = db
			.traverse()
			.v(1)
			.add_v("person")
			.add_v("coder")
			.property("github", "chungquantin")
			.exec()
			.to_list()
			.await
			.unwrap();

		assert_eq!(vertices.len(), 2);

		let mut iter = vertices.iter();
		let person_vertex = iter.next().unwrap();
		assert_eq!(person_vertex.label(), "person");
		let coder_vertex = iter.next().unwrap();
		let github = coder_vertex.property("github").unwrap();
		assert_eq!(github[0].value(), &GValue::String("chungquantin".to_string()));

		let vertices = db.traverse().v(()).exec().to_list().await.unwrap();
		assert_eq!(vertices.len(), 2);
	}
}
