#![cfg(feature = "kv-rocksdb")]

use self::ty::{DBType, TxType};
use crate::{
	err::Error,
	model::{
		adapter::{StorageAdapter, StorageAdapterName},
		tx::DBTransaction,
	},
	util::status::StorageVariant,
};
use rocksdb::OptimisticTransactionDB;
pub struct Adapter(StorageAdapter<DBType>);

pub mod tx;
pub mod ty;

impl Adapter {
	super::super::rule::impl_new_type_adapter!(DBType);

	pub async fn new(path: &str) -> Result<Adapter, Error> {
		let db_instance = OptimisticTransactionDB::open_default(path)?;
		Ok(Adapter(StorageAdapter::<DBType>::new(
			StorageAdapterName::RocksDB,
			db_instance,
			StorageVariant::KeyValueStore,
		)?))
	}

	// # Create new database transaction
	// Set `rw` default to false means readable but not readable
	pub async fn transaction(&self, rw: bool) -> Result<DBTransaction<DBType, TxType>, Error> {
		let inner = self.get_initialized_inner();
		let db = &inner.db_instance;
		let tx = db.transaction();

		// The database reference must always outlive
		// the transaction. If it doesn't then this
		// is undefined behavior. This unsafe block
		// ensures that the transaction reference is
		// static, but will cause a crash if the
		// datastore is dropped prematurely.
		let tx = unsafe { self.extend_tx_lifetime(tx) };

		Ok(DBTransaction::<DBType, TxType>::new(tx, rw, db.clone())?)
	}

	unsafe fn extend_tx_lifetime(
		self: &Self,
		tx: rocksdb::Transaction<'_, OptimisticTransactionDB>,
	) -> rocksdb::Transaction<'static, OptimisticTransactionDB> {
		std::mem::transmute::<
			rocksdb::Transaction<'_, OptimisticTransactionDB>,
			rocksdb::Transaction<'static, OptimisticTransactionDB>,
		>(tx)
	}
}

#[cfg(test)]
mod tests {
	use std::str::from_utf8;

	use super::*;

	#[tokio::test]
	async fn test_raw_tx() {
		let adapter = Adapter::new(".temp/raw_rocks.db").await.unwrap();
		let db = &adapter.get_initialized_inner().db_instance;
		let tx = db.transaction();

		let key = "mock key";
		let val = "mock value";
		tx.put(key, val).unwrap();
		tx.commit().unwrap();

		let tx = db.transaction();
		let res = tx.get(key).unwrap();

		println!("{:?} {:?}", res, val);

		match res {
			Some(v) => assert_eq!(val, from_utf8(&v).unwrap()),
			None => panic!("Wrong value"),
		}
	}

	#[tokio::test]
	async fn test_adapter_tx() {
		let adapter = Adapter::new(".temp/rocks.db").await.unwrap();
		let mut tx = adapter.transaction(true).await.unwrap();

		let key = "mock key";
		let val = "mock value";

		tx.set(key, val).await.unwrap();
		let res = tx.get(key).await.unwrap();
		match res {
			Some(v) => assert_eq!(val, from_utf8(&v).unwrap()),
			None => panic!("Wrong value"),
		}
	}
}
