/// A string that must be less than 256 characters long, and can only contain
/// letters, numbers, dashes and underscores. This is used for vertex and edge
/// types, as well as property names.
#[derive(Eq, PartialEq, Clone, Debug, Hash, Ord, PartialOrd)]
pub struct Identifier(pub(crate) String);
