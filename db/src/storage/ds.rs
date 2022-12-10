use log::info;

use crate::model::DatastoreAdapter;
use crate::{storage::LOG, Error};

use super::{ReDBAdapter, RocksDBAdapter, Transaction};

#[derive(Copy, Clone)]
pub struct DatastoreRef<'a> {
	pub db: &'a Datastore,
}

impl<'a> DatastoreRef<'a> {
	pub fn new(db: &'a Datastore) -> Self {
		DatastoreRef {
			db,
		}
	}
}

#[allow(clippy::large_enum_variant)]
pub enum Inner {
	#[cfg(feature = "kv-rocksdb")]
	RocksDB(RocksDBAdapter),
	#[cfg(feature = "kv-redb")]
	ReDB(ReDBAdapter),
}

pub struct Datastore {
	pub inner: Inner,
}

impl Default for Datastore {
	fn default() -> Self {
		Datastore::new("default")
	}
}

impl Datastore {
	pub fn new(path: &str) -> Datastore {
		match path {
			#[cfg(feature = "kv-rocksdb")]
			s if s.starts_with("default:") | s.starts_with("rocksdb:") | s.eq("default") => {
				info!(target: LOG, "Starting RocksDB kvs store at {}", path);
				let db = RocksDBAdapter::new(s, None).unwrap();
				let v = Datastore {
					inner: Inner::RocksDB(db),
				};
				info!(target: LOG, "Started RocksDB kvs store at {}", path);
				v
			}
			#[cfg(feature = "kv-redb")]
			s if s.starts_with("redb:") => {
				info!(target: LOG, "Starting Redb kvs store at {}", path);
				let db = ReDBAdapter::new(s).unwrap();
				let v = Datastore {
					inner: Inner::ReDB(db),
				};
				info!(target: LOG, "Started Redb kvs store at {}", path);
				v
			}
			_ => unimplemented!(),
		}
	}

	pub fn borrow(&self) -> DatastoreRef {
		DatastoreRef::new(self)
	}

	pub fn path(&self) -> &str {
		macro_rules! impl_transaction_method {
			($($x: ident feat $f: expr),*) => {
				match &self.inner {
					$(
						#[cfg(feature = $f)]
						Inner::$x(v) => {
							v.path()
						}
					)*
				}
			};
		}
		impl_transaction_method!(
			RocksDB feat "kv-rocksdb",
			ReDB feat "kv-redb"
		)
	}

	pub fn transaction(&self, write: bool) -> Result<Transaction, Error> {
		macro_rules! impl_transaction_method {
			($($x: ident feat $f: expr),*) => {
				match &self.inner {
					$(
						#[cfg(feature = $f)]
						Inner::$x(v) => {
							let tx = v.transaction(write)?;
							Ok(Transaction {
								inner: super::tx::Inner::$x(tx),
							})
						}
					)*
				}
			};
		}
		impl_transaction_method!(
			RocksDB feat "kv-rocksdb",
			ReDB feat "kv-redb"
		)
	}
}

#[cfg(test)]
mod test {
	use crate::{
		constant::{ColumnFamily, COLUMN_FAMILIES},
		SimpleTransaction,
	};

	use super::Datastore;

	#[tokio::test]
	async fn should_create() {
		let db = Datastore::new("redb:../temp/redb");
		assert!(db.transaction(false).is_ok());

		// Seeding database
		let cf = None;

		let key1 = i32::to_be_bytes(2001);
		let key2 = "new key new data hehe";
		let key3 = "this is a key";

		let val1 = "mock value";
		let val2 = "mock value mock data hehe";
		let val3 = "this is a new value";

		let mut tx = db.transaction(true).unwrap();
		tx.set(cf.clone(), key1, val1).await.unwrap();
		tx.set(cf.clone(), key2, val2).await.unwrap();
		tx.set(cf.clone(), key3, val3).await.unwrap();
		tx.commit().await.unwrap();
	}

	#[tokio::test]
	async fn should_create_with_cf() {
		let db = Datastore::new("rocksdb:../temp/cf");
		assert!(db.transaction(false).is_ok());

		// Seeding database
		let cf_name = COLUMN_FAMILIES.get(&ColumnFamily::TestSuite).unwrap();
		let cf = Some(cf_name.to_string().into());

		let key1 = i32::to_be_bytes(2100);
		let key2 = "cf => hello world";
		let key3 = "cf => this is a key";

		let val1 = "cf => mock value";
		let val2 = "cf => mock value 2";
		let val3 = "cf => this is a new value";

		let mut tx = db.transaction(true).unwrap();
		tx.set(cf.clone(), key1, val1).await.unwrap();
		tx.set(cf.clone(), key2, val2).await.unwrap();
		tx.set(cf.clone(), key3, val3).await.unwrap();
		tx.commit().await.unwrap();
	}
}
