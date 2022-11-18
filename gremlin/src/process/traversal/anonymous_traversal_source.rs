use crate::process::traversal::step::has::HasStep;
use crate::process::traversal::step::loops::LoopsStep;
use crate::process::traversal::step::not::NotStep;
use crate::process::traversal::step::or::OrStep;
use crate::process::traversal::step::repeat::RepeatStep;
use crate::process::traversal::step::select::SelectStep;
use crate::process::traversal::step::until::UntilStep;
use crate::process::traversal::step::where_step::WhereStep;
use crate::process::traversal::TraversalBuilder;
use crate::structure::{Either2, GIDs, IntoPredicate, Labels, T};
use crate::GValue;

pub struct AnonymousTraversalSource {
	traversal: TraversalBuilder,
}

impl AnonymousTraversalSource {
	pub fn new() -> AnonymousTraversalSource {
		AnonymousTraversalSource {
			traversal: TraversalBuilder::default(),
		}
	}

	pub fn in_v(&self) -> TraversalBuilder {
		self.traversal.clone().in_v()
	}

	pub fn aggregate<A>(&self, alias: A) -> TraversalBuilder
	where
		A: Into<String>,
	{
		self.traversal.clone().aggregate(alias)
	}

	pub fn add_v<A>(&self, label: A) -> TraversalBuilder
	where
		A: Into<Labels>,
	{
		self.traversal.clone().add_v(label)
	}

	pub fn property<A>(&self, key: Either2<&str, T>, value: A) -> TraversalBuilder
	where
		A: Into<GValue>,
	{
		self.traversal.clone().property(key, value)
	}

	pub fn v<T>(&self, ids: T) -> TraversalBuilder
	where
		T: Into<GIDs>,
	{
		self.traversal.clone().v(ids)
	}

	pub fn add_e<A>(&self, label: A) -> TraversalBuilder
	where
		A: Into<String>,
	{
		self.traversal.clone().add_e(label)
	}

	pub fn count(&self) -> TraversalBuilder {
		self.traversal.clone().count()
	}

	pub fn out<L>(&self, labels: L) -> TraversalBuilder
	where
		L: Into<Labels>,
	{
		self.traversal.clone().out(labels)
	}

	pub fn out_e<L>(&self, labels: L) -> TraversalBuilder
	where
		L: Into<Labels>,
	{
		self.traversal.clone().out_e(labels)
	}

	pub fn in_<L>(&self, labels: L) -> TraversalBuilder
	where
		L: Into<Labels>,
	{
		self.traversal.clone().in_(labels)
	}

	pub fn in_e<L>(&self, labels: L) -> TraversalBuilder
	where
		L: Into<Labels>,
	{
		self.traversal.clone().in_e(labels)
	}

	pub fn both<L>(&self, labels: L) -> TraversalBuilder
	where
		L: Into<Labels>,
	{
		self.traversal.clone().both(labels)
	}

	pub fn both_e<L>(&self, labels: L) -> TraversalBuilder
	where
		L: Into<Labels>,
	{
		self.traversal.clone().both_e(labels)
	}

	pub fn other(&self) -> TraversalBuilder {
		self.traversal.clone().other()
	}

	pub fn other_v(&self) -> TraversalBuilder {
		self.traversal.clone().other_v()
	}

	pub fn values<L>(&self, labels: L) -> TraversalBuilder
	where
		L: Into<Labels>,
	{
		self.traversal.clone().values(labels)
	}
	pub fn has_label<L>(&self, labels: L) -> TraversalBuilder
	where
		L: Into<Labels>,
	{
		self.traversal.clone().has_label(labels)
	}

	pub fn as_<A>(&self, alias: A) -> TraversalBuilder
	where
		A: Into<String>,
	{
		self.traversal.clone().as_(alias)
	}

	pub fn has<A>(&self, step: A) -> TraversalBuilder
	where
		A: Into<HasStep>,
	{
		self.traversal.clone().has(step)
	}

	pub fn has_many<A>(&self, steps: Vec<A>) -> TraversalBuilder
	where
		A: Into<HasStep>,
	{
		self.traversal.clone().has_many(steps)
	}

	pub fn not<A>(&self, step: A) -> TraversalBuilder
	where
		A: Into<NotStep>,
	{
		self.traversal.clone().not(step)
	}

	pub fn loops<A>(&self, step: A) -> TraversalBuilder
	where
		A: Into<LoopsStep>,
	{
		self.traversal.clone().loops(step)
	}

	pub fn select<A>(&self, step: A) -> TraversalBuilder
	where
		A: Into<SelectStep>,
	{
		self.traversal.clone().select(step)
	}

	pub fn fold(&self) -> TraversalBuilder {
		self.traversal.clone().fold()
	}

	pub fn unfold(&self) -> TraversalBuilder {
		self.traversal.clone().unfold()
	}

	pub fn out_v(&self) -> TraversalBuilder {
		self.traversal.clone().out_v()
	}

	pub fn is<A>(&self, val: A) -> TraversalBuilder
	where
		A: IntoPredicate,
	{
		self.traversal.clone().is(val)
	}

	pub fn or<A>(&self, step: A) -> TraversalBuilder
	where
		A: Into<OrStep>,
	{
		self.traversal.clone().or(step)
	}

	pub fn where_<A>(&self, step: A) -> TraversalBuilder
	where
		A: Into<WhereStep>,
	{
		self.traversal.clone().where_(step)
	}

	pub fn cap(&self, step: &'static str) -> TraversalBuilder {
		self.traversal.clone().cap(step)
	}

	pub fn project<A>(&self, step: A) -> TraversalBuilder
	where
		A: Into<SelectStep>,
	{
		self.traversal.clone().project(step)
	}

	pub fn constant<A>(&self, value: A) -> TraversalBuilder
	where
		A: Into<GValue>,
	{
		self.traversal.clone().constant(value)
	}

	pub fn until<A>(&self, step: A) -> TraversalBuilder
	where
		A: Into<UntilStep>,
	{
		self.traversal.clone().until(step)
	}

	pub fn repeat<A>(&self, step: A) -> TraversalBuilder
	where
		A: Into<RepeatStep>,
	{
		self.traversal.clone().repeat(step)
	}

	pub fn emit(&self) -> TraversalBuilder {
		self.traversal.clone().emit()
	}
}

impl Default for AnonymousTraversalSource {
	fn default() -> Self {
		Self::new()
	}
}
