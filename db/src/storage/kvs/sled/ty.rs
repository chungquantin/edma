use std::marker::PhantomData;

use crate::DBTransaction;

/// OptimisticTransactionDB
/// Using OptimisticTransactionDB type instead of default DB type
/// This is for multithreaded concurrency control used in distributed system
pub type DBType = sled::Db;
pub type TxType = PhantomData<String>;
pub type SledTransaction = DBTransaction<DBType, TxType>;
