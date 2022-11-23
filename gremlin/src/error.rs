use crate::structure::GValue;

use thiserror::Error;

#[allow(clippy::large_enum_variant)]
#[derive(Debug, Error)]
pub enum GremlinError {
	#[error("data store disconnected")]
	Generic(String),

	#[error("Got wrong type {0:?}")]
	WrongType(GValue),

	#[error("Cast error: {0}")]
	Cast(String),

	#[error(transparent)]
	Serde(#[from] serde_json::Error),

	#[error(transparent)]
	Uuid(#[from] uuid::Error),
}
