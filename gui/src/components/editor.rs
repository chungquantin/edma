use db::{Datastore, KeyValuePair, SimpleTransaction};
use tui::{backend::Backend, layout::Rect, Frame};

use crate::config::Config;

use super::{container::render_container, RenderAbleComponent};

pub struct DatabaseEditorComponent {
	config: Config,
	pairs: Vec<KeyValuePair>,
}

async fn scan_from_path(path: &str) -> Vec<KeyValuePair> {
	let mut result = vec![];
	let ds = Datastore::new(path);
	let tx = ds.transaction(false).unwrap();
	let data = tx.iterate(None).await.unwrap();
	for pair in data {
		result.push(pair.unwrap());
	}
	result
}

impl DatabaseEditorComponent {
	pub async fn scan_database(&mut self, name: &str, path: &str) {
		let db_path = format!("{}:{}", name.to_string(), path.to_string());
		self.pairs = scan_from_path(&db_path).await;
	}

	fn generate_label(&self) -> String {
		format!("Editor ({} key-value pairs)", self.pairs.len().to_string())
	}

	pub fn new(config: Config) -> Self {
		DatabaseEditorComponent {
			config,
			pairs: vec![],
		}
	}
}
impl RenderAbleComponent for DatabaseEditorComponent {
	fn render<B: Backend>(
		&self,
		f: &mut Frame<B>,
		rect: Rect,
		focused: bool,
	) -> Result<(), anyhow::Error> {
		let label = self.generate_label();
		f.render_widget(render_container(&label, focused), rect);
		Ok(())
	}
}
