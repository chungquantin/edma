use crate::{
	err::Error,
	interface::kv::{Key, Val},
	model::transaction::Transaction,
};

use super::ty::TxType;

impl Transaction<TxType> {
	// Check if closed
	pub fn closed(&self) -> bool {
		self.ok
	}
	// Cancel a transaction
	pub async fn cancel(&mut self) -> Result<(), Error> {
		if self.ok {
			return Err(Error::TxFinished);
		}

		match self.inner.lock().unwrap().take() {
			Some(tx) => tx.rollback()?,
			None => unreachable!(),
		}

		Ok(())
	}
	// Commit a transaction
	pub async fn commit(&mut self) -> Result<(), Error> {
		match self.inner.lock().unwrap().take() {
			Some(tx) => tx.commit()?,
			None => unreachable!(),
		}
		Ok(())
	}
	// Check if a key exists
	pub async fn exi<K>(&mut self, key: K) -> Result<bool, Error>
	where
		K: Into<Key>,
	{
		match self.inner.lock().unwrap().take() {
			Some(tx) => Ok(tx.get(key.into())?.is_none()),
			None => unreachable!(),
		}
	}
	// Fetch a key from the database
	pub async fn get<K>(&mut self, key: K) -> Result<Option<Val>, Error>
	where
		K: Into<Key>,
	{
		match self.inner.lock().unwrap().take() {
			Some(tx) => Ok(tx.get(key.into()).unwrap()),
			None => unreachable!(),
		}
	}
	// Insert or update a key in the database
	pub async fn set<K, V>(&mut self, key: K, val: V) -> Result<(), Error>
	where
		K: Into<Key>,
		V: Into<Val>,
	{
		match self.inner.lock().unwrap().take() {
			Some(tx) => {
				tx.put(key.into(), val.into()).unwrap();
				Ok(())
			}
			None => unreachable!(),
		}
	}
	// Insert a key if it doesn't exist in the database
	pub async fn put<K, V>(&mut self, key: K, val: V) -> Result<(), Error>
	where
		K: Into<Key>,
		V: Into<Val>,
	{
		let tx = self.inner.lock().unwrap().take().unwrap();
		let (key, val) = (key.into(), val.into());
		match tx.get(&key)? {
			None => tx.put(key, val)?,
			_ => unreachable!(),
		};
		Ok(())
	}

	// // Delete a key
	// pub async fn del<K>(&mut self, key: K) -> Result<(), Error>
	// where
	//     K: Into<Key>,
	// {
	//     match self.inner.lock().unwrap().take() {
	//         Some(tx) => tx.delete(key.into()),
	//         None => unreachable!(),
	//     };

	//     Ok(())
	// }
}
