mod cassandradb;
mod rocksdb;

pub const LOG: &str = "solomondb::kvs";

pub use self::rocksdb::*;
pub use cassandradb::*;
