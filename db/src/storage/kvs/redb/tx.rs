use async_trait::async_trait;
use redb::{RangeIter, ReadableTable, TableDefinition};

use crate::{
	interface::{Key, KeyValuePair, Val},
	DBTransaction, Error, SimpleTransaction, TagBucket, CF,
};

use super::ty::{DBType, TxType};

type TableKey = &'static [u8];
type TableValue = &'static [u8];

fn filter_with_prefix(
	iterator: RangeIter<TableKey, TableValue>,
	prefix: Vec<u8>,
) -> impl Iterator<Item = (&[u8], &[u8])> + '_ {
	iterator.filter(move |item| -> bool {
		let (k, _) = *item;
		k.starts_with(&prefix)
	})
}

fn filter_with_suffix(
	iterator: RangeIter<TableKey, TableValue>,
	suffix: Vec<u8>,
) -> impl Iterator<Item = (&[u8], &[u8])> + '_ {
	iterator.filter(move |item| -> bool {
		let (k, _) = *item;
		k.ends_with(&suffix)
	})
}

fn get_table_name(cf: CF) -> String {
	let default = "default".as_bytes().to_vec();
	String::from_utf8(cf.unwrap_or(default)).unwrap()
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
		let name = get_table_name(cf);
		let def = TableDefinition::<TableKey, TableValue>::new(&name);
		let table = &tx.open_table(def);

		match table {
			Ok(t) => Ok(t.len()?),
			Err(_) => Err(Error::DsNoColumnFamilyFound),
		}
	}

	async fn cancel(&mut self) -> Result<(), Error> {
		if self.ok {
			return Err(Error::TxFinished);
		}

		// Mark this transaction as done
		self.ok = true;

		let mut tx = self.tx.lock().await;
		match tx.take() {
			Some(tx) => tx.abort()?,
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

		let guarded_tx = self.tx.lock().await;
		let tx = guarded_tx.as_ref().unwrap();

		let cf = tags.get_bytes("column_family");
		let name = get_table_name(cf);
		let def = TableDefinition::<TableKey, TableValue>::new(&name);
		let table = &tx.open_table(def);

		let key = key.into();
		match table {
			Ok(t) => Ok(t.get(&key)?.is_some()),
			Err(_) => Err(Error::DsNoColumnFamilyFound),
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
		let name = get_table_name(cf);
		let def = TableDefinition::<TableKey, TableValue>::new(&name);
		let table = &tx.open_table(def).unwrap();

		let key = key.into();
		let result = table.get(&key).unwrap();
		Ok(result.map(|v| v.to_vec()))
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

		let guarded_tx = self.tx.lock().await;
		let tx = guarded_tx.as_ref().unwrap();

		let cf = tags.get_bytes("column_family");
		let name = get_table_name(cf);
		let def = TableDefinition::<TableKey, TableValue>::new(&name);
		let (key, val) = (key.into(), val.into());

		let mut table = tx.open_table(def);
		match table.as_mut() {
			Ok(t) => t.insert(&key, &val)?,
			Err(_) => return Err(Error::DsNoColumnFamilyFound),
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

		let guarded_tx = self.tx.lock().await;
		let tx = guarded_tx.as_ref().unwrap();

		let cf = tags.get_bytes("column_family");
		let name = get_table_name(cf);
		let def = TableDefinition::<TableKey, TableValue>::new(&name);
		let mut table = tx.open_table(def)?;

		let (key, val) = (key.into(), val.into());

		match table.get(&key)? {
			None => table.insert(&key, &val)?,
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

		let guarded_tx = self.tx.lock().await;
		let tx = guarded_tx.as_ref().unwrap();

		let cf = tags.get_bytes("column_family");
		let name = get_table_name(cf);
		let def = TableDefinition::<TableKey, TableValue>::new(&name);
		let mut table = tx.open_table(def);

		let key = key.into();

		match table.as_mut() {
			Ok(t) => t.remove(&key)?,
			Err(_) => return Err(Error::DsNoColumnFamilyFound),
		};

		Ok(())
	}

	async fn iterate(&self, tags: TagBucket) -> Result<Vec<Result<KeyValuePair, Error>>, Error> {
		if self.closed() {
			return Err(Error::TxFinished);
		}

		let guarded_tx = self.tx.lock().await;
		let tx = guarded_tx.as_ref().unwrap();

		let cf = tags.get_bytes("column_family");
		let name = get_table_name(cf);
		let def = TableDefinition::<TableKey, TableValue>::new(&name);
		let table = tx.open_table(def);

		let iterator = match table.as_ref() {
			Ok(t) => t.iter()?,
			Err(_) => return Err(Error::DsNoColumnFamilyFound),
		};

		Ok(iterator
			.map(|p| {
				let (k, v) = p;
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

		let guarded_tx = self.tx.lock().await;
		let tx = guarded_tx.as_ref().unwrap();

		let cf = tags.get_bytes("column_family");
		let name = get_table_name(cf);
		let def = TableDefinition::<TableKey, TableValue>::new(&name);
		let table = tx.open_table(def);

		let iterator = table.as_ref().unwrap().iter()?;

		let prefix: Key = prefix.into();
		let filtered_iterator = filter_with_prefix(iterator, prefix);

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

		let guarded_tx = self.tx.lock().await;
		let tx = guarded_tx.as_ref().unwrap();

		let cf = tags.get_bytes("column_family");
		let name = get_table_name(cf);
		let def = TableDefinition::<TableKey, TableValue>::new(&name);
		let table = tx.open_table(def);

		let iterator = table.as_ref().unwrap().iter()?;
		let suffix: Key = suffix.into();
		let filtered_iterator = filter_with_suffix(iterator, suffix);

		Ok(filtered_iterator
			.map(|pair| {
				let (k, v) = pair;
				Ok((k.to_vec(), v.to_vec()))
			})
			.collect())
	}
}
