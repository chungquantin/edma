use futures::lock::Mutex;
use std::{pin::Pin, sync::Arc};

use crate::{err::Error, util::time::get_epoch_ms};

/// # Distributed Database Transaction
/// ## Atomically reference counter
/// Shared references in Rust disallow mutation by default, and Arc is no exception: you cannot
/// generally obtain a mutable reference to something inside an Arc. If you need to mutate
/// through an Arc, use Mutex, RwLock, or one of the Atomic types.
///
/// because it tries to borrow arc as mutable. For it to happen, DerefMut would have
/// to be implemented for Arc but it's not because Arc is not meant to be mutable.
#[derive(Debug)]
pub struct DBTransaction<DBType, TxType>
where
	DBType: 'static,
	TxType: 'static,
{
	pub tx: Arc<Mutex<Option<TxType>>>,
	pub ok: bool,
	pub writable: bool,
	pub readable: bool,
	pub timestamp: u128,
	pub _db: Pin<Arc<DBType>>,
}

impl<DBType, TxType> DBTransaction<DBType, TxType>
where
	DBType: 'static,
	TxType: 'static,
{
	pub fn new(tx: TxType, rw: bool, db: Pin<Arc<DBType>>) -> Result<Self, Error> {
		Ok(DBTransaction {
			tx: Arc::new(Mutex::new(Some(tx))),
			ok: false,
			writable: rw,
			readable: true,
			timestamp: get_epoch_ms(),
			_db: db,
		})
	}
}
