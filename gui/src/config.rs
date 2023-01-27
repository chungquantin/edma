use std::{collections::HashMap, fs, path::{Path, PathBuf}};

use serde_json::Value;
use structopt::StructOpt;

use crate::{
	events::Key,
	utils::{get_absolute_path_buf, sanitize, ByteLayout, LayoutTemplate, LayoutVariant},
};

#[derive(Clone, Debug)]
pub struct DatabaseConfig {
	pub path: String,
}

#[derive(StructOpt, Debug)]
pub struct CliConfig {
	/// Set the config file)
	#[structopt(long, short, global = true, default_value="./edma.json")]
	config_path: std::path::PathBuf,
	/// Create a new example config file
	#[structopt(long, global = true)]
	create_config: bool,
}

#[derive(Clone, Debug)]
pub struct KeyConfig {
	pub enter: Key,
	pub backspace: Key,
	pub escape: Key,
	pub up: Key,
	pub down: Key,
	pub left: Key,
	pub right: Key,
	pub key_layout_up: Key,
	pub key_layout_down: Key,
	pub value_layout_up: Key,
	pub value_layout_down: Key,
	pub database_select_up: Key,
	pub database_select_down: Key,
	pub home_tab: Key,
	pub database_tab: Key,
	pub layout_tab: Key,
	pub quit: Key,
}

#[derive(Clone, Debug)]
pub struct Config {
	pub databases: HashMap<String, Vec<DatabaseConfig>>,
	pub templates: Vec<LayoutTemplate>,
	pub path: String,
	pub key_config: KeyConfig,
}

fn build_template(name: &str, variant: LayoutVariant) -> LayoutTemplate {
	LayoutTemplate::new(
		&format!("SYSTEM:{}", name),
		vec![ByteLayout::default().with_variant(variant).build()],
	)
}

impl Config {
	pub fn new(config: &CliConfig) -> Self {
		Config {
			databases: Default::default(),
			path: get_absolute_path_buf(config.config_path.to_path_buf()),
			templates: Default::default(),
			key_config: KeyConfig {
				backspace: Key::Backspace,
				enter: Key::Enter,
				escape: Key::Esc,
				up: Key::Up,
				down: Key::Down,
				left: Key::Left,
				right: Key::Right,
				key_layout_up: Key::Char('h'),
				key_layout_down: Key::Char('j'),
				value_layout_up: Key::Char('k'),
				value_layout_down: Key::Char('l'),
				database_select_up: Key::Char('9'),
				database_select_down: Key::Char('0'),
				home_tab: Key::Char('h'),
				database_tab: Key::Char('d'),
				layout_tab: Key::Char('l'),
				quit: Key::Char('q'),
			},
		}
	}

	pub fn set_databases(&mut self, databases: HashMap<String, Vec<DatabaseConfig>>) {
		self.databases = databases;
	}

	pub fn set_layouts(&mut self, layouts: Vec<LayoutTemplate>) {
		self.templates = layouts;
	}
}

fn create_config_example(path: &PathBuf) {
	let config_example = include_str!("../config-example.json");
	if !path.exists() {
		println!("Creating config file: {}", path.to_str().unwrap());
		fs::write(path, config_example).unwrap();
	} else {
		panic!("Config file already exists: {}", path.to_str().unwrap());
	}
}

pub fn load_config(cli: &CliConfig) -> Config {
	let mut config = Config::new(cli);
	if cli.create_config {
		create_config_example(&cli.config_path);
	}
	let Ok(data) = fs::read_to_string(&config.path) else {
		panic!("Unable to read config file: {}. Run with `--create-config` to create an example config file", config.path)
	};
	let res: serde_json::Value = serde_json::from_str(&data).expect("Unable to parse");

	if let Some(d) = res.get("databases") {
		let databases = load_databases(d);
		config.set_databases(databases);
	}

	if let Some(t) = res.get("templates") {
		let system_templates = vec![
			build_template("Bytes", LayoutVariant::Bytes),
			build_template("String", LayoutVariant::String),
			build_template("Int32", LayoutVariant::Int32),
			build_template("Int64", LayoutVariant::Int64),
			build_template("Float32", LayoutVariant::Float32),
			build_template("Float64", LayoutVariant::Float64),
			build_template("Boolean", LayoutVariant::Boolean),
		];
		let templates = load_templates(t);
		config.set_layouts([system_templates, templates].concat());
	}

	config
}

/// Load byte layout template from JSON config file
fn load_templates(json_templates: &Value) -> Vec<LayoutTemplate> {
	let templates = json_templates.as_array();
	let mut layout_templates = Vec::<LayoutTemplate>::new();
	for template in templates.unwrap().iter() {
		let mut t = LayoutTemplate::default();
		let name = sanitize(&template.get("name").unwrap().to_string());
		t.set_name(&name);
		let layouts = template.get("layouts").unwrap().as_array();
		// Load layout from json template
		for layout in layouts.unwrap().iter() {
			let mut l = ByteLayout::default();
			let name = sanitize(&layout.get("name").unwrap().to_string());
			let variant = sanitize(&layout.get("variant").unwrap().to_string());
			let from = layout.get("from").unwrap().as_i64().unwrap() as usize;
			let to = layout.get("to").unwrap().as_i64().unwrap() as usize;
			let variant = LayoutVariant::from_string(&variant);
			t.push_layout(l.with_name(name).with_variant(variant).with_range(from, to).build());
		}
		layout_templates.push(t);
	}

	layout_templates
}

/// Load databases from JSON config file
fn load_databases(json_database: &Value) -> HashMap<String, Vec<DatabaseConfig>> {
	let databases = json_database.as_array();
	let mut databases_config = HashMap::<String, Vec<DatabaseConfig>>::new();
	for database in databases.unwrap().iter() {
		let path = sanitize(&database.get("path").unwrap().to_string());
		let name = sanitize(&database.get("name").unwrap().to_string());
		databases_config.entry(name).or_default().push(DatabaseConfig {
			path,
		});
	}

	databases_config
}
