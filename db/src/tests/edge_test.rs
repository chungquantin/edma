// #[cfg(feature = "test-suite")]
#[cfg(test)]
mod test {
	use serde_json::json;

	use crate::{
		storage::{DBRef, Datastore},
		util::generate_path,
		EdgeController, EdgePropertyController,
	};

	#[tokio::test]
	async fn should_create_edge() {
		use crate::{LabelController, VertexController};

		let path = generate_path(None);
		let ds = Datastore::new(&path).unwrap();
		let r = DBRef::new(&ds);
		let vc = VertexController::new(r);
		let ec = EdgeController::new(r);
		let lc = LabelController::new(r);
		let epc = EdgePropertyController::new(r);

		let raw_labels = ["Person", "Student", "Employee"];

		let labels = lc.create_labels(raw_labels.to_vec()).await.unwrap();
		let v1 = vc
			.create_vertex(
				labels.to_vec(),
				json!({
					"name": "mock example"
				}),
			)
			.await
			.unwrap();
		assert_eq!(v1.labels.len(), raw_labels.len());

		let v2 = vc
			.create_vertex(
				labels.to_vec(),
				json!({
					"name": "mock example"
				}),
			)
			.await
			.unwrap();

		let edge = ec
			.create(
				v1.id,
				v2.id,
				"LIKE",
				json!({
					"name": "mock example"
				}),
			)
			.await
			.unwrap();

		let value = epc.count().await.unwrap();
		println!("Value: {:?}", value);

		let res = ec.get(v1.id, v2.id, "LIKE").await.unwrap();

		assert_eq!(edge, res);
		assert_eq!(res.source, v1.id);
		assert_eq!(res.target, v2.id);
	}
}
