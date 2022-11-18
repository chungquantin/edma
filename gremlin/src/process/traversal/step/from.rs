use crate::process::traversal::TraversalBuilder;
use crate::structure::{GValue, Vertex};

pub struct FromStep {
	params: Vec<GValue>,
}

impl FromStep {
	fn new(params: Vec<GValue>) -> Self {
		FromStep {
			params,
		}
	}
}

impl From<FromStep> for Vec<GValue> {
	fn from(step: FromStep) -> Self {
		step.params
	}
}

impl From<&str> for FromStep {
	fn from(param: &str) -> Self {
		FromStep::new(vec![param.into()])
	}
}

impl From<&Vertex> for FromStep {
	fn from(param: &Vertex) -> Self {
		FromStep::new(vec![param.into()])
	}
}

impl From<TraversalBuilder> for FromStep {
	fn from(param: TraversalBuilder) -> Self {
		FromStep::new(vec![param.bytecode.into()])
	}
}
