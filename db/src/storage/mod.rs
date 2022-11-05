/// Storage
pub mod kvs;
pub use kvs::*;

macro_rules! register_adapter {
 ($name:ident, $($x:ident),*) => {
   use $crate::model::adapter::DatastoreAdapter;
   pub enum DatastoreManager {
       $(
           $x,
       )*
   }

   impl DatastoreManager {
       pub fn $name(&self) -> $( $x)*
       {
           match self {
               $(
                  DatastoreManager::$x => $x::default(),
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
