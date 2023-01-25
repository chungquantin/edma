use async_trait::async_trait;
use sled::{IVec, Iter};

use crate::{
	interface::{Key, KeyValuePair, Val},
	DBTransaction, Error, SimpleTransaction, TagBucket,
};

use super::ty::{DBType, TxType};

fn filter_with_prefix(iterator: Iter, prefix: Vec<u8>) -> impl Iterator<Item = (IVec, IVec)> {
	iterator
		.filter(move |item| -> bool {
			if let Ok((k, _)) = item.clone() {
				return k.starts_with(&prefix);
			}
			false
		})
		.map(move |item| item.unwrap())
}

fn filter_with_suffix(iterator: Iter, prefix: Vec<u8>) -> impl Iterator<Item = (IVec, IVec)> {
	iterator
		.filter(move |item| -> bool {
			if let Ok((k, _)) = item.clone() {
				return k.ends_with(&prefix);
			};
			false
		})
		.map(move |item| item.unwrap())
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

		let db = &self._db;
		let tree_name = tags.get("tree");
		let result = if let Some(t) = tree_name {
			let tree = db.open_tree(t).unwrap();
			tree.len()
		} else {
			db.len()
		};
		Ok(result)
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

		// Check to see if transaction is writable
		if !self.writable {
			return Err(Error::TxReadonly);
		}

		// Mark this transaction as done
		self.ok = true;

		Ok(())
	}

	async fn exi<K>(&self, key: K, tags: TagBucket) -> Result<bool, Error>
	where
		K: Into<Key> + Send,
	{
		if self.closed() {
			return Err(Error::TxFinished);
		}

		let db = &self._db;
		let tree_name = tags.get("tree");
		let key = key.into();
		let k = key.as_slice();
		let result = if let Some(t) = tree_name {
			let tree = db.open_tree(t).unwrap();
			tree.contains_key(k)
		} else {
			db.contains_key(k)
		};
		Ok(result.unwrap())
	}
	// Fetch a key from the database [column family]
	async fn get<K>(&self, key: K, tags: TagBucket) -> Result<Option<Val>, Error>
	where
		K: Into<Key> + Send,
	{
		if self.closed() {
			return Err(Error::TxFinished);
		}

		let db = &self._db;
		let tree_name = tags.get("tree");
		let key = key.into();
		let k = key.as_slice();
		let result = if let Some(t) = tree_name {
			let tree = db.open_tree(t).unwrap();
			tree.get(k)
		} else {
			db.get(k)
		};
		Ok(result.unwrap().map(|v| v.to_vec()))
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

		if !self.writable {
			return Err(Error::TxReadonly);
		}

		let db = &self._db;
		let tree_name = tags.get("tree");
		let key = key.into();
		let k = key.as_slice();
		let v = val.into();
		if let Some(t) = tree_name {
			let tree = db.open_tree(t).unwrap();
			tree.insert(k, v).unwrap()
		} else {
			db.insert(k, v).unwrap()
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

		let db = &self._db;
		let tree_name = tags.get("tree");
		let key = key.into();
		let k = key.as_slice();
		let v = val.into();
		let is_exists = if let Some(t) = tree_name.clone() {
			let tree = db.open_tree(t).unwrap();
			tree.contains_key(k)
		} else {
			db.contains_key(k)
		}
		.unwrap();

		match is_exists {
			false => if let Some(t) = tree_name {
				let tree = db.open_tree(t).unwrap();
				tree.insert(k, v)
			} else {
				db.insert(k, v)
			}
			.unwrap(),
			_ => return Err(Error::TxConditionNotMet),
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

		let db = &self._db;
		let tree_name = tags.get("tree");
		let key = key.into();
		let k = key.as_slice();
		if let Some(t) = tree_name {
			let tree = db.open_tree(t).unwrap();
			tree.remove(k)
		} else {
			db.remove(k)
		}
		.unwrap();

		Ok(())
	}

	async fn iterate(&self, tags: TagBucket) -> Result<Vec<Result<KeyValuePair, Error>>, Error> {
		if self.closed() {
			return Err(Error::TxFinished);
		}

		let db = &self._db;
		let tree_name = tags.get("tree");
		let iter = if let Some(t) = tree_name {
			let tree = db.open_tree(t).unwrap();
			tree.iter()
		} else {
			db.iter()
		};

		Ok(iter
			.map(|p| {
				let (k, v) = p.unwrap();
				Ok((k.to_vec(), v.to_vec()))
			})
			.collect())
	}

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

		let db = &self._db;
		let tree_name = tags.get("tree");
		let iter = if let Some(t) = tree_name {
			let tree = db.open_tree(t).unwrap();
			tree.iter()
		} else {
			db.iter()
		};

		let prefix: Key = prefix.into();
		let filtered_iterator = filter_with_prefix(iter, prefix);

		Ok(filtered_iterator
			.map(|pair| {
				let (k, v) = pair;
				Ok((k.to_vec(), v.to_vec()))
			})
			.collect())
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

		let db = &self._db;
		let tree_name = tags.get("tree");
		let iter = if let Some(t) = tree_name {
			let tree = db.open_tree(t).unwrap();
			tree.iter()
		} else {
			db.iter()
		};

		let suffix: Key = suffix.into();
		let filtered_iterator = filter_with_suffix(iter, suffix);

		Ok(filtered_iterator
			.map(|pair| {
				let (k, v) = pair;
				Ok((k.to_vec(), v.to_vec()))
			})
			.collect())
	}
}
