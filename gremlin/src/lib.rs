#[macro_use]
extern crate lazy_static;

mod conversion;
mod error;

pub use conversion::{BorrowFromGValue, FromGValue, ToGValue};
pub use error::GremlinError;

pub type GremlinResult<T> = Result<T, error::GremlinError>;

pub use structure::{
	Cardinality, Edge, GKey, GValue, IntermediateRepr, List, Map, Metric, Path, Property, Token,
	TraversalExplanation, TraversalMetrics, Vertex, VertexProperty, GID,
};

pub mod process;
pub mod structure;
pub mod utils;

#[cfg(feature = "derive")]
pub mod derive {
	pub use gremlin_derive::FromGMap;
	pub use gremlin_derive::FromGValue;
}
