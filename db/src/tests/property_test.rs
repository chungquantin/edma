#[cfg(feature = "test-suite")]
#[cfg(test)]
#[tokio::test]
async fn should_create_property() {
	use crate::{PropType, PropertyController};

	let pc = PropertyController::default();
	let res = pc.create_property("Name", PropType::String).await.unwrap();
	let property = pc.get_property(res.id.as_bytes().to_vec()).await.unwrap();
	assert_eq!(property, res);
}

#[cfg(feature = "test-suite")]
#[cfg(test)]
#[tokio::test]
async fn should_create_properties() {
	use crate::{PropType, PropertyController};

	let pc = PropertyController::default();
	let properties = pc
		.create_properties(vec![("name", PropType::String), ("age", PropType::UInt128)])
		.await
		.unwrap();
	assert_eq!(properties.len(), 2);
}
