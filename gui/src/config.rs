use crate::utils::{ByteLayout, LayoutTemplate, LayoutVariant};

#[derive(Clone)]
pub struct Config {
	pub databases: Vec<String>,
	pub layouts: Vec<LayoutTemplate>,
}

fn build_template(name: &str, variant: LayoutVariant) -> LayoutTemplate {
	LayoutTemplate::new(
		&format!("SYSTEM:{}", name),
		vec![ByteLayout::default().with_variant(variant).build()],
	)
}

impl Config {
	pub fn new() -> Self {
		let layouts = vec![
			build_template("Bytes", LayoutVariant::Bytes),
			build_template("String", LayoutVariant::String),
			build_template("Int32", LayoutVariant::Int32),
			build_template("Int64", LayoutVariant::Int64),
			build_template("Float32", LayoutVariant::Float32),
			build_template("Float64", LayoutVariant::Float64),
			build_template("Boolean", LayoutVariant::Boolean),
		];
		Config {
			databases: vec!["rocksdb:./temp".to_string(), "rocksdb:./temp/v2".to_string()],
			layouts,
		}
	}
}
