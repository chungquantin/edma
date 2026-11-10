use async_trait::async_trait;
use leveldb::database::kv::KV;
use leveldb::database::iterator::Iterable;
use leveldb::options::{ReadOptions, WriteOptions};

use crate::{
	interface::{Key, KeyValuePair, Val},
	DBTransaction, Error, SimpleTransaction, TagBucket,
};

use super::ty::{DBType, TxType, LevelDBKey};

fn filter_with_prefix(
	iter: leveldb::database::iterator::Iterator<'_, LevelDBKey>,
	prefix: Vec<u8>,
) -> Vec<(LevelDBKey, Vec<u8>)> {
	iter.filter(|(k, _)| k.0.starts_with(&prefix)).collect()
}

fn filter_with_suffix(
	iter: leveldb::database::iterator::Iterator<'_, LevelDBKey>,
	suffix: Vec<u8>,
) -> Vec<(LevelDBKey, Vec<u8>)> {
	iter.filter(|(k, _)| k.0.ends_with(&suffix)).collect()
}

#[async_trait(?Send)]
impl SimpleTransaction for DBTransaction<DBType, TxType> {
	fn closed(&self) -> bool {
		self.ok
	}

	async fn count(&mut self, _tags: TagBucket) -> Result<usize, Error> {
		if self.closed() {
			return Err(Error::TxFinished);
		}

		let db = &self._db;
		let read_opts = ReadOptions::new();
		let iter = db.iter(read_opts);
		Ok(iter.count())
	}

	async fn cancel(&mut self) -> Result<(), Error> {
		if self.ok {
			return Err(Error::TxFinished);
		}

		self.ok = true;
		Ok(())
	}

	async fn commit(&mut self) -> Result<(), Error> {
		if self.closed() {
			return Err(Error::TxFinished);
		}

		if !self.writable {
			return Err(Error::TxReadonly);
		}

		self.ok = true;
		Ok(())
	}

	async fn exi<K>(&self, key: K, _tags: TagBucket) -> Result<bool, Error>
	where
		K: Into<Key> + Send,
	{
		if self.closed() {
			return Err(Error::TxFinished);
		}

		let db = &self._db;
		let read_opts = ReadOptions::new();
		let key: Key = key.into();
		let leveldb_key = LevelDBKey(key);
		let result = db.get(read_opts, &leveldb_key).map_err(|e| Error::Tx(e.to_string()))?;
		Ok(result.is_some())
	}

	async fn get<K>(&self, key: K, _tags: TagBucket) -> Result<Option<Val>, Error>
	where
		K: Into<Key> + Send,
	{
		if self.closed() {
			return Err(Error::TxFinished);
		}

		let db = &self._db;
		let read_opts = ReadOptions::new();
		let key: Key = key.into();
		let leveldb_key = LevelDBKey(key);
		let result = db.get(read_opts, &leveldb_key).map_err(|e| Error::Tx(e.to_string()))?;
		Ok(result)
	}

	async fn set<K, V>(&mut self, key: K, val: V, _tags: TagBucket) -> Result<(), Error>
	where
		K: Into<Key> + Send,
		V: Into<Key> + Send,
	{
		if self.closed() {
			return Err(Error::TxFinished);
		}

		if !self.writable {
			return Err(Error::TxReadonly);
		}

		let db = &self._db;
		let write_opts = WriteOptions::new();
		let key: Key = key.into();
		let val: Key = val.into();
		let leveldb_key = LevelDBKey(key);
		db.put(write_opts, &leveldb_key, &val)
			.map_err(|e| Error::Tx(e.to_string()))?;
		Ok(())
	}

	async fn put<K, V>(&mut self, key: K, val: V, _tags: TagBucket) -> Result<(), Error>
	where
		K: Into<Key> + Send,
		V: Into<Key> + Send,
	{
		if self.closed() {
			return Err(Error::TxFinished);
		}

		if !self.writable {
			return Err(Error::TxReadonly);
		}

		let db = &self._db;
		let read_opts = ReadOptions::new();
		let write_opts = WriteOptions::new();
		let key: Key = key.into();
		let val: Key = val.into();
		let leveldb_key = LevelDBKey(key.clone());

		let exists = db
			.get(read_opts, &leveldb_key)
			.map_err(|e| Error::Tx(e.to_string()))?;
		if exists.is_some() {
			return Err(Error::TxConditionNotMet);
		}

		db.put(write_opts, &leveldb_key, &val)
			.map_err(|e| Error::Tx(e.to_string()))?;
		Ok(())
	}

	async fn del<K>(&mut self, key: K, _tags: TagBucket) -> Result<(), Error>
	where
		K: Into<Key> + Send,
	{
		if self.closed() {
			return Err(Error::TxFinished);
		}

		if !self.writable {
			return Err(Error::TxReadonly);
		}

		let db = &self._db;
		let read_opts = ReadOptions::new();
		let write_opts = WriteOptions::new();
		let key: Key = key.into();
		let leveldb_key = LevelDBKey(key.clone());

		let exists = db
			.get(read_opts, &leveldb_key)
			.map_err(|e| Error::Tx(e.to_string()))?;
		if exists.is_none() {
			return Err(Error::TxnKeyNotFound);
		}

		db.delete(write_opts, &leveldb_key)
			.map_err(|e| Error::Tx(e.to_string()))?;
		Ok(())
	}

	async fn iterate(&self, _tags: TagBucket) -> Result<Vec<Result<KeyValuePair, Error>>, Error> {
		if self.closed() {
			return Err(Error::TxFinished);
		}

		let db = &self._db;
		let read_opts = ReadOptions::new();
		let iter = db.iter(read_opts);

		Ok(iter
			.map(|(k, v)| Ok((k.0, v)))
			.collect())
	}

	async fn prefix_iterate<P>(
		&self,
		prefix: P,
		_tags: TagBucket,
	) -> Result<Vec<Result<KeyValuePair, Error>>, Error>
	where
		P: Into<Key> + Send,
	{
		if self.closed() {
			return Err(Error::TxFinished);
		}

		let db = &self._db;
		let read_opts = ReadOptions::new();
		let iter = db.iter(read_opts);
		let prefix: Key = prefix.into();
		let filtered = filter_with_prefix(iter, prefix);

		Ok(filtered
			.into_iter()
			.map(|(k, v)| Ok((k.0, v)))
			.collect())
	}

	async fn suffix_iterate<S>(
		&self,
		suffix: S,
		_tags: TagBucket,
	) -> Result<Vec<Result<KeyValuePair, Error>>, Error>
	where
		S: Into<Key> + Send,
	{
		if self.closed() {
			return Err(Error::TxFinished);
		}

		let db = &self._db;
		let read_opts = ReadOptions::new();
		let iter = db.iter(read_opts);
		let suffix: Key = suffix.into();
		let filtered = filter_with_suffix(iter, suffix);

		Ok(filtered
			.into_iter()
			.map(|(k, v)| Ok((k.0, v)))
			.collect())
	}
}
