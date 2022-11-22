use crate::{err::Error, VertexResult};

#[derive(Debug, Clone, PartialEq)]
pub enum IxValue {
	VertexSeq(Vec<VertexResult>),
	None,
}

impl IxValue {
	pub fn get<'a, T>(&'a self) -> Result<&'a T, Error>
	where
		T: BorrowFromIx,
	{
		T::from_ix(self)
	}
}

#[doc(hidden)]
pub trait BorrowFromIx: Sized {
	fn from_ix<'a>(v: &'a IxValue) -> Result<&'a Self, Error>;
}

macro_rules! impl_borrow_from_ix {
	($t:ty, $v:path) => {
		impl BorrowFromIx for $t {
			fn from_ix<'a>(v: &'a IxValue) -> Result<&'a $t, Error> {
				match v {
					$v(e) => Ok(e),
					_ => panic!("UNable to borrow from ix"),
				}
			}
		}
	};
}

#[derive(Debug, Clone, PartialEq)]
pub struct IxResult<'a> {
	pub operator: &'a str,
	pub source_operator: String,
	pub value: IxValue,
}

impl_borrow_from_ix!(Vec<VertexResult>, IxValue::VertexSeq);
