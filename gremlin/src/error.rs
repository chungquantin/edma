use crate::structure::GValue;

use thiserror::Error;

use websocket::WebSocketError;

#[allow(clippy::large_enum_variant)]
#[derive(Debug, Error)]
pub enum GremlinError {
	#[error("data store disconnected")]
	Generic(String),

	#[error(transparent)]
	WebSocket(#[from] WebSocketError),

	#[error(transparent)]
	Pool(#[from] r2d2::Error),

	#[error("Got wrong type {0:?}")]
	WrongType(GValue),

	#[error("Cast error: {0}")]
	Cast(String),

	#[error("JSON error: {0}")]
	Json(String),

	#[error("Request error: {0:?} ")]
	Request((i16, String)),

	#[error(transparent)]
	Serde(#[from] serde_json::Error),

	#[error(transparent)]
	Uuid(#[from] uuid::Error),
}
