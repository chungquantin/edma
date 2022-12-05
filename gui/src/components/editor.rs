use tui::{backend::Backend, layout::Rect, Frame};

use crate::config::Config;

use super::{container::render_container, RenderAbleComponent};

pub struct EditorComponent {
	config: Config,
}

impl EditorComponent {
	pub fn new(config: Config) -> Self {
		EditorComponent {
			config,
		}
	}
}
impl RenderAbleComponent for EditorComponent {
	fn render<B: Backend>(
		&self,
		f: &mut Frame<B>,
		rect: Rect,
		focused: bool,
	) -> Result<(), anyhow::Error> {
		f.render_widget(render_container("Database", focused), rect);
		Ok(())
	}
}
