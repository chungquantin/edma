use crate::process::traversal::TraversalBuilder;
use crate::structure::GValue;

pub trait IntoChooseStep {
	fn into_step(self) -> Vec<GValue>;
}

impl IntoChooseStep for TraversalBuilder {
	fn into_step(self) -> Vec<GValue> {
		vec![self.bytecode.into()]
	}
}

impl IntoChooseStep for (TraversalBuilder, TraversalBuilder) {
	fn into_step(self) -> Vec<GValue> {
		let mut out = vec![];
		out.append(&mut vec![self.0.bytecode.into()]);
		out.append(&mut vec![self.1.bytecode.into()]);
		out
	}
}

impl IntoChooseStep for (TraversalBuilder, TraversalBuilder, TraversalBuilder) {
	fn into_step(self) -> Vec<GValue> {
		let mut out = vec![];
		out.append(&mut vec![self.0.bytecode.into()]);
		out.append(&mut vec![self.1.bytecode.into()]);
		out.append(&mut vec![self.2.bytecode.into()]);
		out
	}
}
