use std::marker::PhantomData;

use crate::DBTransaction;
use db_key::Key;
use leveldb::database::Database as LevelDBDatabase;

/// Wrapper type for Vec<u8> keys to implement the db-key Key trait.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct LevelDBKey(pub Vec<u8>);

impl Key for LevelDBKey {
	fn from_u8(key: &[u8]) -> Self {
		LevelDBKey(key.to_vec())
	}

	fn as_slice<T, F: Fn(&[u8]) -> T>(&self, f: F) -> T {
		f(&self.0)
	}
}

impl From<Vec<u8>> for LevelDBKey {
	fn from(v: Vec<u8>) -> Self {
		LevelDBKey(v)
	}
}

pub type DBType = LevelDBDatabase<LevelDBKey>;
pub type TxType = PhantomData<String>;
pub type LevelDBTransaction = DBTransaction<DBType, TxType>;
