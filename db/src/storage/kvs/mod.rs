mod redb;
mod rocksdb;

pub const LOG: &str = "solomondb::kvs";

pub use self::redb::*;
pub use self::rocksdb::*;
