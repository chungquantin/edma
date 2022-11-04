#![cfg(feature = "kv-rocksdb")]
mod cf;
mod tx;
mod ty;

use std::env::temp_dir;

use self::ty::{DBType, TxType};
use crate::{
	adapter::StorageVariant,
	err::Error,
	mac::adapter::impl_new_type_adapter,
	model::{DBTransaction, DatastoreAdapter, StorageAdapter, StorageAdapterName},
	util::{generate_random_i32, path_to_string},
};
use async_trait::async_trait;
use rocksdb::{DBCompactionStyle, OptimisticTransactionDB, Options};

pub struct RocksDBAdapter(StorageAdapter<DBType>);

#[cfg(feature = "test-suite")]
crate::full_test_impl!(RocksDBAdapter::default());

impl RocksDBAdapter {
	impl_new_type_adapter!(DBType);

	/// Generate a path to store data for RocksDB
	fn generate_path(id: Option<i32>) -> String {
		let random_id: i32 = generate_random_i32();
		let id = &id.unwrap_or(random_id).to_string();
		let path = if cfg!(target_os = "linux") {
			"/dev/shm/".into()
		} else {
			temp_dir()
		}
		.join(format!("solomon-rocksdb-{}", id));

		path_to_string(&path).unwrap()
	}

	pub fn new(path: &str, max_open_files: Option<i32>) -> Result<RocksDBAdapter, Error> {
		let opts = get_options(max_open_files);
		let db_instance = OptimisticTransactionDB::open_cf(&opts, path, &cf::CF_NAMES)?;
		Ok(RocksDBAdapter(StorageAdapter::<DBType>::new(
			StorageAdapterName::RocksDB,
			db_instance,
			StorageVariant::KeyValueStore,
		)?))
	}
}

#[async_trait]
impl DatastoreAdapter<DBTransaction<DBType, TxType>> for RocksDBAdapter {
	fn default() -> Self {
		let path = &RocksDBAdapter::generate_path(None);
		RocksDBAdapter::new(path, None).unwrap()
	}

	fn spawn(&self) -> Self {
		RocksDBAdapter::default()
	}

	fn transaction(&self, rw: bool) -> Result<DBTransaction<DBType, TxType>, Error> {
		let inner = self.get_initialized_inner().unwrap();
		let db = &inner.db_instance;
		let tx = db.transaction();

		let tx = unsafe { extend_tx_lifetime(tx) };

		Ok(DBTransaction::<DBType, TxType>::new(tx, rw, db.clone()).unwrap())
	}
}

// The database reference must always outlive the transaction. If it doesn't then this
// is undefined behavior. This unsafe block ensures that the transaction reference is
// static, but will cause a crash if the datastore is dropped prematurely.
unsafe fn extend_tx_lifetime(
	tx: rocksdb::Transaction<'_, OptimisticTransactionDB>,
) -> rocksdb::Transaction<'static, OptimisticTransactionDB> {
	std::mem::transmute::<
		rocksdb::Transaction<'_, OptimisticTransactionDB>,
		rocksdb::Transaction<'static, OptimisticTransactionDB>,
	>(tx)
}

fn get_options(max_open_files: Option<i32>) -> Options {
	// Current tuning based off of the total ordered example, flash
	// storage example on
	// https://github.com/facebook/rocksdb/wiki/RocksDB-Tuning-Guide
	let mut opts = Options::default();
	opts.create_if_missing(true);

	// If there is no column missing, create a new column family
	opts.create_missing_column_families(true);
	opts.set_compaction_style(DBCompactionStyle::Level);
	opts.set_write_buffer_size(67_108_864); // 64mb
	opts.set_max_write_buffer_number(3);
	opts.set_target_file_size_base(67_108_864); // 64mb
	opts.set_level_zero_file_num_compaction_trigger(8);
	opts.set_level_zero_slowdown_writes_trigger(17);
	opts.set_level_zero_stop_writes_trigger(24);
	opts.set_num_levels(4);
	opts.set_max_bytes_for_level_base(536_870_912); // 512mb
	opts.set_max_bytes_for_level_multiplier(8.0);

	if let Some(max_open_files) = max_open_files {
		opts.set_max_open_files(max_open_files);
	}

	opts
}
