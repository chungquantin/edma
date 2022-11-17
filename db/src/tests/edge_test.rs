// #[cfg(feature = "test-suite")]
#[cfg(test)]
mod test {
	use serde_json::json;

	use crate::{
		storage::{DBRef, Datastore},
		util::generate_path,
		EdgeController,
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

		let raw_labels = ["Person", "Student", "Employee"];

		let labels = lc.multi_create(raw_labels.to_vec()).await.unwrap();
		let v1 = vc
			.create(
				labels.to_vec(),
				json!({
					"name": "mock example"
				}),
			)
			.await
			.unwrap();
		assert_eq!(v1.labels.len(), raw_labels.len());

		let v2 = vc
			.create(
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
				"LIKE",
				v2.id,
				json!({
					"name": "mock example"
				}),
			)
			.await
			.unwrap();

		let res = ec.get(v1.id, "LIKE", v2.id).await.unwrap();

		assert_eq!(edge, res);
		assert_eq!(res.in_id, v1.id);
		assert_eq!(res.out_id, v2.id);
	}
}
