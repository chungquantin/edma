use crate::conversion::FromGValue;
use crate::{GValue, GremlinResult};

#[derive(Debug, PartialEq, Clone)]
pub struct Traverser {
	bulk: i64,
	value: Box<GValue>,
}

impl Traverser {
	pub fn new(bulk: i64, value: GValue) -> Traverser {
		Traverser {
			bulk,
			value: Box::new(value),
		}
	}

	pub fn take<T>(self) -> GremlinResult<T>
	where
		T: FromGValue,
	{
		T::from_gvalue(*self.value)
	}
}
