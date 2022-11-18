use crate::structure::GValue;
use crate::structure::{Either2, TextP, T};
use crate::structure::{IntoPredicate, P};

pub enum HasStepKey {
	Str(String),
	T(T),
}

impl Into<HasStepKey> for T {
	fn into(self) -> HasStepKey {
		HasStepKey::T(self)
	}
}

impl Into<HasStepKey> for String {
	fn into(self) -> HasStepKey {
		HasStepKey::Str(self)
	}
}

impl Into<HasStepKey> for &str {
	fn into(self) -> HasStepKey {
		HasStepKey::Str(String::from(self))
	}
}

pub struct HasStep {
	label: Option<String>,
	key: HasStepKey,
	predicate: Option<Either2<P, TextP>>,
}

impl From<HasStep> for Vec<GValue> {
	fn from(step: HasStep) -> Self {
		let mut params: Vec<GValue> = vec![];

		if let Some(s) = step.label {
			params.push(Into::into(s));
		}

		match step.key {
			HasStepKey::Str(key) => params.push(Into::into(key)),
			HasStepKey::T(key) => params.push(Into::into(key)),
		};

		if let Some(p) = step.predicate {
			params.push(Into::into(p));
		}

		params
	}
}

impl<A, B> From<(A, B)> for HasStep
where
	A: Into<HasStepKey>,
	B: IntoPredicate,
{
	fn from(param: (A, B)) -> Self {
		HasStep {
			label: None,
			key: param.0.into(),
			predicate: Some(param.1.into_predicate()),
		}
	}
}

impl<A, B, C> From<(A, B, C)> for HasStep
where
	A: Into<String>,
	B: Into<HasStepKey>,
	C: IntoPredicate,
{
	fn from(param: (A, B, C)) -> Self {
		HasStep {
			label: Some(param.0.into()),
			key: param.1.into(),
			predicate: Some(param.2.into_predicate()),
		}
	}
}

impl From<String> for HasStep {
	fn from(param: String) -> Self {
		HasStep {
			label: None,
			key: HasStepKey::Str(param),
			predicate: None,
		}
	}
}

impl From<&str> for HasStep {
	fn from(param: &str) -> Self {
		HasStep {
			label: None,
			key: HasStepKey::Str(String::from(param)),
			predicate: None,
		}
	}
}
