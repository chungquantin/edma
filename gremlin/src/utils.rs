use crate::{conversion::BorrowFromGValue, GremlinError, List, Map};

pub fn unwrap_map<'a, T>(map: &'a Map, key: &str, index: usize) -> Result<&'a T, GremlinError>
where
	T: BorrowFromGValue,
{
	match key {
		"id" | "label" => map[key].get::<T>(),
		_ => map[key].get::<List>().unwrap()[index].get::<T>(),
	}
}
