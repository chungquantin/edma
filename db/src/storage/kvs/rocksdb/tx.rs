use std::sync::Arc;

use async_trait::async_trait;
use rocksdb::{BoundColumnFamily, DBAccess, DBIteratorWithThreadMode, IteratorMode};

use super::ty::{DBType, TxType};
use crate::{
	err::Error,
	interface::{
		kv::{Key, Val},
		KeyValuePair,
	},
	model::{DBTransaction, SimpleTransaction},
	TagBucket, CF,
};

fn take_with_prefix<T: DBAccess>(
	iterator: DBIteratorWithThreadMode<T>,
	prefix: Vec<u8>,
) -> impl Iterator<Item = Result<(Box<[u8]>, Box<[u8]>), rocksdb::Error>> + '_ {
	iterator.take_while(move |item| -> bool {
		if let Ok((ref k, _)) = *item {
			k.starts_with(&prefix)
		} else {
			true
		}
	})
}

fn take_with_suffix<T: DBAccess>(
	iterator: DBIteratorWithThreadMode<T>,
	suffix: Vec<u8>,
) -> impl Iterator<Item = Result<(Box<[u8]>, Box<[u8]>), rocksdb::Error>> + '_ {
	iterator.take_while(move |item| -> bool {
		if let Ok((ref k, _)) = *item {
			k.ends_with(&suffix)
		} else {
			true
		}
	})
}

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

#[async_trait(?Send)]
impl SimpleTransaction for DBTransaction<DBType, TxType> {
	fn closed(&self) -> bool {
		self.ok
	}

	async fn count(&mut self, tags: TagBucket) -> Result<usize, Error> {
		if self.closed() {
			return Err(Error::TxFinished);
		}

		let guarded_tx = self.tx.lock().await;
		let tx = guarded_tx.as_ref().unwrap();
		let cf = tags.get_bytes("column_family");
		let cf = &self.get_column_family(cf).unwrap();
		Ok(tx.iterator_cf(cf, IteratorMode::Start).count())
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

	async fn exi<K>(&self, key: K, tags: TagBucket) -> Result<bool, Error>
	where
		K: Into<Key> + Send,
	{
		if self.closed() {
			return Err(Error::TxFinished);
		}

		let tx = self.tx.lock().await;
		let cf = tags.get_bytes("column_family");
		match cf {
			Some(_) => {
				let cf = &self.get_column_family(cf).unwrap();
				let result = tx.as_ref().unwrap().get_cf(cf, key.into()).unwrap().is_some();
				Ok(result)
			}
			None => {
				let result = tx.as_ref().unwrap().get(key.into()).unwrap().is_some();
				Ok(result)
			}
		}
	}
	// Fetch a key from the database [column family]
	async fn get<K>(&self, key: K, tags: TagBucket) -> Result<Option<Val>, Error>
	where
		K: Into<Key> + Send,
	{
		if self.closed() {
			return Err(Error::TxFinished);
		}

		let guarded_tx = self.tx.lock().await;
		let tx = guarded_tx.as_ref().unwrap();
		let cf = tags.get_bytes("column_family");
		Ok(match cf {
			Some(_) => {
				let cf = &self.get_column_family(cf).unwrap();
				tx.get_cf(cf, key.into()).unwrap()
			}
			None => tx.get(key.into()).unwrap(),
		})
	}

	// Insert or update a key in the database
	async fn set<K, V>(&mut self, key: K, val: V, tags: TagBucket) -> Result<(), Error>
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
		let guarded_tx = self.tx.lock().await;
		let tx = guarded_tx.as_ref().unwrap();
		let cf = tags.get_bytes("column_family");
		match cf {
			Some(_) => {
				let cf = &self.get_column_family(cf).unwrap();
				tx.put_cf(cf, key.into(), val.into())?;
			}
			None => tx.put(key.into(), val.into())?,
		};
		Ok(())
	}

	// Insert a key if it doesn't exist in the database
	async fn put<K, V>(&mut self, key: K, val: V, tags: TagBucket) -> Result<(), Error>
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
		let cf = tags.get_bytes("column_family");
		match cf {
			Some(_) => {
				let cf = &self.get_column_family(cf).unwrap();
				match tx.get_cf(cf, &key)? {
					None => tx.put_cf(cf, key, val)?,
					_ => return Err(Error::TxConditionNotMet),
				};
			}
			None => {
				match tx.get(&key)? {
					None => tx.put(key, val)?,
					_ => return Err(Error::TxConditionNotMet),
				};
			}
		};

		Ok(())
	}

	// Delete a key
	async fn del<K>(&mut self, key: K, tags: TagBucket) -> Result<(), Error>
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
		let cf = tags.get_bytes("column_family");
		let cf = &self.get_column_family(cf).unwrap();
		match tx.get_cf(cf, &key)? {
			Some(_v) => tx.delete_cf(cf, key)?,
			None => return Err(Error::TxnKeyNotFound),
		};

		Ok(())
	}

	// Iterate key value elements with handler
	async fn iterate(&self, tags: TagBucket) -> Result<Vec<Result<KeyValuePair, Error>>, Error> {
		if self.closed() {
			return Err(Error::TxFinished);
		}

		let guarded_tx = self.tx.lock().await;
		let tx = guarded_tx.as_ref().unwrap();

		let cf = tags.get_bytes("column_family");
		let get_iterator = match cf {
			Some(_) => {
				let get_cf = self.get_column_family(cf);
				match get_cf {
					Ok(cf) => Ok(tx.iterator_cf(&cf, IteratorMode::Start)),
					Err(err) => Err(err),
				}
			}
			None => Ok(tx.iterator(IteratorMode::Start)),
		};

		match get_iterator {
			Ok(iterator) => Ok(iterator
				.map(|pair| {
					let (k, v) = pair.unwrap();
					Ok((k.to_vec(), v.to_vec()))
				})
				.collect()),
			Err(err) => Err(err),
		}
	}

	async fn suffix_iterate<S>(
		&self,
		suffix: S,
		tags: TagBucket,
	) -> Result<Vec<Result<KeyValuePair, Error>>, Error>
	where
		S: Into<Key> + Send,
	{
		if self.closed() {
			return Err(Error::TxFinished);
		}

		let guarded_tx = self.tx.lock().await;
		let tx = guarded_tx.as_ref().unwrap();
		let suffix: Key = suffix.into();
		let cf = tags.get_bytes("column_family");
		let iterator = match cf {
			Some(_) => {
				let cf = &self.get_column_family(cf).unwrap();
				tx.iterator_cf(cf, IteratorMode::Start)
			}
			None => tx.iterator(IteratorMode::Start),
		};
		let taken_iterator = take_with_suffix(iterator, suffix);

		Ok(taken_iterator
			.map(|pair| {
				let (k, v) = pair.unwrap();
				Ok((k.to_vec(), v.to_vec()))
			})
			.collect())
	}

	// Iterate key value elements with handler
	async fn prefix_iterate<P>(
		&self,
		prefix: P,
		tags: TagBucket,
	) -> Result<Vec<Result<KeyValuePair, Error>>, Error>
	where
		P: Into<Key> + Send,
	{
		if self.closed() {
			return Err(Error::TxFinished);
		}

		let guarded_tx = self.tx.lock().await;
		let tx = guarded_tx.as_ref().unwrap();
		let prefix: Key = prefix.into();
		let cf = tags.get_bytes("column_family");
		let iterator = match cf {
			Some(_) => {
				let cf = &self.get_column_family(cf).unwrap();
				tx.iterator_cf(cf, IteratorMode::Start)
			}
			None => tx.iterator(IteratorMode::Start),
		};
		let taken_iterator = take_with_prefix(iterator, prefix);

		Ok(taken_iterator
			.map(|v| {
				let (k, v) = v.unwrap();
				Ok((k.to_vec(), v.to_vec()))
			})
			.collect())
	}
}
