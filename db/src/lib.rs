mod err;
mod interface;
#[macro_use]
mod mac;
mod constant;
#[macro_use]
pub mod model;
mod storage;
mod util;

#[cfg(feature = "kv-redb")]
pub use crate::storage::kvs::ReDBAdapter;
#[cfg(feature = "kv-rocksdb")]
pub use crate::storage::kvs::RocksDBAdapter;
#[macro_use]
#[cfg(test)]
pub mod tests;

pub use crate::err::*;
pub use crate::interface::*;
pub use crate::model::*;
pub use crate::storage::{Datastore, DatastoreRef, Transaction};
