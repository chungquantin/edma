pub mod cassandradb;
pub mod rocksdb;

pub const LOG: &str = "solomondb::kvs";

pub use self::cassandradb::*;
pub use self::rocksdb::*;
