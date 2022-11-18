use crate::{
	process::traversal::Bytecode,
	structure::{TextP, P as Predicate},
	Edge, GKey, GValue, GremlinError, GremlinResult, IntermediateRepr, List, Map, Metric, Path,
	Property, Token, TraversalExplanation, TraversalMetrics, Vertex, VertexProperty, GID,
};

use crate::structure::Set;
use crate::structure::Traverser;

use std::collections::HashMap;

pub trait ToGValue: Send + Sync {
	fn to_gvalue(&self) -> GValue;
}

#[derive(Debug, PartialEq)]
pub struct Params(pub HashMap<String, GValue>);

impl Into<Params> for () {
	fn into(self) -> Params {
		Params(HashMap::new())
	}
}

impl ToGValue for Vec<GValue> {
	fn to_gvalue(&self) -> GValue {
		GValue::List(List::new(self.clone()))
	}
}

impl ToGValue for GID {
	fn to_gvalue(&self) -> GValue {
		match self {
			GID::Int32(n) => GValue::from(*n),
			GID::Int64(n) => GValue::from(*n),
			GID::String(n) => GValue::from(n),
		}
	}
}

macro_rules! impl_to_gvalue {
	($t:ty, $v:path) => {
		impl ToGValue for $t {
			fn to_gvalue(&self) -> GValue {
				$v(*self)
			}
		}
	};
}

impl_to_gvalue!(f32, GValue::Float);
impl_to_gvalue!(f64, GValue::Double);
impl_to_gvalue!(i32, GValue::Int32);
impl_to_gvalue!(i64, GValue::Int64);
impl_to_gvalue!(chrono::DateTime<chrono::Utc>, GValue::Date);
impl_to_gvalue!(uuid::Uuid, GValue::Uuid);
impl_to_gvalue!(bool, GValue::Bool);

impl ToGValue for &str {
	fn to_gvalue(&self) -> GValue {
		GValue::String(String::from(*self))
	}
}

impl ToGValue for Predicate {
	fn to_gvalue(&self) -> GValue {
		GValue::P(self.clone())
	}
}

impl ToGValue for TextP {
	fn to_gvalue(&self) -> GValue {
		GValue::TextP(self.clone())
	}
}

impl ToGValue for String {
	fn to_gvalue(&self) -> GValue {
		GValue::String(self.clone())
	}
}

impl ToGValue for Bytecode {
	fn to_gvalue(&self) -> GValue {
		GValue::Bytecode(self.clone())
	}
}

// Take from GValue

#[doc(hidden)]
pub trait FromGValue: Sized {
	fn from_gvalue(v: GValue) -> GremlinResult<Self>;
}

macro_rules! impl_from_gvalue {
	($t:ty, $v:path) => {
		impl FromGValue for $t {
			fn from_gvalue(v: GValue) -> GremlinResult<$t> {
				match v {
					$v(e) => Ok(e),
					_ => Err(GremlinError::Cast(format!(
						"Cannot convert {:?} to {}",
						v,
						stringify!($t)
					))),
				}
			}
		}
	};
}

impl_from_gvalue!(VertexProperty, GValue::VertexProperty);
impl_from_gvalue!(Property, GValue::Property);
impl_from_gvalue!(Map, GValue::Map);
impl_from_gvalue!(Set, GValue::Set);
impl_from_gvalue!(List, GValue::List);
impl_from_gvalue!(Token, GValue::Token);
impl_from_gvalue!(Vertex, GValue::Vertex);
impl_from_gvalue!(Edge, GValue::Edge);
impl_from_gvalue!(Path, GValue::Path);
impl_from_gvalue!(String, GValue::String);
impl_from_gvalue!(f32, GValue::Float);
impl_from_gvalue!(f64, GValue::Double);
impl_from_gvalue!(i32, GValue::Int32);
impl_from_gvalue!(i64, GValue::Int64);
impl_from_gvalue!(bool, GValue::Bool);
impl_from_gvalue!(uuid::Uuid, GValue::Uuid);
impl_from_gvalue!(Metric, GValue::Metric);
impl_from_gvalue!(TraversalMetrics, GValue::TraversalMetrics);
impl_from_gvalue!(TraversalExplanation, GValue::TraversalExplanation);
impl_from_gvalue!(IntermediateRepr, GValue::IntermediateRepr);
impl_from_gvalue!(chrono::DateTime<chrono::Utc>, GValue::Date);
impl_from_gvalue!(Traverser, GValue::Traverser);

impl FromGValue for GKey {
	fn from_gvalue(v: GValue) -> GremlinResult<GKey> {
		match v {
			GValue::String(s) => Ok(GKey::String(s)),
			GValue::Token(s) => Ok(GKey::String(s.value().clone())),
			GValue::Vertex(s) => Ok(GKey::Vertex(s)),
			GValue::Edge(s) => Ok(GKey::Edge(s)),
			_ => Err(GremlinError::Cast(format!("Cannot convert {:?} to {}", v, "GKey"))),
		}
	}
}

impl FromGValue for GValue {
	fn from_gvalue(v: GValue) -> GremlinResult<GValue> {
		Ok(v)
	}
}
// Borrow from GValue

impl<T: FromGValue> FromGValue for Vec<T> {
	fn from_gvalue(v: GValue) -> GremlinResult<Vec<T>> {
		match v {
			GValue::List(l) => {
				let results: GremlinResult<Vec<T>> =
					l.take().into_iter().map(T::from_gvalue).collect();
				Ok(results?)
			}
			_ => Err(GremlinError::Cast(format!("Cannot convert {:?} to List of T", v))),
		}
	}
}

#[doc(hidden)]
pub trait BorrowFromGValue: Sized {
	fn from_gvalue<'a>(v: &'a GValue) -> GremlinResult<&'a Self>;
}

macro_rules! impl_borrow_from_gvalue {
	($t:ty, $v:path) => {
		impl BorrowFromGValue for $t {
			fn from_gvalue<'a>(v: &'a GValue) -> GremlinResult<&'a $t> {
				match v {
					$v(e) => Ok(e),
					_ => Err(GremlinError::Cast(format!(
						"Cannot convert {:?} to {}",
						v,
						stringify!($t)
					))),
				}
			}
		}
	};
}

impl_borrow_from_gvalue!(VertexProperty, GValue::VertexProperty);
impl_borrow_from_gvalue!(Property, GValue::Property);
impl_borrow_from_gvalue!(Map, GValue::Map);
impl_borrow_from_gvalue!(Set, GValue::Set);
impl_borrow_from_gvalue!(List, GValue::List);
impl_borrow_from_gvalue!(Vertex, GValue::Vertex);
impl_borrow_from_gvalue!(Edge, GValue::Edge);
impl_borrow_from_gvalue!(Path, GValue::Path);
impl_borrow_from_gvalue!(String, GValue::String);
impl_borrow_from_gvalue!(Token, GValue::Token);
impl_borrow_from_gvalue!(f32, GValue::Float);
impl_borrow_from_gvalue!(f64, GValue::Double);
impl_borrow_from_gvalue!(i32, GValue::Int32);
impl_borrow_from_gvalue!(i64, GValue::Int64);
impl_borrow_from_gvalue!(uuid::Uuid, GValue::Uuid);
impl_borrow_from_gvalue!(chrono::DateTime<chrono::Utc>, GValue::Date);
impl_borrow_from_gvalue!(bool, GValue::Bool);

#[cfg(feature = "test-suite")]
#[test]
fn to_gvalue_for_vec_gvalue() {
	let ids_from_somewhere = vec![1, 2, 3];
	let converted_ids: Vec<GValue> = ids_from_somewhere.into_iter().map(|x| x.into()).collect();
	let actual: GValue = converted_ids.clone().into();
	let expected = GValue::List(List::new(converted_ids));
	assert_eq!(actual, expected);
}
