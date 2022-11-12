#[cfg(feature = "test-suite")]
#[cfg(test)]
mod test {
	#[tokio::test]
	async fn should_create_label() {
		use crate::LabelController;

		let lc = LabelController::default();
		let res = lc.create_label("Person").await.unwrap();
		let label = lc.get_label(res.id.as_bytes().to_vec()).await.unwrap();
		assert_eq!(label, res);
	}

	#[tokio::test]
	async fn should_create_labels() {
		use crate::LabelController;

		let lc = LabelController::default();
		let res = lc.create_labels(vec!["Person", "Human", "Coder"]).await.unwrap();
		let label = lc.get_label(res[0].id.as_bytes().to_vec()).await.unwrap();
		assert_eq!(label, res[0]);
	}
}
