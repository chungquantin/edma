use crate::{
	err::Error,
	interface::kv::{Key, Val},
	util::time::get_now,
};
use async_trait::async_trait;
use futures::lock::Mutex;
use std::{pin::Pin, sync::Arc};

pub type CF = Option<Vec<u8>>;

/// # Distributed Database Transaction
/// ## Atomically reference counter
/// Shared references in Rust disallow mutation by default, and Arc is no exception: you cannot
/// generally obtain a mutable reference to something inside an Arc. If you need to mutate
/// through an Arc, use Mutex, RwLock, or one of the Atomic types.
///
/// because it tries to borrow arc as mutable. For it to happen, DerefMut would have
/// to be implemented for Arc but it's not because Arc is not meant to be mutable.
#[derive(Debug)]
pub struct DBTransaction<D, T>
where
	D: 'static,
	T: 'static,
{
	pub tx: Arc<Mutex<Option<T>>>,
	pub ok: bool,
	pub writable: bool,
	pub readable: bool,
	pub timestamp: i64,
	pub _db: Pin<Arc<D>>,
}

#[async_trait]
pub trait SimpleTransaction {
	// Check if closed
	fn closed(&self) -> bool;

	// Cancel a transaction
	async fn cancel(&mut self) -> Result<(), Error>;

	// Commit a transaction
	async fn commit(&mut self) -> Result<(), Error>;

	// Check if a key exists
	async fn exi<K: Into<Key> + Send>(&mut self, cf: CF, key: K) -> Result<bool, Error>;

	/// Fetch a key from the database
	async fn get<K: Into<Key> + Send>(&mut self, cf: CF, key: K) -> Result<Option<Val>, Error>;

	/// Insert or update a key in the database
	async fn set<K: Into<Key> + Send, V: Into<Key> + Send>(
		&mut self,
		cf: CF,
		key: K,
		val: V,
	) -> Result<(), Error>;

	/// Insert a key if it doesn't exist in the database
	async fn put<K: Into<Key> + Send, V: Into<Key> + Send>(
		&mut self,
		cf: CF,
		key: K,
		val: V,
	) -> Result<(), Error>;

	/// Delete a key
	async fn del<K: Into<Key> + Send>(&mut self, cf: CF, key: K) -> Result<(), Error>;
}

impl<DBType, TxType> DBTransaction<DBType, TxType>
where
	DBType: 'static,
	TxType: 'static,
{
	pub fn new(tx: TxType, db: Pin<Arc<DBType>>, rw: bool) -> Result<Self, Error> {
		Ok(DBTransaction {
			tx: Arc::new(Mutex::new(Some(tx))),
			ok: false,
			writable: rw,
			readable: true,
			timestamp: get_now(),
			_db: db,
		})
	}
}
