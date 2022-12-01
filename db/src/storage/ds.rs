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
			s if s.starts_with("rocksdb:") | s.eq("default") => {
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
