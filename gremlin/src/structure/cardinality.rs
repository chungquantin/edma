#[derive(Debug, PartialEq, Eq, Clone)]
pub enum Cardinality {
	List,
	Set,
	Single,
}
