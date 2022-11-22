mod anonymous_traversal_source;
mod builder;
mod bytecode;
mod graph_traversal;
mod graph_traversal_source;
mod internal;
mod order;
mod scope;
mod step;

pub use internal::MockTerminator;
pub use internal::Terminator;
pub use order::Order;

pub use builder::TraversalBuilder;
pub use bytecode::{Bytecode, Instruction, WRITE_OPERATORS};
pub use graph_traversal::GraphTraversal;
pub use graph_traversal_source::GraphTraversalSource;
pub use scope::Scope;

pub use anonymous_traversal_source::AnonymousTraversalSource;

use lazy_static::lazy_static;

pub use step::*;

pub trait Traversal<S, E> {
	fn bytecode(&self) -> &Bytecode;
}

lazy_static! {
	pub static ref __: AnonymousTraversalSource = AnonymousTraversalSource::new();
}
