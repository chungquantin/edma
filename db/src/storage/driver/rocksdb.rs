use rocksdb::{ColumnFamily, DB};

use super::core::StorageDriver;
use crate::util::err::StorageErr;

pub struct RocksDBDriver(StorageDriver<DB>);

impl RocksDBDriver {
 super::core::impl_driver_core!(DB);

 /// # Initializing RocksDB driver
 /// ## Column family checks
 /// List existing ColumnFamilies in the given path. returns Err when no DB exists.
 /// If there is no Column Family specified, key-value pair is associated with Column Family "default".
 pub fn initialize(self: &mut Self, cf_name: &String, options: rocksdb::Options) {
  let core = self.get_core();
  let path = &core.path;
  let cfs = DB::list_cf(&options, path).unwrap_or(vec![]);
  let is_cf_initialized = !cfs.iter().find(|cf| cf == &cf_name).is_none();
  let mut instance = DB::open_cf(&options, path, cfs).unwrap();
  if is_cf_initialized {
   // create a new ColumnFamily
   let options = rocksdb::Options::default();
   instance.create_cf(cf_name, &options).unwrap();
  }

  core.db_instance = Some(instance);
 }

 /// # Get column family
 /// Each key-value pair in RocksDB is associated with exactly one Column Family.
 /// Can imagine column family as a container and key-value pairs is inside it
 pub fn get_cf(self: &mut Self, cf_name: &String) -> &ColumnFamily {
  let core = self.get_initialized_core();
  let instance = core.db_instance.as_ref().unwrap();
  let cf = instance.cf_handle(cf_name);
  cf.unwrap()
 }
}
