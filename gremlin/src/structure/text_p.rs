use crate::{GValue, ToGValue};

#[derive(Debug, PartialEq, Clone)]
pub struct TextP {
	pub(crate) operator: String,
	pub(crate) value: Box<GValue>,
}

impl TextP {
	pub fn operator(&self) -> &String {
		&self.operator
	}

	pub fn value(&self) -> &GValue {
		&self.value
	}

	pub(crate) fn new<T>(operator: T, value: GValue) -> TextP
	where
		T: Into<String>,
	{
		TextP {
			operator: operator.into(),
			value: Box::new(value),
		}
	}
	pub fn containing<V>(value: V) -> TextP
	where
		V: ToGValue,
	{
		TextP::new("containing", value.to_gvalue())
	}

	pub fn starting_with<V>(value: V) -> TextP
	where
		V: ToGValue,
	{
		TextP::new("startingWith", value.to_gvalue())
	}

	pub fn ending_with<V>(value: V) -> TextP
	where
		V: ToGValue,
	{
		TextP::new("endingWith", value.to_gvalue())
	}

	pub fn not_starting_with<V>(value: V) -> TextP
	where
		V: ToGValue,
	{
		TextP::new("notStartingWith", value.to_gvalue())
	}

	pub fn not_ending_with<V>(value: V) -> TextP
	where
		V: ToGValue,
	{
		TextP::new("notEndingWith", value.to_gvalue())
	}

	pub fn not_containing<V>(value: V) -> TextP
	where
		V: ToGValue,
	{
		TextP::new("notContaining", value.to_gvalue())
	}
}
