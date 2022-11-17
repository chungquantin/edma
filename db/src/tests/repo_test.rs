// #[cfg(feature = "test-suite")]
#[cfg(test)]
mod repository_test {
	use serde_json::json;

	use crate::{
		storage::{DBRef, Datastore},
		util::generate_path,
		EdgeRepository, LabelRepository, VertexRepository,
	};

	#[tokio::test]
	async fn should_create_label() {
		let path = generate_path(None);
		let ds = Datastore::new(&path).unwrap();
		let r = DBRef::new(&ds);
		let lr = LabelRepository::new(r);
		let res = lr.create("Person").await.unwrap();
		let label = lr.get(res.id.as_bytes().to_vec()).await.unwrap();
		assert_eq!(label, res);

		let res = lr.multi_create(vec!["Person", "Human", "Coder"]).await.unwrap();
		let label = lr.get(res[0].id.as_bytes().to_vec()).await.unwrap();
		assert_eq!(label, res[0]);
	}

	#[tokio::test]
	async fn should_create_vertex() {
		let path = generate_path(None);
		let ds = Datastore::new(&path).unwrap();
		let r = DBRef::new(&ds);
		let vr = VertexRepository::new(r);
		let lr = LabelRepository::new(r);

		let raw_labels = ["Person", "Student", "Employee"];
		let labels = lr.multi_create(raw_labels.to_vec()).await.unwrap();
		let res = vr
			.create(
				labels,
				json!({
					"name": "example name",
					"age": 12
				}),
			)
			.await
			.unwrap();
		assert_eq!(res.labels.len(), raw_labels.len());

		let vertex = vr.get(res.id.as_bytes().to_vec()).await.unwrap();
		assert_eq!(vertex, res);
		assert_eq!(vertex.labels.len(), raw_labels.len());
	}

	#[tokio::test]
	async fn should_create_edge() {
		let path = generate_path(None);
		let ds = Datastore::new(&path).unwrap();
		let r = DBRef::new(&ds);
		let vr = VertexRepository::new(r);
		let er = EdgeRepository::new(r);
		let lr = LabelRepository::new(r);

		let raw_labels = ["Person", "Student", "Employee"];

		let labels = lr.multi_create(raw_labels.to_vec()).await.unwrap();
		let v1 = vr
			.create(
				labels.to_vec(),
				json!({
					"name": "mock example"
				}),
			)
			.await
			.unwrap();
		assert_eq!(v1.labels.len(), raw_labels.len());

		let v2 = vr
			.create(
				labels.to_vec(),
				json!({
					"name": "mock example"
				}),
			)
			.await
			.unwrap();

		let edge = er
			.create(
				v1.id,
				"LIKE",
				v2.id,
				json!({
					"name": "mock example"
				}),
			)
			.await
			.unwrap();

		let res = er.get(v1.id, "LIKE", v2.id).await.unwrap();

		assert_eq!(edge, res);
		assert_eq!(res.in_id, v1.id);
		assert_eq!(res.out_id, v2.id);
	}
}
