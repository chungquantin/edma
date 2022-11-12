use std::sync::Arc;

use async_trait::async_trait;
use rocksdb::BoundColumnFamily;

use super::ty::{DBType, TxType};
use crate::{
	err::Error,
	interface::kv::{Key, Val},
	model::tx::{DBTransaction, SimpleTransaction},
	CF,
};

impl DBTransaction<DBType, TxType> {
	fn get_column_family(&self, cf: CF) -> Result<Arc<BoundColumnFamily>, Error> {
		if cf.is_none() {
			return Err(Error::DsColumnFamilyIsNotValid);
		}
		let cf_name = String::from_utf8(cf.unwrap()).unwrap();
		let bounded_cf = self._db.cf_handle(&cf_name);

		match bounded_cf {
			Some(cf) => Ok(cf),
			_ => Err(Error::DsNoColumnFamilyFound),
		}
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

		let mut tx = self.tx.lock().await;
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

		let mut tx = self.tx.lock().await;
		match tx.take() {
			Some(tx) => tx.commit()?,
			None => unreachable!(),
		}

		Ok(())
	}

	async fn exi<K>(&self, cf: CF, key: K) -> Result<bool, Error>
	where
		K: Into<Key> + Send,
	{
		if self.closed() {
			return Err(Error::TxFinished);
		}

		let tx = self.tx.lock().await;
		let cf = &self.get_column_family(cf).unwrap();
		let result = tx.as_ref().unwrap().get_cf(cf, key.into()).unwrap().is_some();

		Ok(result)
	}
	// Fetch a key from the database [column family]
	async fn get<K>(&self, cf: CF, key: K) -> Result<Option<Val>, Error>
	where
		K: Into<Key> + Send,
	{
		if self.closed() {
			return Err(Error::TxFinished);
		}

		let tx = self.tx.lock().await;
		let cf = &self.get_column_family(cf).unwrap();
		Ok(tx.as_ref().unwrap().get_cf(cf, key.into()).unwrap())
	}

	async fn multi_get<K>(&self, cf: CF, keys: Vec<K>) -> Result<Vec<Option<Val>>, Error>
	where
		K: Into<Key> + Send + AsRef<[u8]>,
	{
		if self.closed() {
			return Err(Error::TxFinished);
		}

		let tx = self.tx.lock().await;
		let mut values = vec![];
		let cf = &self.get_column_family(cf).unwrap();
		for key in keys.iter() {
			let value = tx.as_ref().unwrap().get_cf(cf, key).unwrap();
			values.push(value);
		}
		Ok(values)
	}
	// Insert or update a key in the database
	async fn set<K, V>(&mut self, cf: CF, key: K, val: V) -> Result<(), Error>
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
		let tx = self.tx.lock().await;
		let cf = &self.get_column_family(cf).unwrap();
		tx.as_ref().unwrap().put_cf(cf, key.into(), val.into())?;
		Ok(())
	}

	// Insert a key if it doesn't exist in the database
	async fn put<K, V>(&mut self, cf: CF, key: K, val: V) -> Result<(), Error>
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
		let guarded_tx = self.tx.lock().await;
		let tx = guarded_tx.as_ref().unwrap();
		let (key, val) = (key.into(), val.into());

		let cf = &self.get_column_family(cf).unwrap();
		match tx.get_cf(cf, &key)? {
			None => tx.put_cf(cf, key, val)?,
			_ => return Err(Error::TxConditionNotMet),
		};
		Ok(())
	}

	// Delete a key
	async fn del<K>(&mut self, cf: CF, key: K) -> Result<(), Error>
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
		let guarded_tx = self.tx.lock().await;
		let tx = guarded_tx.as_ref().unwrap();

		let cf = &self.get_column_family(cf).unwrap();
		match tx.get_cf(cf, &key)? {
			Some(_v) => tx.delete_cf(cf, key)?,
			None => return Err(Error::TxnKeyNotFound),
		};

		Ok(())
	}
}
