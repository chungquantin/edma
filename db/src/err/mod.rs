use thiserror::Error;

#[derive(Error, Debug)]
pub enum Error {
	/// This error is used for ignoring a document when processing a query
	#[doc(hidden)]
	#[error("Conditional clause is not truthy")]
	Ignore,

	/// There was a problem with the underlying datastore
	#[error("There was a problem with the underlying datastore: {0}")]
	Ds(String),

	/// There was a problem with a datastore transaction
	#[error("There was a problem with a datastore transaction: {0}")]
	Tx(String),

	/// There was an error when starting a new datastore transaction
	#[error("There was an error when starting a new datastore transaction")]
	TxFailure,

	/// The transaction was already cancelled or committed
	#[error("Couldn't update a finished transaction")]
	TxFinished,

	/// The current transaction was created as read-only
	#[error("Couldn't write to a read only transaction")]
	TxReadonly,

	/// The conditional value in the request was not equal
	#[error("Value being checked was not correct")]
	TxConditionNotMet,

	/// The key being mutated is not in the database
	#[error("The key is not in the database")]
	TxnKeyNotFound,

	/// The key being inserted in the transaction already exists
	#[error("The key being inserted already exists")]
	TxKeyAlreadyExists,

	/// It's is not possible to convert between the two types
	#[error("Cannot convert from '{0}' to '{1}'")]
	TryFromError(String, &'static str),
}

#[cfg(feature = "kv-rocksdb")]
impl From<rocksdb::Error> for Error {
	fn from(e: rocksdb::Error) -> Error {
		Error::Tx(e.to_string())
	}
}
