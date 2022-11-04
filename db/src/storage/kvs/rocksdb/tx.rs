use async_trait::async_trait;
use futures::lock::MutexGuard;

use super::ty::{DBType, TxType};
use crate::{
	err::Error,
	interface::kv::{Key, Val},
	model::tx::{DBTransaction, SimpleTransaction},
};

impl DBTransaction<DBType, TxType> {
	async fn get_guarded_tx(self: &Self) -> MutexGuard<Option<TxType>> {
		self.tx.lock().await
	}
}

#[async_trait]
impl SimpleTransaction for DBTransaction<DBType, TxType> {
	fn closed(&self) -> bool {
		self.ok
	}
	async fn cancel(&mut self) -> Result<(), Error> {
		if self.ok {
			return Err(Error::TxFinished);
		}

		// Mark this transaction as done
		self.ok = true;

		let mut tx = self.get_guarded_tx().await;
		match tx.take() {
			Some(tx) => tx.rollback()?,
			None => unreachable!(),
		}

		Ok(())
	}

	async fn commit(&mut self) -> Result<(), Error> {
		if self.closed() {
			return Err(Error::TxFinished);
		}

		// Check to see if transaction is writable
		if !self.writable {
			return Err(Error::TxReadonly);
		}

		// Mark this transaction as done
		self.ok = true;

		let mut tx = self.get_guarded_tx().await;
		match tx.take() {
			Some(tx) => tx.commit()?,
			None => unreachable!(),
		}

		Ok(())
	}

	async fn exi<K>(&mut self, key: K) -> Result<bool, Error>
	where
		K: Into<Key> + Send,
	{
		if self.closed() {
			return Err(Error::TxFinished);
		}

		Ok(!self.tx.lock().await.as_ref().unwrap().get(key.into())?.is_none())
	}
	// Fetch a key from the database
	async fn get<K>(&mut self, key: K) -> Result<Option<Val>, Error>
	where
		K: Into<Key> + Send,
	{
		if self.closed() {
			return Err(Error::TxFinished);
		}

		let tx = self.get_guarded_tx().await;
		Ok(tx.as_ref().unwrap().get(key.into()).unwrap())
	}
	// Insert or update a key in the database
	async fn set<K, V>(&mut self, key: K, val: V) -> Result<(), Error>
	where
		K: Into<Key> + Send,
		V: Into<Key> + Send,
	{
		if self.closed() {
			return Err(Error::TxFinished);
		}

		// Check to see if transaction is writable
		if !self.writable {
			return Err(Error::TxReadonly);
		}

		// Set the key
		let tx = self.get_guarded_tx().await;
		tx.as_ref().unwrap().put(key.into(), val.into())?;
		Ok(())
	}

	// Insert a key if it doesn't exist in the database
	async fn put<K, V>(&mut self, key: K, val: V) -> Result<(), Error>
	where
		K: Into<Key> + Send,
		V: Into<Key> + Send,
	{
		if self.closed() {
			return Err(Error::TxFinished);
		}

		// Check to see if transaction is writable
		if !self.writable {
			return Err(Error::TxReadonly);
		}

		// Future tx
		let guarded_tx = self.get_guarded_tx().await;
		let tx = guarded_tx.as_ref().unwrap();
		let (key, val) = (key.into(), val.into());

		match tx.get(&key)? {
			None => tx.put(key, val)?,
			_ => return Err(Error::TxConditionNotMet),
		};
		Ok(())
	}

	// Delete a key
	async fn del<K>(&mut self, key: K) -> Result<(), Error>
	where
		K: Into<Key> + Send,
	{
		if self.closed() {
			return Err(Error::TxFinished);
		}

		// Check to see if transaction is writable
		if !self.writable {
			return Err(Error::TxReadonly);
		}

		let key = key.into();
		let guarded_tx = self.get_guarded_tx().await;
		let tx = guarded_tx.as_ref().unwrap();

		match tx.get(&key)? {
			Some(_v) => tx.delete(key)?,
			None => return Err(Error::TxnKeyNotFound),
		};

		Ok(())
	}
}
