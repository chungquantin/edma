mod err;
mod interface;
#[macro_use]
mod mac;
mod constant;
mod db;
mod model;
mod repo;
mod storage;
mod util;

#[cfg(feature = "kv-cassandradb")]
pub use crate::storage::kvs::CassandraDBAdapter;
#[cfg(feature = "kv-rocksdb")]
pub use crate::storage::kvs::RocksDBAdapter;
#[macro_use]
pub mod tests;

pub use crate::db::*;
use crate::err::*;
pub use crate::model::*;
pub use crate::repo::*;
