use std::{
	ops::Range,
	sync::{Arc, Mutex},
};

use rocksdb::Error;

use crate::{
	interface::kv::{Key, Val},
	util::time::get_epoch_ms,
};

/// # Distributed Database Transaction
/// ## Atomically reference counter
/// Shared references in Rust disallow mutation by default, and Arc is no exception: you cannot
/// generally obtain a mutable reference to something inside an Arc. If you need to mutate
/// through an Arc, use Mutex, RwLock, or one of the Atomic types.
///
/// because it tries to borrow arc as mutable. For it to happen, DerefMut would have
/// to be implemented for Arc but it's not because Arc is not meant to be mutable.
#[derive(Debug)]
pub struct Transaction<Inner> {
	pub inner: Arc<Mutex<Option<Inner>>>,
	pub ok: bool,
	pub err: bool,
	pub writable: bool,
	pub readable: bool,
	pub timestamp: u128,
}

impl<I> Transaction<I> {
	pub fn new(tx: I, w: bool, r: bool) -> Self {
		Transaction {
			inner: Arc::new(Mutex::new(Some(tx))),
			ok: false,
			err: false,
			writable: w,
			readable: r,
			timestamp: get_epoch_ms(),
		}
	}
}

pub trait TransactionFn {
	// Check if closed
	fn closed(&self) -> bool;
	// Cancel a transaction
	fn cancel(&mut self) -> Result<(), Error>;
	// Commit a transaction
	fn commit(&mut self) -> Result<(), Error>;
	// Check if a key exists
	fn exist<K>(&mut self, key: K) -> Result<bool, Error>
	where
		K: Into<Key>;
	// Fetch a key from the database
	fn get<K>(&mut self, key: K) -> Result<Option<Val>, Error>;
	// Insert or update a key in the database
	fn set<K, V>(&mut self, key: K, val: V) -> Result<(), Error>;
	// Insert a key if it doesn't exist in the database
	fn put<K, V>(&mut self, key: K, val: V) -> Result<(), Error>;
	// Delete a key
	fn del<K>(&mut self, key: K) -> Result<(), Error>;
	// Retrieve a range of keys from the databases
	fn scan<K>(&mut self, rng: Range<K>, limit: u32) -> Result<Vec<(Key, Val)>, Error>;
}
