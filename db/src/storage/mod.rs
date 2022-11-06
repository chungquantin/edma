/// Storage
pub mod kvs;
pub use kvs::*;

macro_rules! register_adapter {
 ($name:ident, $($x:ident),*) => {
   use $crate::model::adapter::DatastoreAdapter;
   /// # Datastore Manager
   /// A generated enumeration Datastore Manager to dynamically register
   /// and return the datastore adapter without being constrained by the
   /// type system.
   #[allow(dead_code)]
   pub enum DatastoreManager {
    $($x)*
   }

   #[allow(dead_code)]
   impl DatastoreManager {
    pub fn $name(&self) -> $($x)*
    {
        match self {
            $(
                DatastoreManager::$x => $x::$name(),
            )*
        }
    }
   }
  };
}

#[macro_export]
macro_rules! full_adapter_impl {
	() => {
		register_adapter!(default, RocksDBAdapter);
	};
}

full_adapter_impl!();
