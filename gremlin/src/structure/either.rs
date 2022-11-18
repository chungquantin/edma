use crate::structure::{GValue, Vertex, T};

pub enum Either2<A: Into<GValue>, B: Into<GValue>> {
	A(A),
	B(B),
}

pub enum Either3<A: Into<GValue>, B: Into<GValue>, C: Into<GValue>> {
	A(A),
	B(B),
	C(C),
}

impl<A, B> From<Either2<A, B>> for GValue
where
	A: Into<GValue>,
	B: Into<GValue>,
{
	fn from(val: Either2<A, B>) -> Self {
		match val {
			Either2::A(a) => a.into(),
			Either2::B(b) => b.into(),
		}
	}
}

impl From<&str> for Either2<String, T> {
	fn from(val: &str) -> Self {
		Either2::A(String::from(val))
	}
}

impl From<T> for Either2<String, T> {
	fn from(val: T) -> Self {
		Either2::B(val)
	}
}

impl From<&str> for Either2<String, Vertex> {
	fn from(val: &str) -> Self {
		Either2::A(String::from(val))
	}
}

impl From<&Vertex> for Either2<String, Vertex> {
	fn from(val: &Vertex) -> Self {
		Either2::B(val.clone())
	}
}
