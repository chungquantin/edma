use crate::TagBucket;
use async_trait::async_trait;

use crate::{
	interface::{Key, Val},
	Error, SimpleTransaction,
};

#[cfg(feature = "kv-redb")]
use super::ReDBTransaction;

#[cfg(feature = "kv-rocksdb")]
use super::RocksDBTransaction;

#[allow(clippy::large_enum_variant)]
pub(super) enum Inner {
	#[cfg(feature = "kv-rocksdb")]
	RocksDB(RocksDBTransaction),
	#[cfg(feature = "kv-redb")]
	ReDB(ReDBTransaction),
}

pub struct Transaction {
	pub(super) inner: Inner,
}

impl_global_transaction!(
	RocksDB; feat "kv-rocksdb",
	ReDB; feat "kv-redb"
);
