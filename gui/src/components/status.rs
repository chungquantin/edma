use tui::{backend::Backend, layout::Rect, Frame};

use crate::config::Config;

use super::{container::render_container, RenderAbleComponent};

pub struct StatusComponent {
	config: Config,
}

impl StatusComponent {
	pub fn new(config: Config) -> Self {
		StatusComponent {
			config,
		}
	}
}
impl RenderAbleComponent for StatusComponent {
	fn render<B: Backend>(
		&self,
		f: &mut Frame<B>,
		rect: Rect,
		focused: bool,
	) -> Result<(), anyhow::Error> {
		f.render_widget(render_container("Status", focused), rect);
		Ok(())
	}
}
