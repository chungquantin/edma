use crate::structure::GValue;

pub struct LoopsStep {
	params: Vec<GValue>,
}

impl LoopsStep {
	fn new(params: Vec<GValue>) -> Self {
		LoopsStep {
			params,
		}
	}
}

impl From<LoopsStep> for Vec<GValue> {
	fn from(step: LoopsStep) -> Self {
		step.params
	}
}

impl From<()> for LoopsStep {
	fn from(_: ()) -> LoopsStep {
		LoopsStep::new(vec![])
	}
}

impl From<&str> for LoopsStep {
	fn from(param: &str) -> LoopsStep {
		LoopsStep::new(vec![param.into()])
	}
}

impl From<String> for LoopsStep {
	fn from(param: String) -> LoopsStep {
		LoopsStep::new(vec![param.into()])
	}
}
