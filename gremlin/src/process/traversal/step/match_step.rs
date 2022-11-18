use crate::process::traversal::TraversalBuilder;
use crate::structure::GValue;

pub struct MatchStep {
	params: Vec<GValue>,
}

impl MatchStep {
	fn new(params: Vec<GValue>) -> Self {
		MatchStep {
			params,
		}
	}
}

impl From<MatchStep> for Vec<GValue> {
	fn from(step: MatchStep) -> Self {
		step.params
	}
}

impl From<TraversalBuilder> for MatchStep {
	fn from(param: TraversalBuilder) -> MatchStep {
		MatchStep::new(vec![param.bytecode.into()])
	}
}

impl From<Vec<TraversalBuilder>> for MatchStep {
	fn from(param: Vec<TraversalBuilder>) -> MatchStep {
		MatchStep::new(param.into_iter().map(|s| s.bytecode.into()).collect())
	}
}

macro_rules! impl_into_match {
	($n:expr) => {
		impl From<[TraversalBuilder; $n]> for MatchStep {
			fn from(param: [TraversalBuilder; $n]) -> MatchStep {
				MatchStep::new(param.iter().map(|s| s.bytecode.clone().into()).collect())
			}
		}
	};
}

impl_into_match!(1);
impl_into_match!(2);
impl_into_match!(3);
impl_into_match!(4);
impl_into_match!(5);
impl_into_match!(6);
impl_into_match!(7);
impl_into_match!(8);
impl_into_match!(9);
impl_into_match!(10);
