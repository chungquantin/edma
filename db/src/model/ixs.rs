use crate::err::Error;
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
	pub source: String,
	pub value: GValue,
}

impl IxResult {
	pub fn empty() -> Self {
		IxResult::new(&String::from(""), GValue::Null)
	}

	pub fn new(operator: &str, value: GValue) -> Self {
		IxResult {
			operator: String::from(operator),
			value,
			source: "".to_string(),
		}
	}

	pub fn set_source(&mut self, element: String) -> Result<(), Error> {
		self.source = element;
		Ok(())
	}
}
