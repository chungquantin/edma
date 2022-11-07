/// Storage
pub mod kvs;
pub use kvs::*;

macro_rules! register_adapter {
	(register $name:ident; for $x:ident) => {
		#[allow(dead_code)]
		impl DatastoreManager {
			pub fn $name(&self) -> $x {
				$x::default()
			}
		}
	};

	(register $name:ident; for $x:ident, $(register $names:ident; for $xs:ident),+) => {
		register_adapter! { register $name; for $x }
		register_adapter! { $(register $names; for $xs),+ }
	};
}

#[macro_export]
macro_rules! full_adapter_impl {
	() => {
		use $crate::model::adapter::DatastoreAdapter;
		/// # Datastore Manager
		/// A generated enumeration Datastore Manager to dynamically register
		/// and return the datastore adapter without being constrained by the
		/// type system.
		#[allow(dead_code)]
		pub enum DatastoreManager {
			RocksDBAdapter,
			CassandraDBAdapter,
		}

		register_adapter! {
			register rocks_db; for RocksDBAdapter,
			register cassandra_db; for CassandraDBAdapter
		}
	};
}

full_adapter_impl!();
