use anyhow::Result;
use tui::{
	backend::Backend,
	layout::{Constraint, Direction, Layout, Rect},
	Frame,
};

use crate::{
	config::Config,
	events::{EventState, Key},
};

use super::{render_container, RenderAbleComponent};

pub struct LayoutTabComponent {
	config: Config,
}

impl LayoutTabComponent {
	pub fn new(config: Config) -> Self {
		LayoutTabComponent {
			config,
		}
	}

	pub async fn event(&mut self, _key: Key) -> Result<EventState> {
		Ok(EventState::NotConsumed)
	}
}

impl RenderAbleComponent for LayoutTabComponent {
	fn render<B: Backend>(
		&self,
		f: &mut Frame<B>,
		rect: Rect,
		focused: bool,
	) -> Result<(), anyhow::Error> {
		let main_chunks = Layout::default()
			.direction(Direction::Horizontal)
			.constraints([Constraint::Percentage(20), Constraint::Percentage(80)])
			.split(rect);

		f.render_widget(render_container("Templates", focused), main_chunks[0]);
		f.render_widget(render_container("Layout", focused), main_chunks[1]);
		Ok(())
	}
}
