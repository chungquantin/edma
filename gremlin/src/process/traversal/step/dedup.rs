use crate::structure::GValue;

pub struct DedupStep {
	params: Vec<GValue>,
}

impl DedupStep {
	fn new(params: Vec<GValue>) -> Self {
		DedupStep {
			params,
		}
	}
}

impl From<DedupStep> for Vec<GValue> {
	fn from(step: DedupStep) -> Self {
		step.params
	}
}

impl From<()> for DedupStep {
	fn from(_: ()) -> DedupStep {
		DedupStep::new(vec![])
	}
}

impl From<&str> for DedupStep {
	fn from(param: &str) -> DedupStep {
		DedupStep::new(vec![String::from(param).into()])
	}
}
