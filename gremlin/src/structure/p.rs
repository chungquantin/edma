use crate::structure::either::Either2;
use crate::structure::text_p::TextP;
use crate::{GValue, ToGValue};

#[derive(Debug, PartialEq, Clone)]
pub struct Predicate {
	pub(crate) operator: String,
	pub(crate) value: Box<GValue>,
}

impl Predicate {
	pub fn operator(&self) -> &String {
		&self.operator
	}

	pub fn value(&self) -> &GValue {
		&self.value
	}

	pub(crate) fn new<T>(operator: T, value: GValue) -> Predicate
	where
		T: Into<String>,
	{
		Predicate {
			operator: operator.into(),
			value: Box::new(value),
		}
	}
	pub fn eq<V>(value: V) -> Predicate
	where
		V: ToGValue,
	{
		Predicate::new("eq", value.to_gvalue())
	}

	pub fn neq<V>(value: V) -> Predicate
	where
		V: ToGValue,
	{
		Predicate::new("neq", value.to_gvalue())
	}

	pub fn gt<V>(value: V) -> Predicate
	where
		V: ToGValue,
	{
		Predicate::new("gt", value.to_gvalue())
	}

	pub fn gte<V>(value: V) -> Predicate
	where
		V: ToGValue,
	{
		Predicate::new("gte", value.to_gvalue())
	}

	pub fn lt<V>(value: V) -> Predicate
	where
		V: ToGValue,
	{
		Predicate::new("lt", value.to_gvalue())
	}

	pub fn lte<V>(value: V) -> Predicate
	where
		V: ToGValue,
	{
		Predicate::new("lte", value.to_gvalue())
	}

	pub fn within<V>(value: V) -> Predicate
	where
		V: IntoRange,
	{
		Predicate::new("within", value.into_range().values.into())
	}
}

pub trait IntoPredicate {
	fn into_predicate(self) -> Either2<Predicate, TextP>;
}

impl<T: ToGValue> IntoPredicate for T {
	fn into_predicate(self) -> Either2<Predicate, TextP> {
		let val = self.to_gvalue();
		match val {
			GValue::P(ref p) => Either2::A(p.clone()),
			GValue::TextP(ref p) => Either2::B(p.clone()),
			_ => Either2::A(Predicate::new("eq", val)),
		}
	}
}

pub struct Range {
	values: Vec<GValue>,
}

pub trait IntoRange {
	fn into_range(self) -> Range;
}

impl<T> IntoRange for (T, T)
where
	T: ToGValue,
{
	fn into_range(self) -> Range {
		let v1 = self.0.to_gvalue();
		let v2 = self.1.to_gvalue();

		Range {
			values: vec![v1, v2],
		}
	}
}

impl<T> IntoRange for Vec<T>
where
	T: ToGValue,
{
	fn into_range(self) -> Range {
		Range {
			values: self.into_iter().map(|e| e.to_gvalue()).collect(),
		}
	}
}
