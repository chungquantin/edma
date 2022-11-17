macro_rules! impl_global_transaction {
	($($x: ident), *) => {
		#[async_trait]
		impl SimpleTransaction for Transaction {
			// Check if closed
			fn closed(&self) -> bool {
				match self {
					$(
						Transaction {
							inner: Inner::$x(ds),
							..
						} => ds.closed(),
					)*
				}
			}

			// Cancel a transaction
			async fn cancel(&mut self) -> Result<(), Error> {
				match self {
					$(
						Transaction {
							inner: Inner::$x(ds),
							..
						} => ds.cancel().await,
					)*
				}
			}

			// Count number of items
			async fn count(&mut self, cf: CF) -> Result<usize, Error> {
				match self {
					$(
						Transaction {
							inner: Inner::$x(ds),
							..
						} => ds.count(cf).await,
					)*
				}
			}

			// Commit a transaction
			async fn commit(&mut self) -> Result<(), Error> {
				match self {
					$(
						Transaction {
							inner: Inner::$x(ds),
							..
						} => ds.commit().await,
					)*
				}
			}

			// Check if a key exists
			async fn exi<K: Into<Key> + Send>(&self, cf: CF, key: K) -> Result<bool, Error> {
				match self {
					$(
						Transaction {
							inner: Inner::$x(ds),
							..
						} => ds.exi(cf, key).await,
					)*
				}
			}

			/// Fetch a key from the database
			async fn get<K: Into<Key> + Send>(&self, cf: CF, key: K) -> Result<Option<Val>, Error> {
				match self {
					$(
						Transaction {
							inner: Inner::$x(ds),
							..
						} => ds.get(cf, key).await,
					)*
				}
			}

			// OPTIONAL Fetch multiple keys from the database
			async fn multi_get<K: Into<Key> + Send + AsRef<[u8]>>(
				&self,
				cf: CF,
				keys: Vec<K>,
			) -> Result<Vec<Option<Val>>, Error> {
				match self {
					$(
						Transaction {
							inner: Inner::$x(ds),
							..
						} => ds.multi_get(cf, keys).await,
					)*
				}
			}

			/// Insert or update a key in the database
			async fn set<K: Into<Key> + Send, V: Into<Key> + Send>(
				&mut self,
				cf: CF,
				key: K,
				val: V,
			) -> Result<(), Error> {
				match self {
					$(
						Transaction {
							inner: Inner::$x(ds),
							..
						} => ds.set(cf, key, val).await,
					)*
				}
			}

			/// Insert a key if it doesn't exist in the database
			async fn put<K: Into<Key> + Send, V: Into<Key> + Send>(
				&mut self,
				cf: CF,
				key: K,
				val: V,
			) -> Result<(), Error> {
				match self {
					$(
						Transaction {
							inner: Inner::$x(ds),
							..
						} => ds.put(cf, key, val).await,
					)*
				}
			}

			/// Delete a key
			async fn del<K: Into<Key> + Send>(&mut self, cf: CF, key: K) -> Result<(), Error> {
				match self {
					$(
						Transaction {
							inner: Inner::$x(ds),
							..
						} => ds.del(cf, key ).await,
					)*
				}
			}

			async fn prefix_iterate<P>(
				&self,
				cf: CF,
				prefix: P,
			) -> Result<Vec<Result<(Val, Val), Error>>, Error>
			where
				P: Into<Key> + Send,
			{
				match self {
					$(
						Transaction {
							inner: Inner::$x(ds),
							..
						} => ds.prefix_iterate(cf, prefix).await,
					)*
				}
			}

			async fn suffix_iterate<S>(
				&self,
				cf: CF,
				suffix: S,
			) -> Result<Vec<Result<(Val, Val), Error>>, Error>
			where
				S: Into<Key> + Send,
			{
				match self {
					$(
						Transaction {
							inner: Inner::$x(ds),
							..
						} => ds.suffix_iterate(cf, suffix).await,
					)*
				}
			}

			async fn iterate(&self, cf: CF) -> Result<Vec<Result<(Val, Val), Error>>, Error> {
				match self {
					$(
						Transaction {
							inner: Inner::$x(ds),
							..
						} => ds.iterate(cf ).await,
					)*
				}
			}
		}
	}
}
