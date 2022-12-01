pub mod tx;
pub mod ty;

pub use tx::*;
pub use ty::*;

use crate::{
	constant::CF_NAMES,
	err::Error,
	model::{DBTransaction, DatastoreAdapter, StorageAdapter, StorageAdapterName},
	util::generate_path,
	StorageVariant,
};
use rocksdb::{DBCompactionStyle, OptimisticTransactionDB, Options};

#[derive(Debug)]
pub struct RocksDBAdapter(StorageAdapter<DBType>);

#[cfg(feature = "test-suite")]
crate::full_adapter_test_impl!(RocksDBAdapter::default());

impl RocksDBAdapter {
	impl_new_type_adapter!(DBType);

	// Path example: rocksdb://dev/smh/solomon-db
	pub fn new(path: &str, max_open_files: Option<i32>) -> Result<RocksDBAdapter, Error> {
		let path = &path["rocksdb:".len()..];
		let opts = get_options(max_open_files);
		let cf_names = CF_NAMES.iter();
		let db_instance = OptimisticTransactionDB::open_cf(&opts, path, cf_names)?;
		Ok(RocksDBAdapter(StorageAdapter::<DBType>::new(
			StorageAdapterName::RocksDB,
			path.to_string(),
			db_instance,
			StorageVariant::KeyValueStore,
		)?))
	}
}

impl DatastoreAdapter for RocksDBAdapter {
	type Transaction = RocksDBTransaction;

	fn default() -> Self {
		let path = &generate_path("rocksdb", None);
		RocksDBAdapter::new(path, None).unwrap()
	}

	fn spawn(&self) -> Self {
		RocksDBAdapter::default()
	}

	fn transaction(&self, rw: bool) -> Result<RocksDBTransaction, Error> {
		let inner = self.get_initialized_inner().unwrap();
		let db = &inner.db_instance;
		let tx = db.transaction();

		let tx = unsafe { extend_tx_lifetime(tx) };

		Ok(DBTransaction::<DBType, TxType>::new(tx, db.clone(), rw).unwrap())
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

pub fn get_options(max_open_files: Option<i32>) -> Options {
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
