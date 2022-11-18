use crate::process::traversal::Scope;
use crate::structure::GValue;

pub struct LimitStep {
	limit: GValue,
	scope: Option<Scope>,
}

impl LimitStep {
	fn new(limit: GValue, scope: Option<Scope>) -> Self {
		LimitStep {
			limit,
			scope,
		}
	}
}

impl From<LimitStep> for Vec<GValue> {
	fn from(step: LimitStep) -> Self {
		let mut params = step
			.scope
			.map(|m| match m {
				Scope::Global => vec![String::from("Global").into()],
				Scope::Local => vec![String::from("Local").into()],
			})
			.unwrap_or_else(Vec::new);

		params.push(step.limit);
		params
	}
}

impl From<i64> for LimitStep {
	fn from(param: i64) -> LimitStep {
		LimitStep::new(param.into(), None)
	}
}
