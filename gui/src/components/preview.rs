use anyhow::Result;
use db::KeyValuePair;
use tui::{backend::Backend, layout::Rect, Frame};

use crate::{
	config::Config,
	events::{EventState, Key},
};

use super::{container::render_container, RenderAbleComponent};

pub struct PreviewComponent {
	config: Config,
	pair: Option<KeyValuePair>,
}

impl PreviewComponent {
	pub fn new(config: Config) -> Self {
		PreviewComponent {
			config,
			pair: None,
		}
	}

	pub async fn _event(&mut self, key: Key) -> Result<EventState> {
		Ok(EventState::NotConsumed)
	}
}

impl RenderAbleComponent for PreviewComponent {
	fn render<B: Backend>(
		&self,
		f: &mut Frame<B>,
		rect: Rect,
		focused: bool,
	) -> Result<(), anyhow::Error> {
		f.render_widget(render_container("Preview", focused), rect);
		Ok(())
	}
}
