use crate::model::DatastoreAdapter;
use crate::Error;
use crate::Transaction;

#[cfg(feature = "kv-redb")]
use super::ReDBAdapter;

#[cfg(feature = "kv-rocksdb")]
use super::RocksDBAdapter;
use super::SledAdapter;

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
	#[cfg(feature = "kv-sled")]
	Sled(SledAdapter),
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
			s if s.starts_with("rocksdb:") => {
				let db = RocksDBAdapter::new(s, None).unwrap();

				Datastore {
					inner: Inner::RocksDB(db),
				}
			}
			#[cfg(feature = "kv-redb")]
			s if s.starts_with("redb:") => {
				let db = ReDBAdapter::new(s).unwrap();

				Datastore {
					inner: Inner::ReDB(db),
				}
			}
			#[cfg(feature = "kv-sled")]
			s if s.starts_with("sled:") => {
				let db = SledAdapter::new(s).unwrap();

				Datastore {
					inner: Inner::Sled(db),
				}
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
			ReDB feat "kv-redb",
			Sled feat "kv-sled"
		)
	}

	pub async fn transaction(&self, write: bool) -> Result<Transaction, Error> {
		macro_rules! impl_transaction_method {
			($($x: ident feat $f: expr),*) => {
				match &self.inner {
					$(
						#[cfg(feature = $f)]
						Inner::$x(v) => {
							let tx = v.transaction(write).await?;
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
			ReDB feat "kv-redb",
			Sled feat "kv-sled"
		)
	}
}

#[cfg(test)]
mod test {
	use crate::{
		constant::{ColumnFamily, KEYSPACES},
		tag, SimpleTransaction,
	};

	use super::Datastore;

	#[tokio::test]
	async fn should_rocksdb_create_with_cf() {
		let db = Datastore::new("rocksdb:../temp/cf");
		assert!(db.transaction(false).await.is_ok());

		// Seeding database
		let cf_name = KEYSPACES.get(&ColumnFamily::TestSuite).unwrap();
		let key1 = i32::to_be_bytes(2100);
		let key2 = "cf => hello world";
		let key3 = "cf => this is a key";

		let val1 = "cf => mock value";
		let val2 = "cf => mock value 2";
		let val3 = "cf => this is a new value";

		let mut tx = db.transaction(true).await.unwrap();
		let tags = tag!("column_family" => cf_name.clone());
		tx.set(key1, val1, tags.clone()).await.unwrap();
		tx.set(key2, val2, tags.clone()).await.unwrap();
		tx.set(key3, val3, tags.clone()).await.unwrap();
		let iter = tx.iterate(tags.clone()).await.unwrap();
		assert!(iter.len() == 3);
		tx.commit().await.unwrap();
	}

	#[tokio::test]
	async fn should_redb_create() {
		let db = Datastore::new("redb:../temp/redb");
		assert!(db.transaction(false).await.is_ok());

		let key1 = i32::to_be_bytes(2001);
		let key2 = "new key new data hehe";
		let key3 = "this is a key";

		let val1 = "mock value";
		let val2 = "mock value mock data hehe";
		let val3 = "this is a new value";

		let mut tx = db.transaction(true).await.unwrap();
		tx.set(key1, val1, tag!()).await.unwrap();
		tx.set(key2, val2, tag!()).await.unwrap();
		tx.set(key3, val3, tag!()).await.unwrap();
		let iter = tx.iterate(tag!()).await.unwrap();
		assert!(iter.len() == 3);
		tx.commit().await.unwrap();
	}

	#[tokio::test]
	async fn should_sled_create() {
		let db = Datastore::new("sled:../temp");
		assert!(db.transaction(false).await.is_ok());

		// Seeding database
		let key1 = i32::to_be_bytes(2100);
		let key2 = "hello world";
		let key3 = "this is a key";

		let val1 = "mock value";
		let val2 = "mock value 2";
		let val3 = "this is a new value";

		let mut tx = db.transaction(true).await.unwrap();
		tx.set(key1, val1, tag!()).await.unwrap();
		tx.set(key2, val2, tag!()).await.unwrap();
		tx.set(key3, val3, tag!()).await.unwrap();
		let iter = tx.iterate(tag!()).await.unwrap();
		assert!(iter.len() == 3);
		tx.commit().await.unwrap();
	}

	#[tokio::test]
	async fn should_sled_create_tree() {
		let db = Datastore::new("sled:../temp/cf");
		assert!(db.transaction(false).await.is_ok());

		// Seeding database
		let tree_name = KEYSPACES.get(&ColumnFamily::TestSuite).unwrap();
		let key1 = i32::to_be_bytes(2100);
		let key2 = "tree => hello world";
		let key3 = "tree => this is a key";

		let val1 = "tree => mock value";
		let val2 = "tree => mock value 2";
		let val3 = "tree => this is a new value";

		let mut tx = db.transaction(true).await.unwrap();
		let tags = tag!("tree" => tree_name.clone());
		tx.set(key1, val1, tags.clone()).await.unwrap();
		tx.set(key2, val2, tags.clone()).await.unwrap();
		tx.set(key3, val3, tags.clone()).await.unwrap();
		let iter = tx.iterate(tags.clone()).await.unwrap();
		assert!(iter.len() == 3);
		tx.commit().await.unwrap();
	}
}
