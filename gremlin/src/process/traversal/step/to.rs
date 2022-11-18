use crate::process::traversal::TraversalBuilder;
use crate::structure::{GValue, Vertex};

pub struct ToStep {
	params: Vec<GValue>,
}

impl ToStep {
	fn new(params: Vec<GValue>) -> Self {
		ToStep {
			params,
		}
	}
}

impl From<ToStep> for Vec<GValue> {
	fn from(step: ToStep) -> Self {
		step.params
	}
}

impl From<&str> for ToStep {
	fn from(param: &str) -> Self {
		ToStep::new(vec![param.into()])
	}
}

impl From<&Vertex> for ToStep {
	fn from(param: &Vertex) -> Self {
		ToStep::new(vec![param.into()])
	}
}

impl From<TraversalBuilder> for ToStep {
	fn from(param: TraversalBuilder) -> Self {
		ToStep::new(vec![param.bytecode.into()])
	}
}
