// use uuid::Uuid;

// use crate::util::{build_bytes, deserialize_data_with_meta, from_uuid_bytes, Component};
// use crate::{Error, Label, SimpleTransaction};

// impl_controller!(LabelRepository("labels:v1"));

// impl<'a> LabelRepository<'a> {
// 	pub fn key(&self, id: Uuid) -> Vec<u8> {
// 		build_bytes(&[Component::Uuid(id)]).unwrap()
// 	}

// 	/// # Create a new vertex from labels and properties
// 	pub async fn create(&self, name: &str) -> Result<Label, Error> {
// 		let label = Label::new(name).unwrap();

// 		let mut tx = self.get_ds().transaction(true).unwrap();
// 		let cf = self.get_cf();
// 		let key = self.key(label.id);
// 		let val = Label::serialize(&label).unwrap();

// 		tx.put(cf, key, val).await.unwrap();
// 		tx.commit().await.unwrap();
// 		Ok(label)
// 	}

// 	pub async fn delete_by_id(&self, id: Vec<u8>) -> Result<(), Error> {
// 		let mut tx = self.get_ds().transaction(true).unwrap();
// 		let cf = self.get_cf();
// 		tx.del(cf, id).await.unwrap();
// 		tx.commit().await
// 	}

// 	pub async fn multi_create(&self, names: Vec<&str>) -> Result<Vec<Label>, Error> {
// 		let mut tx = self.get_ds().transaction(true).unwrap();
// 		let mut result = Vec::<Label>::new();
// 		for name in names.iter() {
// 			let label = Label::new(name).unwrap();
// 			let cf = self.get_cf();
// 			let key = build_bytes(&[Component::Uuid(label.id)]).unwrap();
// 			let val = Label::serialize(&label).unwrap();

// 			tx.put(cf, key, val).await.unwrap();
// 			result.push(label);
// 		}
// 		tx.commit().await.unwrap();

// 		Ok(result)
// 	}

// 	pub async fn get(&self, id: Vec<u8>) -> Result<Label, Error> {
// 		let tx = self.get_ds().transaction(false).unwrap();

// 		let cf = self.get_cf();
// 		let raw = tx.get(cf, id.to_vec()).await.unwrap();
// 		match raw {
// 			Some(r) => {
// 				let (data, _, _) = deserialize_data_with_meta(r, true).unwrap();
// 				Ok(Label {
// 					id: from_uuid_bytes(&id).unwrap(),
// 					name: String::from_utf8(data[0].to_vec()).unwrap(),
// 				})
// 			}
// 			None => panic!("No label value"),
// 		}
// 	}
// }
