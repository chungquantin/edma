use crate::process::traversal::TraversalBuilder;
use crate::structure::GValue;
use crate::structure::IntoPredicate;

pub struct WhereStep {
	params: Vec<GValue>,
}

impl WhereStep {
	fn new(params: Vec<GValue>) -> Self {
		WhereStep {
			params,
		}
	}
}

impl From<WhereStep> for Vec<GValue> {
	fn from(step: WhereStep) -> Self {
		step.params
	}
}

impl From<TraversalBuilder> for WhereStep {
	fn from(param: TraversalBuilder) -> WhereStep {
		WhereStep::new(vec![param.bytecode.into()])
	}
}

impl<A, B> From<(A, B)> for WhereStep
where
	A: Into<String>,
	B: IntoPredicate,
{
	fn from(param: (A, B)) -> WhereStep {
		WhereStep::new(vec![param.0.into().into(), param.1.into_predicate().into()])
	}
}

impl<A> From<A> for WhereStep
where
	A: IntoPredicate,
{
	fn from(param: A) -> WhereStep {
		WhereStep::new(vec![param.into_predicate().into()])
	}
}
