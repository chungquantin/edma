macro_rules! impl_new_type_adapter {
    ($DbType: ty) => {
        pub fn get_inner(self: &Self) -> &StorageAdapter<$DbType> {
            &self.0
        }

        pub fn get_initialized_inner(self: &Self) -> &StorageAdapter<$DbType> {
            use $crate::util::err::StorageErr;

            let core = self.get_inner();
            if Some(&core.db_instance).is_none() {
                panic!(
                    "{}",
                    StorageErr::DriverErr("Database instance is not initialized")
                );
            }
            core
        }

        pub fn get_mut_inner(self: &mut Self) -> &mut StorageAdapter<$DbType> {
            &mut self.0
        }

        pub fn get_mut_initialized_inner(self: &mut Self) -> &mut StorageAdapter<$DbType> {
            use $crate::util::err::StorageErr;

            let core = self.get_mut_inner();
            if Some(&core.db_instance).is_none() {
                panic!(
                    "{}",
                    StorageErr::DriverErr("Database instance is not initialized")
                );
            }
            core
        }
    };
}

pub(crate) use impl_new_type_adapter;
