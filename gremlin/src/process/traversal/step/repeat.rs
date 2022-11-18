use crate::process::traversal::TraversalBuilder;
use crate::structure::GValue;

pub struct RepeatStep {
	params: Vec<GValue>,
}

impl RepeatStep {
	fn new(params: Vec<GValue>) -> Self {
		RepeatStep {
			params,
		}
	}
}

impl From<RepeatStep> for Vec<GValue> {
	fn from(step: RepeatStep) -> Self {
		step.params
	}
}

impl From<TraversalBuilder> for RepeatStep {
	fn from(param: TraversalBuilder) -> RepeatStep {
		RepeatStep::new(vec![param.bytecode.into()])
	}
}
