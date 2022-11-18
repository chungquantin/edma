use crate::process::traversal::TraversalBuilder;
use crate::structure::GValue;

pub struct LocalStep {
	params: Vec<GValue>,
}

impl LocalStep {
	fn new(params: Vec<GValue>) -> Self {
		LocalStep {
			params,
		}
	}
}

impl From<LocalStep> for Vec<GValue> {
	fn from(step: LocalStep) -> Self {
		step.params
	}
}

impl From<TraversalBuilder> for LocalStep {
	fn from(param: TraversalBuilder) -> LocalStep {
		LocalStep::new(vec![param.bytecode.into()])
	}
}
