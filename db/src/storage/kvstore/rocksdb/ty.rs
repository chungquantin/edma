use rocksdb::OptimisticTransactionDB;

/// OptimisticTransactionDB
/// Using OptimisticTransactionDB type instead of default DB type
/// This is for multithreaded concurrency control used in distributed system
pub type DBType = OptimisticTransactionDB;
pub type TxType = rocksdb::Transaction<'static, DBType>;
