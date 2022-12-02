use crate::{storage::Datastore, util::generate_path, Database};
use solomon_gremlin::{structure::Predicate, GValue};

pub async fn vertex_with_property(storage: &str) {
	let path = &generate_path(storage, None);
	let datastore = Datastore::new(path);
	let db = Database::new(datastore.borrow());

	let result = db
		.traverse()
		.v(1)
		.add_v("person")
		.property("github", "chungquantin")
		.property("twitter", "chasechung111")
		.property("age", 21)
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

	let result = db.traverse().v(vertex.id()).properties(()).exec().to_list().await.unwrap();
	assert_eq!(result.len(), 3);

	// non existing property
	let result = db.traverse().v(vertex.id()).properties("name").exec().to_list().await.unwrap();
	assert_eq!(result.len(), 0);

	// get example specific property
	let result = db.traverse().v(vertex.id()).properties("github").exec().to_list().await.unwrap();
	assert_eq!(result.len(), 1);
}

pub async fn vertex_with_many_property(storage: &str) {
	let path = &generate_path(storage, None);
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

pub async fn vertices_iter(storage: &str) {
	let path = &generate_path(storage, None);
	let datastore = Datastore::new(path);
	let db = Database::new(datastore.borrow());

	let mut exec = db.traverse().add_v("person").add_v("rustacean").exec();
	let get_vertex_one = exec.next().await.unwrap();
	let vertex_one = get_vertex_one.unwrap();
	assert!(vertex_one.has_label());
	assert_eq!(vertex_one.label(), "person");

	let get_vertex_two = exec.next().await.unwrap();
	let vertex_two = get_vertex_two.unwrap();
	assert!(vertex_two.has_label());
	assert_eq!(vertex_two.label(), "rustacean");
}

pub async fn vertex_property(storage: &str) {
	let path = &generate_path(storage, None);
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
	assert_eq!(vertex.label(), "person");
	let name = vertex.property("name").unwrap();
	assert_eq!(name[0].value(), &GValue::String("Tin Chung".to_string()));
	let github = vertex.property("github").unwrap();
	assert_eq!(github[0].value(), &GValue::String("chungquantin".to_string()));
	assert_eq!(github[1].value(), &GValue::String("tin-snowflake".to_string()));

	let properties_count =
		db.traverse().v(()).properties("github").count().exec().done().await.unwrap();
	assert_eq!(properties_count, 2);
}

pub async fn multiple_new_vertex(storage: &str) {
	let path = &generate_path(storage, None);
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

	let count = db.traverse().v(()).count().exec().done().await.unwrap();
	assert_eq!(count, vertices.len() as i64);

	let properties_count =
		db.traverse().v(()).properties("github").count().exec().done().await.unwrap();
	assert_eq!(properties_count, 1);
}

pub async fn vertex_has_step(storage: &str) {
	let path = &generate_path(storage, None);
	let datastore = Datastore::new(path);
	let db = Database::new(datastore.borrow());

	let t1 = db
		.traverse()
		.v(1)
		.add_v("person")
		.property("github", "tin-snowflake")
		.property("name", "Tin Chung")
		.property("age", 21)
		.add_v("coder")
		.property("github", "chungquantin")
		.property("age", 30);

	let t2 = t1.clone().has_key("github").has_label("person").exec().to_list().await.unwrap();
	let t3 = t1.clone().has_key("github").exec().to_list().await.unwrap();
	let t4 = t1.clone().has_not("name").exec().next().await.unwrap();
	let t5 = t1
		.clone()
		.has(("age", Predicate::within((21, 24))))
		.properties(())
		.exec()
		.to_list()
		.await
		.unwrap();

	assert_eq!(t2.len(), 1);
	assert_eq!(t3.len(), 2);
	assert_eq!(t4.unwrap().label(), "coder");
	assert_eq!(t5.len(), 3);

	let mut iter = t3.iter();
	let person_vertex = iter.next().unwrap();
	assert_eq!(person_vertex.label(), "person");
	let coder_vertex = iter.next().unwrap();
	assert_eq!(coder_vertex.label(), "coder");
}
