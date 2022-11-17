// #[cfg(feature = "test-suite")]
#[cfg(test)]
mod test {
	use crate::{
		storage::{DBRef, Datastore},
		util::generate_path,
	};

	#[tokio::test]
	async fn should_create_label() {
		use crate::LabelController;

		let path = generate_path(None);
		let ds = Datastore::new(&path).unwrap();
		let r = DBRef::new(&ds);
		let lc = LabelController::new(r);
		let res = lc.create("Person").await.unwrap();
		let label = lc.get(res.id.as_bytes().to_vec()).await.unwrap();
		assert_eq!(label, res);

		let res = lc.multi_create(vec!["Person", "Human", "Coder"]).await.unwrap();
		let label = lc.get(res[0].id.as_bytes().to_vec()).await.unwrap();
		assert_eq!(label, res[0]);
	}
}
