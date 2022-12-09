mod redb;
mod rocksdb;

pub const LOG: &str = "edma::kvs";

pub use self::redb::*;
pub use self::rocksdb::*;
