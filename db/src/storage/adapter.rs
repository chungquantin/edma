use crate::{
 interface::misc::{Seq, Uint8Array},
 util::status::{OpsStatus, StorageVariant},
};

use super::driver::core::StorageDriver;

pub struct StorageAdapter<T> {
 pub name: String,
 pub driver: StorageDriver<T>,
 pub variant: StorageVariant,
}

pub trait Writable {
 fn write_one(self, key: String, value: Uint8Array) -> OpsStatus;
 fn write_multiple(self, keys: Seq<String>, values: Seq<Uint8Array>) -> OpsStatus;
}

pub trait Readable {
 fn get_one(key: String) -> OpsStatus;
 fn get_all(keys: Seq<String>) -> OpsStatus;
}
