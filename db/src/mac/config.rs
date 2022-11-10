macro_rules! impl_global_config {
	($datastore: ident) => {
		pub struct GlobalConfig {
			pub ds: $datastore,
		}

		impl GlobalConfig {
			pub fn new() -> Result<Self, Error> {
				Ok(GlobalConfig {
					ds: $datastore::default(),
				})
			}
		}
	};
}
