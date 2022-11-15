pub mod cassandradb;
pub mod rocksdb;

pub const LOG: &str = "solomondb::kvs";

pub use self::cassandradb::CassandraDBAdapter;
pub use self::rocksdb::RocksDBAdapter;
