macro_rules! impl_controller {
	($c_name: ident($c_cf: expr)) => {
		use $crate::storage::DBRef;
		use $crate::storage::Datastore;

		pub struct $c_name<'a> {
			pub ds_ref: DBRef<'a>,
			pub cf: &'static str,
		}

		impl<'a> $c_name<'a> {
			pub fn new(ds_ref: DBRef<'a>) -> Self {
				$c_name {
					ds_ref,
					cf: $c_cf,
				}
			}

			fn get_cf(&self) -> Option<Vec<u8>> {
				Some(self.cf.into())
			}

			fn get_ds(&self) -> &Datastore {
				self.ds_ref.db
			}

			pub async fn count(&self) -> Result<usize, Error> {
				let mut tx = self.get_ds().transaction(true).unwrap();
				Ok(tx.count(self.get_cf()).await.unwrap())
			}
		}
	};
}
