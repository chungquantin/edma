use std::cmp::Ordering;

use solomon_gremlin::{structure::Predicate, FromGValue, GValue, List, Vertex};

pub fn is_vertex_step(s: &str) -> bool {
	s == "V" || s == "addV"
}

pub fn is_edge_step(s: &str) -> bool {
	s == "E" || s == "addE"
}

pub fn is_source_step(s: &str) -> bool {
	is_edge_step(s) || is_vertex_step(s)
}

/// # ReducingBarrierStep()
/// All of the traversers prior to the step are processed by a reduce function and once
/// all the previous traversers are processed, a single "reduced value" traverser is emitted
/// to the next step. Note that the path history leading up to a reducing barrier step is
/// destroyed given its many-to-one nature.
pub fn is_reducing_barrier_step(s: &str) -> bool {
	s == "fold" || s == "count" || s == "sum" || s == "max" || s == "min"
}

pub fn is_has_key_predicate(a: &[GValue]) -> bool {
	a.len() >= 2 && a[0].to_variant() == 1 && a[1].to_variant() == 13
}

pub fn is_has_label_key(a: &[GValue]) -> bool {
	a.len() >= 2 && a[0].to_variant() == 1 && a[1].to_variant() == 1
}

pub fn is_has_label_key_predicate(a: &[GValue]) -> bool {
	a.len() >= 3 && is_has_label_key(a) && a[2].to_variant() == 13
}

pub fn has_key_predicate_vertex(v: &Vertex, args: &[GValue]) -> bool {
	let key = String::from_gvalue(args[0].clone()).unwrap();
	let predicate = Predicate::from_gvalue(args[1].clone()).unwrap();
	match predicate.operator().as_str() {
		"within" => {
			let input = predicate.value().get::<List>().unwrap();
			input.iter().any(|i| {
				v.properties()
					.get(&key)
					.unwrap()
					.iter()
					.any(|p| p.value().partial_cmp(i).unwrap() == Ordering::Equal)
			})
		}
		_ => unimplemented!(),
	}
}
