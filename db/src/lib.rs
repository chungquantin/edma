mod err;
mod interface;
#[macro_use]
mod mac;
mod constant;
mod model;
mod repo;
mod storage;
mod util;

#[cfg(feature = "kv-cassandradb")]
pub use crate::storage::kvs::CassandraDBAdapter;
#[cfg(feature = "kv-rocksdb")]
pub use crate::storage::kvs::RocksDBAdapter;
// #[cfg(feature = "test-suite")]
#[macro_use]
pub mod tests;

use crate::err::*;
pub use crate::model::*;
pub use crate::repo::*;
