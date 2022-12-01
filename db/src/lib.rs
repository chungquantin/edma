mod err;
mod interface;
#[macro_use]
mod mac;
mod constant;
mod model;
mod process;
mod repo;
mod storage;
mod util;

#[cfg(feature = "kv-redb")]
pub use crate::storage::kvs::ReDBAdapter;
#[cfg(feature = "kv-rocksdb")]
pub use crate::storage::kvs::RocksDBAdapter;
#[macro_use]
#[cfg(test)]
pub mod tests;

use crate::err::*;

pub use crate::model::*;
pub use crate::process::Database;
pub use crate::repo::*;
