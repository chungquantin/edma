pub struct StorageDriver<T> {
 pub db_instance: Option<T>,
 pub path: String,
}

macro_rules! impl_driver_core {
 ($DbType: ty) => {
  fn get_core(self: &mut Self) -> &mut StorageDriver<$DbType> {
   &mut self.0
  }
  fn get_initialized_core(self: &mut Self) -> &mut StorageDriver<$DbType> {
   let core = self.get_core();
   if core.db_instance.is_none() {
    panic!(
     "{}",
     StorageErr::DriverErr("Database instance is not initialized")
    );
   }
   core
  }
 };
}

pub(crate) use impl_driver_core;
