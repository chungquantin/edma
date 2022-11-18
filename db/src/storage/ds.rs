use log::info;

use crate::model::adapter::DatastoreAdapter;
use crate::{storage::LOG, CassandraDBAdapter, Error, RocksDBAdapter};

use super::Transaction;

#[derive(Copy, Clone)]
pub struct DBRef<'a> {
	pub db: &'a Datastore,
}

impl<'a> DBRef<'a> {
	pub fn new(db: &'a Datastore) -> Self {
		DBRef {
			db,
		}
	}
}

#[allow(clippy::large_enum_variant)]
pub enum Inner {
	#[cfg(feature = "kv-rocksdb")]
	RocksDB(RocksDBAdapter),
	#[cfg(feature = "kv-cassandradb")]
	CassandraDB(CassandraDBAdapter),
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
			s if s.starts_with("rocksdb:") | s.eq("default") => {
				info!(target: LOG, "Starting kvs store at {}", path);
				let db = RocksDBAdapter::new(s, None).unwrap();
				let v = Datastore {
					inner: Inner::RocksDB(db),
				};
				info!(target: LOG, "Started kvs store at {}", path);
				v
			}
			_ => unimplemented!(),
		}
	}

	pub fn borrow(&self) -> DBRef {
		DBRef::new(self)
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
			CassandraDB feat "kv-cassandradb"
		)
	}
}
