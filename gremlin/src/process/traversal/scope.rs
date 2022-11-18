#[derive(Debug, PartialEq, Eq, Clone)]
pub enum Scope {
	Global,
	Local,
}

impl Into<Scope> for () {
	fn into(self) -> Scope {
		Scope::Global
	}
}
