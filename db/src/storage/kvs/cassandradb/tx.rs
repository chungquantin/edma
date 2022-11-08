use async_trait::async_trait;

use crate::{
	interface::{Key, Val},
	DBTransaction, Error, SimpleTransaction, CF,
};

use super::ty::{DBType, TxType};

#[async_trait]
impl SimpleTransaction for DBTransaction<DBType, TxType> {
	fn closed(&self) -> bool {
		unimplemented!()
	}
	async fn cancel(&mut self) -> Result<(), Error> {
		unimplemented!()
	}

	async fn commit(&mut self) -> Result<(), Error> {
		unimplemented!()
	}

	async fn exi<K>(&self, _cf: CF, _key: K) -> Result<bool, Error>
	where
		K: Into<Key> + Send,
	{
		unimplemented!()
	}
	// Fetch a key from the database [column family]
	async fn get<K>(&self, _cf: CF, _key: K) -> Result<Option<Val>, Error>
	where
		K: Into<Key> + Send,
	{
		unimplemented!()
	}
	// Insert or update a key in the database
	async fn set<K, V>(&mut self, _cf: CF, _key: K, _val: V) -> Result<(), Error>
	where
		K: Into<Key> + Send,
		V: Into<Key> + Send,
	{
		unimplemented!()
	}

	// Insert a key if it doesn't exist in the database
	async fn put<K, V>(&mut self, _cf: CF, _key: K, _val: V) -> Result<(), Error>
	where
		K: Into<Key> + Send,
		V: Into<Key> + Send,
	{
		unimplemented!()
	}

	// Delete a key
	async fn del<K>(&mut self, _cf: CF, _key: K) -> Result<(), Error>
	where
		K: Into<Key> + Send,
	{
		unimplemented!()
	}
}
