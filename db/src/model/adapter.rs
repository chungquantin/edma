use std::{pin::Pin, sync::Arc};

use crate::{err::Error, util::status::StorageVariant};

pub enum StorageAdapterName {
	RocksDB,
}

pub struct StorageAdapter<T> {
	pub name: StorageAdapterName,
	pub db_instance: Pin<Arc<T>>,
	pub variant: StorageVariant,
}

impl<T> StorageAdapter<T> {
	pub fn new(
		name: StorageAdapterName,
		db_instance: T,
		variant: StorageVariant,
	) -> Result<Self, Error> {
		Ok(StorageAdapter {
			name,
			db_instance: Arc::pin(db_instance),
			variant,
		})
	}
}
