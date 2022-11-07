macro_rules! impl_new_type_adapter {
	($DbType: ty) => {
		#[allow(dead_code)]
		pub fn get_inner(self: &Self) -> &StorageAdapter<$DbType> {
			&self.0
		}

		#[allow(dead_code)]
		pub fn get_initialized_inner(self: &Self) -> Result<&StorageAdapter<$DbType>, Error> {
			let core = self.get_inner();
			if Some(&core.db_instance).is_none() {
				return Err(Error::DbNotInitialized);
			}
			Ok(core)
		}

		#[allow(dead_code)]
		pub fn get_mut_inner(self: &mut Self) -> &mut StorageAdapter<$DbType> {
			&mut self.0
		}

		#[allow(dead_code)]
		pub fn get_mut_initialized_inner(
			self: &mut Self,
		) -> Result<&mut StorageAdapter<$DbType>, Error> {
			let core = self.get_mut_inner();
			if Some(&core.db_instance).is_none() {
				return Err(Error::DbNotInitialized);
			}
			Ok(core)
		}
	};
}
