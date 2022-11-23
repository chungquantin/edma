use crate::{err::Error, VertexResult};
use thiserror::Error;

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum IxValue {
	VertexSeq(Vec<VertexResult>),
	Null,
}

impl IxValue {
	pub fn get<'a, T>(&'a self) -> Result<&'a T, InstructionError>
	where
		T: BorrowFromIx,
	{
		T::from_ix(self)
	}
}

#[doc(hidden)]
pub trait BorrowFromIx: Sized {
	fn from_ix<'a>(v: &'a IxValue) -> Result<&'a Self, InstructionError>;
}

macro_rules! impl_borrow_from_ix {
	($t:ty, $v:path) => {
		impl BorrowFromIx for $t {
			fn from_ix<'a>(v: &'a IxValue) -> Result<&'a $t, InstructionError> {
				match v {
					$v(e) => Ok(e),
					_ => Err(InstructionError::Cast("Unable to borrow ix".to_string())),
				}
			}
		}
	};
}

#[allow(clippy::large_enum_variant)]
#[derive(Debug, Error)]
pub enum InstructionError {
	#[error("Cast error: {0}")]
	Cast(String),
}

#[derive(Debug, Clone, PartialEq)]
pub struct IxResult<'a> {
	pub operator: &'a str,
	pub source: String,
	pub value: IxValue,
}

impl_borrow_from_ix!(Vec<VertexResult>, IxValue::VertexSeq);

impl<'a> IxResult<'a> {
	pub fn empty() -> Self {
		IxResult::new("", IxValue::Null)
	}

	pub fn new(operator: &'a str, value: IxValue) -> Self {
		IxResult {
			operator,
			value,
			source: "".to_string(),
		}
	}

	pub fn set_source(&mut self, element: String) -> Result<(), Error> {
		self.source = element;
		Ok(())
	}
}
