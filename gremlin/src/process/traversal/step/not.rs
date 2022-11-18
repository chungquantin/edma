use crate::process::traversal::TraversalBuilder;
use crate::structure::GValue;

pub struct NotStep {
	params: Vec<GValue>,
}

impl NotStep {
	fn new(params: Vec<GValue>) -> Self {
		NotStep {
			params,
		}
	}
}

impl From<NotStep> for Vec<GValue> {
	fn from(step: NotStep) -> Self {
		step.params
	}
}

impl From<TraversalBuilder> for NotStep {
	fn from(param: TraversalBuilder) -> Self {
		NotStep::new(vec![param.bytecode.into()])
	}
}
