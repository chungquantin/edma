use crate::process::traversal::{Order, TraversalBuilder};
use crate::structure::{GValue, T};

pub struct ByStep {
	params: Vec<GValue>,
}

impl ByStep {
	fn new(params: Vec<GValue>) -> Self {
		ByStep {
			params,
		}
	}
}

impl From<ByStep> for Vec<GValue> {
	fn from(step: ByStep) -> Self {
		step.params
	}
}

impl From<()> for ByStep {
	fn from(_: ()) -> Self {
		ByStep::new(vec![])
	}
}

impl From<&str> for ByStep {
	fn from(param: &str) -> Self {
		ByStep::new(vec![String::from(param).into()])
	}
}

impl From<Order> for ByStep {
	fn from(param: Order) -> Self {
		ByStep::new(vec![param.into()])
	}
}

impl From<T> for ByStep {
	fn from(param: T) -> Self {
		ByStep::new(vec![param.into()])
	}
}

impl From<(&str, Order)> for ByStep {
	fn from(param: (&str, Order)) -> Self {
		ByStep::new(vec![param.0.into(), param.1.into()])
	}
}

impl From<(String, Order)> for ByStep {
	fn from(param: (String, Order)) -> Self {
		ByStep::new(vec![param.0.into(), param.1.into()])
	}
}

impl From<(TraversalBuilder, Order)> for ByStep {
	fn from(param: (TraversalBuilder, Order)) -> Self {
		ByStep::new(vec![param.0.bytecode.into(), param.1.into()])
	}
}

impl From<TraversalBuilder> for ByStep {
	fn from(param: TraversalBuilder) -> Self {
		ByStep::new(vec![param.bytecode.into()])
	}
}
