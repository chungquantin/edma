use std::{pin::Pin, sync::Arc};

use rocksdb::Error;

use crate::{
	interface::{kv::Val, misc::Uint8Array},
	util::status::StorageVariant,
};

pub enum StorageAdapterName {
	RocksDB,
}

pub struct StorageAdapter<T> {
	pub name: StorageAdapterName,
	pub db_instance: Pin<Arc<T>>,
	pub variant: StorageVariant,
}

impl<T> StorageAdapter<T> {
	pub fn new(name: StorageAdapterName, db_instance: T, variant: StorageVariant) -> Self {
		StorageAdapter {
			name,
			db_instance: Arc::pin(db_instance),
			variant,
		}
	}
}

pub trait WritableAdapter {
	fn write_one(&mut self, key: String, value: Uint8Array) -> Result<(), Error>;
	fn write_multiple(&mut self, keys: Vec<String>, values: Vec<Uint8Array>) -> Result<(), Error>;
}

pub trait ReadableAdapter {
	fn get_one<T, K>(&self, tx: T, key: K) -> Result<Option<Val>, Error>;
	fn get_all(&self, keys: Vec<String>) -> Result<Vec<Option<Val>>, Error>;
}
