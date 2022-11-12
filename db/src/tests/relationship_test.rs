#[cfg(feature = "test-suite")]
#[cfg(test)]
mod test {
	#[tokio::test]
	async fn should_create_relationship() {
		use std::collections::HashMap;

		use crate::{
			LabelController, PropType, PropertyController, RelationshipController, VertexController,
		};

		let vc = VertexController::default();
		let rc = RelationshipController::default();
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
		let v1 = vc
			.create_vertex(
				labels.to_vec(),
				HashMap::from([
					(properties[0].id, "example name 1234".as_bytes().to_vec()),
					(properties[1].id, Vec::from([15])),
					(properties[2].id, ["address 1", "address 2"].concat().as_bytes().to_vec()),
				]),
			)
			.await
			.unwrap();
		assert_eq!(v1.labels.len(), raw_labels.len());

		let v2 = vc
			.create_vertex(
				labels.to_vec(),
				HashMap::from([
					(properties[0].id, "vertex number 2".as_bytes().to_vec()),
					(properties[1].id, Vec::from([25])),
					(properties[2].id, ["address 1"].concat().as_bytes().to_vec()),
				]),
			)
			.await
			.unwrap();

		let relationship = rc
			.create_relationship(
				v1.id,
				v2.id,
				"LIKE",
				HashMap::from([
					(properties[0].id, "vertex number 2".as_bytes().to_vec()),
					(properties[1].id, Vec::from([25])),
					(properties[2].id, ["address 1"].concat().as_bytes().to_vec()),
				]),
			)
			.await
			.unwrap();

		let res = rc.get_relationship(v1.id, v2.id, "LIKE").await.unwrap();

		assert_eq!(relationship, res);
		assert_eq!(relationship.source, v1.id);
		assert_eq!(relationship.target, v2.id);
	}
}
