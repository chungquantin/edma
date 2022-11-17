use async_trait::async_trait;

use crate::{
	interface::{Key, Val},
	Error, SimpleTransaction, CF,
};

use super::{CassandraDBTransaction, RocksDBTransaction};

#[allow(clippy::large_enum_variant)]
pub(super) enum Inner {
	#[cfg(feature = "kv-rocksdb")]
	RocksDB(RocksDBTransaction),
	#[cfg(feature = "kv-cassandradb")]
	CassandraDB(CassandraDBTransaction),
}

pub struct Transaction {
	pub(super) inner: Inner,
}

impl_global_transaction!(RocksDB, CassandraDB);
