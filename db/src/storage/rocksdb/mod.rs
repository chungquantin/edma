pub mod tx;
pub mod ty;

use self::ty::{DBType, TxType};

use crate::{
    model::{
        adapter::{StorageAdapter, StorageAdapterName},
        transaction::Transaction,
    },
    util::status::StorageVariant,
};
use rocksdb::{Error, OptimisticTransactionDB};

pub struct RocksDBAdapter(StorageAdapter<DBType>);

impl<'a> RocksDBAdapter {
    super::rule::impl_new_type_adapter!(DBType);

    pub fn new(path: &str, options: rocksdb::Options) -> RocksDBAdapter {
        let db_instance = RocksDBAdapter::initialize(path, options);
        RocksDBAdapter(StorageAdapter::<DBType>::new(
            StorageAdapterName::RocksDB,
            db_instance,
            StorageVariant::KeyValueStore,
        ))
    }

    pub fn transaction(
        self: &'static Self,
        w: bool,
        r: bool,
    ) -> Result<Transaction<TxType>, Error> {
        let inner = self.get_inner();
        let db_instance = &inner.db_instance;
        let tx = db_instance.transaction();

        // The database reference must always outlive
        // the transaction. If it doesn't then this
        // is undefined behaviour. This unsafe block
        // ensures that the transaction reference is
        // static, but will cause a crash if the
        // datastore is dropped prematurely.
        let tx = unsafe {
            std::mem::transmute::<
                rocksdb::Transaction<'_, OptimisticTransactionDB>,
                rocksdb::Transaction<'static, OptimisticTransactionDB>,
            >(tx)
        };

        Ok(Transaction::<TxType>::new(tx, w, r))
    }

    /// # Initializing RocksDB driver
    /// ## Column family checks
    /// List existing ColumnFamilies in the given path. returns Err when no DB exists.
    /// If there is no Column Family specified, key-value pair is associated with Column Family "default".
    pub fn initialize(path: &str, options: rocksdb::Options) -> DBType {
        let instance = DBType::open(&options, path).unwrap();
        instance
    }
}
