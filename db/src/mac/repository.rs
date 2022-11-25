macro_rules! impl_repository {
	($c_name: ident($c_cf: ident)) => {
		use $crate::constant::ColumnFamily;
		use $crate::constant::COLUMN_FAMILIES;
		use $crate::storage::Datastore;
		use $crate::storage::DatastoreRef;

		#[derive(Clone)]
		pub struct $c_name<'a> {
			pub ds_ref: DatastoreRef<'a>,
			pub cf: &'static str,
		}

		impl<'a> $c_name<'a> {
			pub fn new(ds_ref: DatastoreRef<'a>) -> Self {
				$c_name {
					ds_ref,
					cf: COLUMN_FAMILIES.get(&ColumnFamily::$c_cf).unwrap(),
				}
			}

			fn cf(&self) -> Option<Vec<u8>> {
				Some(self.cf.into())
			}

			fn ds(&self) -> &Datastore {
				self.ds_ref.db
			}

			pub fn tx(&self) -> Transaction {
				self.ds().transaction(false).unwrap()
			}

			pub fn mut_tx(&self) -> Transaction {
				self.ds().transaction(true).unwrap()
			}

			pub async fn count(&self) -> Result<usize, Error> {
				let mut tx = self.ds().transaction(true).unwrap();
				Ok(tx.count(self.cf()).await.unwrap())
			}
		}
	};
}
