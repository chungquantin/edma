macro_rules! impl_controller {
	($c_name: ident($c_cf: expr)) => {
		use $crate::GlobalConfig;

		pub struct $c_name {
			pub config: GlobalConfig,
			pub cf: &'static str,
		}

		impl $c_name {
			pub fn new() -> Result<Self, Error> {
				Ok($c_name {
					config: GlobalConfig::new().unwrap(),
					cf: $c_cf,
				})
			}

			pub fn get_cf(&self) -> Option<Vec<u8>> {
				Some(self.cf.into())
			}
		}
	};
}
