macro_rules! impl_global_transaction {
	($($x: ident; feat $feat: expr), *) => {
		#[async_trait(?Send)]
		impl SimpleTransaction for Transaction {
			// Check if closed
			fn closed(&self) -> bool {
				match self {
					$(
						#[cfg(feature = $feat)]
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
						#[cfg(feature = $feat)]
						Transaction {
							inner: Inner::$x(ds),
							..
						} => ds.cancel().await,
					)*
				}
			}

			// Count number of items
			async fn count(&mut self, tags: TagBucket) -> Result<usize, Error> {
				match self {
					$(
						#[cfg(feature = $feat)]
						Transaction {
							inner: Inner::$x(ds),
							..
						} => ds.count(tags).await,
					)*
				}
			}

			// Commit a transaction
			async fn commit(&mut self) -> Result<(), Error> {
				match self {
					$(
						#[cfg(feature = $feat)]
						Transaction {
							inner: Inner::$x(ds),
							..
						} => ds.commit().await,
					)*
				}
			}

			// Check if a key exists
			async fn exi<K: Into<Key> + Send>(&self, key: K, tags: TagBucket) -> Result<bool, Error> {
				match self {
					$(
						#[cfg(feature = $feat)]
						Transaction {
							inner: Inner::$x(ds),
							..
						} => ds.exi(key, tags).await,
					)*
				}
			}

			/// Fetch a key from the database
			async fn get<K: Into<Key> + Send>(&self, key: K, tags: TagBucket) -> Result<Option<Val>, Error> {
				match self {
					$(
						#[cfg(feature = $feat)]
						Transaction {
							inner: Inner::$x(ds),
							..
						} => ds.get(key, tags).await,
					)*
				}
			}

			/// Insert or update a key in the database
			async fn set<K: Into<Key> + Send, V: Into<Key> + Send>(
				&mut self,
				key: K,
				val: V,
				tags: TagBucket
			) -> Result<(), Error> {
				match self {
					$(
						#[cfg(feature = $feat)]
						Transaction {
							inner: Inner::$x(ds),
							..
						} => ds.set(key, val, tags).await,
					)*
				}
			}

			/// Insert a key if it doesn't exist in the database
			async fn put<K: Into<Key> + Send, V: Into<Key> + Send>(
				&mut self,
				key: K,
				val: V,
				tags: TagBucket
			) -> Result<(), Error> {
				match self {
					$(
						#[cfg(feature = $feat)]
						Transaction {
							inner: Inner::$x(ds),
							..
						} => ds.put(key, val, tags).await,
					)*
				}
			}

			/// Delete a key
			async fn del<K: Into<Key> + Send>(&mut self, key: K, tags: TagBucket) -> Result<(), Error> {
				match self {
					$(
						#[cfg(feature = $feat)]
						Transaction {
							inner: Inner::$x(ds),
							..
						} => ds.del(key, tags).await,
					)*
				}
			}

			async fn prefix_iterate<P>(
				&self,
				prefix: P,
				tags: TagBucket
			) -> Result<Vec<Result<(Val, Val), Error>>, Error>
			where
				P: Into<Key> + Send,
			{
				match self {
					$(
						#[cfg(feature = $feat)]
						Transaction {
							inner: Inner::$x(ds),
							..
						} => ds.prefix_iterate(prefix, tags).await,
					)*
				}
			}

			async fn suffix_iterate<S>(
				&self,
				suffix: S,
				tags: TagBucket,
			) -> Result<Vec<Result<(Val, Val), Error>>, Error>
			where
				S: Into<Key> + Send,
			{
				match self {
					$(
						#[cfg(feature = $feat)]
						Transaction {
							inner: Inner::$x(ds),
							..
						} => ds.suffix_iterate(suffix, tags).await,
					)*
				}
			}

			async fn iterate(&self, tags: TagBucket) -> Result<Vec<Result<(Val, Val), Error>>, Error> {
				match self {
					$(
						Transaction {
							inner: Inner::$x(ds),
							..
						} => ds.iterate(tags).await,
					)*
				}
			}
		}
	}
}
