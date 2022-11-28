use gremlin::GValue;
use thiserror::Error;

#[doc(hidden)]
pub trait BorrowFromIx: Sized {
	fn from_ix<'a>(v: &'a GValue) -> Result<&'a Self, InstructionError>;
}

#[allow(clippy::large_enum_variant)]
#[derive(Debug, Error)]
pub enum InstructionError {
	#[error("Cast error: {0}")]
	Cast(String),
}

#[derive(Debug, Clone, PartialEq)]
pub struct IxResult {
	pub operator: String,
	pub value: GValue,
}

impl Default for IxResult {
	fn default() -> Self {
		IxResult::new("", GValue::Null)
	}
}

impl IxResult {
	pub fn is_empty(&self) -> bool {
		self.operator.is_empty()
			&& match &self.value {
				GValue::Null => true,
				_ => false,
			}
	}

	pub fn new(operator: &str, value: GValue) -> Self {
		IxResult {
			operator: String::from(operator),
			value,
		}
	}
}
