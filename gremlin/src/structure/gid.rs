use crate::{GremlinError, GremlinResult};
use uuid::Uuid;

#[derive(Debug, Clone)]
pub struct GIDs(pub(crate) Vec<GID>);

impl<T: Into<GID>> From<T> for GIDs {
	fn from(val: T) -> GIDs {
		GIDs(vec![val.into()])
	}
}

impl<T: Into<GID>> From<Vec<T>> for GIDs {
	fn from(val: Vec<T>) -> GIDs {
		GIDs(val.into_iter().map(|gid| gid.into()).collect())
	}
}

impl From<()> for GIDs {
	fn from(_val: ()) -> GIDs {
		GIDs(vec![])
	}
}

#[derive(Debug, PartialEq, Eq, Hash, Clone)]
pub enum GID {
	String(String),
	Int32(i32),
	Int64(i64),
}

impl GID {
	pub fn get<'a, T>(&'a self) -> GremlinResult<&'a T>
	where
		T: BorrowFromGID,
	{
		T::from_gid(self)
	}
}

impl From<&'static str> for GID {
	fn from(val: &str) -> Self {
		GID::String(String::from(val))
	}
}

impl From<String> for GID {
	fn from(val: String) -> Self {
		GID::String(val)
	}
}

impl From<i32> for GID {
	fn from(val: i32) -> Self {
		GID::Int32(val)
	}
}

impl From<i64> for GID {
	fn from(val: i64) -> Self {
		GID::Int64(val)
	}
}

impl From<&GID> for GID {
	fn from(val: &GID) -> Self {
		val.clone()
	}
}

impl From<Uuid> for GID {
	fn from(val: Uuid) -> Self {
		GID::String(val.to_string())
	}
}

// Borrow from GID

#[doc(hidden)]
pub trait BorrowFromGID: Sized {
	fn from_gid<'a>(v: &'a GID) -> GremlinResult<&'a Self>;
}

macro_rules! impl_borrow_from_gid {
	($t:ty, $v:path) => {
		impl BorrowFromGID for $t {
			fn from_gid<'a>(v: &'a GID) -> GremlinResult<&'a $t> {
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

impl_borrow_from_gid!(String, GID::String);
impl_borrow_from_gid!(i32, GID::Int32);
impl_borrow_from_gid!(i64, GID::Int64);
