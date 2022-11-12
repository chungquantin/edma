#[cfg(feature = "test-suite")]
#[cfg(test)]
#[tokio::test]
async fn should_create_label() {
	use std::collections::HashMap;

	use crate::{LabelController, PropType, PropertyController, VertexController};

	let vc = VertexController::default();
	let lc = LabelController::default();
	let pc = PropertyController::default();

	let raw_labels = ["Person", "Student", "Employee"];
	let properties = pc
		.create_properties(vec![
			("name", PropType::String),
			("age", PropType::UInt128),
			("addresses", PropType::VecString),
		])
		.await
		.unwrap();
	let labels = lc.create_labels(raw_labels.to_vec()).await.unwrap();
	let res = vc
		.create_vertex(
			labels,
			HashMap::from([
				(properties[0].id, "example name 1234".as_bytes().to_vec()),
				(properties[1].id, Vec::from([15])),
				(properties[2].id, ["address 1", "address 2"].concat().as_bytes().to_vec()),
			]),
		)
		.await
		.unwrap();
	assert_eq!(res.labels.len(), raw_labels.len());

	let vertex = vc.get_vertex(res.id.as_bytes().to_vec()).await.unwrap();
	assert_eq!(vertex, res);
	assert_eq!(vertex.labels.len(), raw_labels.len());
}
