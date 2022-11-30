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
