use crate::process::traversal::TraversalBuilder;
use crate::structure::GValue;

pub struct OrStep {
	params: Vec<GValue>,
}

impl OrStep {
	fn new(params: Vec<GValue>) -> Self {
		OrStep {
			params,
		}
	}
}

impl From<OrStep> for Vec<GValue> {
	fn from(step: OrStep) -> Self {
		step.params
	}
}

impl From<()> for OrStep {
	fn from(_: ()) -> Self {
		OrStep::new(vec![])
	}
}

impl From<TraversalBuilder> for OrStep {
	fn from(param: TraversalBuilder) -> Self {
		OrStep::new(vec![param.bytecode.into()])
	}
}

impl From<Vec<TraversalBuilder>> for OrStep {
	fn from(param: Vec<TraversalBuilder>) -> Self {
		OrStep::new(param.into_iter().map(|s| s.bytecode.into()).collect())
	}
}

macro_rules! impl_into_or {
	($n:expr) => {
		impl From<[TraversalBuilder; $n]> for OrStep {
			fn from(param: [TraversalBuilder; $n]) -> OrStep {
				OrStep::new(param.iter().map(|s| s.bytecode.clone().into()).collect())
			}
		}
	};
}

impl_into_or!(1);
impl_into_or!(2);
impl_into_or!(3);
impl_into_or!(4);
impl_into_or!(5);
impl_into_or!(6);
impl_into_or!(7);
impl_into_or!(8);
impl_into_or!(9);
impl_into_or!(10);
