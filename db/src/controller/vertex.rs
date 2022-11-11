use std::collections::HashMap;

use crate::{
	model::adapter::DatastoreAdapter,
	serialize_discriminator,
	util::{build_bytes, build_offset, deserialize_full_data, from_uuid_bytes, Component},
	AccountDiscriminator, Error, Label, SimpleTransaction, Vertex,
};

use uuid::Uuid;

impl_controller!(VertexController("vertices:v1"));

/// Not identify the datastore adapter for vertex controller will set
/// it default to RocksDBAdapter
impl Default for VertexController {
	fn default() -> Self {
		VertexController::new().unwrap()
	}
}

impl VertexController {
	/// # Create a new vertex from labels and properties
	pub async fn create_vertex(
		&self,
		labels: Vec<Label>,
		props: HashMap<Uuid, Vec<u8>>,
	) -> Result<Vertex, Error> {
		let uuid_len = Component::Uuid(Uuid::nil()).len();
		let ll = labels.len() as u8; // Label list length

		let mut tx = self.config.ds.transaction(true).unwrap();
		let cf = self.get_cf();

		let label_components =
			labels.iter().map(|l| Component::Uuid(l.id)).collect::<Vec<Component>>();
		let property_components =
			props.iter().map(|p| Component::Property(p.0, p.1)).collect::<Vec<Component>>();

		// Handle byte concatenate for label components
		let _label_discriminator = serialize_discriminator(AccountDiscriminator::Label).unwrap();
		let _labels = build_bytes(&label_components).unwrap();
		let _label_offset = &build_offset(ll, uuid_len);
		let (l_dis, l, l_offset) =
			(_label_discriminator.as_slice(), _label_offset.as_slice(), _labels.as_slice());
		let labels_concat = [l_dis, l, l_offset].concat();

		// Handle byte concatenate for property components
		let _properties = &build_bytes(&property_components).unwrap();
		let _property_discriminator =
			serialize_discriminator(AccountDiscriminator::Property).unwrap();
		let (p_dis, p) = (_property_discriminator.as_slice(), _properties.as_slice());
		let properties_concat = [p_dis, p].concat();

		let v = Vertex::new(labels.to_vec(), props).unwrap();
		let key = build_bytes(&[Component::Uuid(v.id)]).unwrap();

		let val = [labels_concat, properties_concat].concat();
		tx.set(cf, key, val).await.unwrap();
		tx.commit().await.unwrap();
		Ok(v)
	}

	pub async fn get_vertex(&self, id: Vec<u8>) -> Result<Vertex, Error> {
		let cf = self.get_cf();
		let tx = self.config.ds.transaction(false).unwrap();

		let value = tx.get(cf, id.to_vec()).await.unwrap();

		match value {
			Some(v) => {
				let uuid = from_uuid_bytes(&id).unwrap();
				let deserialized = deserialize_full_data(v.to_vec(), true).unwrap();
				let mut label_ids = vec![];

				for label in &deserialized[0] {
					label_ids.push(from_uuid_bytes(label).unwrap());
				}

				Ok(Vertex {
					id: uuid,
					labels: label_ids,
					props: HashMap::default(),
				})
			}
			None => panic!("No vertex found"),
		}
	}
}

// #[cfg(feature = "test-suite")]
#[cfg(test)]
#[tokio::test]
async fn should_create_label() {
	use crate::{LabelController, PropertyController, PropertyVariant};

	let vc = VertexController::default();
	let lc = LabelController::default();
	let pc = PropertyController::default();

	let raw_labels = ["Person", "Student", "Employee"];
	let properties = pc
		.create_properties(vec![
			("name", PropertyVariant::String),
			("age", PropertyVariant::UInt128),
		])
		.await
		.unwrap();
	let labels = lc.create_labels(raw_labels.to_vec()).await.unwrap();
	let res = vc
		.create_vertex(
			labels,
			HashMap::from([
				(properties[0].id, "example name".as_bytes().to_vec()),
				(properties[1].id, Vec::from([15])),
			]),
		)
		.await
		.unwrap();
	assert_eq!(res.labels.len(), raw_labels.len());

	let vertex = vc.get_vertex(res.id.as_bytes().to_vec()).await.unwrap();
	assert_eq!(vertex, res);
	assert_eq!(vertex.labels.len(), raw_labels.len());
}
