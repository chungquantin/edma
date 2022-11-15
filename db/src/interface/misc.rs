use serde::{Deserialize, Serialize};
use serde_json::{Map, Value};

#[derive(Serialize, Deserialize, Default, Debug, PartialEq, Eq)]
pub struct JsonData {
	pub values: Map<String, Value>,
}

/// Miscellaneous
pub type Uint8Array = Vec<u8>;
pub type KeyValuePair = (Vec<u8>, Vec<u8>);
