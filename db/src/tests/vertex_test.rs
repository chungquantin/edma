// #[cfg(feature = "test-suite")]
#[cfg(test)]
mod test {
	use serde_json::json;

	use crate::{
		storage::{DBRef, Datastore},
		util::generate_path,
	};

	#[tokio::test]
	async fn should_create_vertex() {
		use crate::{LabelController, VertexController};

		let path = generate_path(None);
		let ds = Datastore::new(&path).unwrap();
		let r = DBRef::new(&ds);
		let vc = VertexController::new(r);
		let lc = LabelController::new(r);

		let raw_labels = ["Person", "Student", "Employee"];
		let labels = lc.multi_create(raw_labels.to_vec()).await.unwrap();
		let res = vc
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

		let vertex = vc.get(res.id.as_bytes().to_vec()).await.unwrap();
		assert_eq!(vertex, res);
		assert_eq!(vertex.labels.len(), raw_labels.len());
	}
}
