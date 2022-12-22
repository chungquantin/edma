#[cfg(feature = "kv-redb")]
mod redb;
#[cfg(feature = "kv-rocksdb")]
mod rocksdb;

pub const LOG: &str = "edma::kvs";

#[cfg(feature = "kv-redb")]
pub use self::redb::*;
#[cfg(feature = "kv-rocksdb")]
pub use self::rocksdb::*;
