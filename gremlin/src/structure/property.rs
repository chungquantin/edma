use crate::conversion::{BorrowFromGValue, FromGValue};
use crate::GValue;
use crate::GremlinResult;

#[derive(Debug, PartialEq, Clone)]
pub struct Property {
	label: String,
	value: Box<GValue>,
}

impl Property {
	pub fn new<T, GT>(label: T, value: GT) -> Property
	where
		T: Into<String>,
		GT: Into<GValue>,
	{
		Property {
			label: label.into(),
			value: Box::new(value.into()),
		}
	}

	pub fn value(&self) -> &GValue {
		&self.value
	}

	pub fn take<T>(self) -> GremlinResult<T>
	where
		T: FromGValue,
	{
		T::from_gvalue(*self.value)
	}

	pub fn get<T>(&self) -> GremlinResult<&T>
	where
		T: BorrowFromGValue,
	{
		T::from_gvalue(&self.value)
	}

	pub fn label(&self) -> &String {
		&self.label
	}
}
