use anyhow::Result;
use tui::{
	backend::Backend,
	layout::{Constraint, Direction, Layout, Rect},
	Frame,
};

use crate::{
	components::RenderAbleComponent,
	config::Config,
	events::{EventState, Key},
};

use super::{LayoutContentComponent, LayoutExplorerComponent};

enum Focus {
	Explorer,
	Layout,
}

pub struct LayoutTabComponent<'a> {
	config: Config,
	explorer: LayoutExplorerComponent<'a>,
	layout: LayoutContentComponent,
	focus: Focus,
}

impl LayoutTabComponent<'_> {
	pub fn new(config: Config) -> Self {
		LayoutTabComponent {
			explorer: LayoutExplorerComponent::new(config.clone()),
			layout: LayoutContentComponent::new(config.clone()),
			config,
			focus: Focus::Explorer,
		}
	}

	pub async fn event(&mut self, key: Key) -> Result<EventState> {
		match self.focus {
			Focus::Explorer => {
				if key == Key::Right {
					self.focus = Focus::Layout;
					return Ok(EventState::Consumed);
				}

				if self.explorer.event(key).await?.is_consumed() {
					if let Some(selected) = self.explorer.list.state.selected() {
						let template = self.config.layouts[selected].clone();
						self.layout.set_layout(Some(template));
					}
					return Ok(EventState::Consumed);
				}
				Ok(EventState::NotConsumed)
			}
			Focus::Layout => {
				if key == Key::Left {
					self.focus = Focus::Explorer;
					return Ok(EventState::Consumed);
				}

				if self.layout.event(key).await?.is_consumed() {
					return Ok(EventState::Consumed);
				}
				Ok(EventState::NotConsumed)
			}
		}
	}
}

impl RenderAbleComponent for LayoutTabComponent<'_> {
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

		self.explorer.render(
			f,
			main_chunks[0],
			focused && matches!(self.focus, Focus::Explorer),
		)?;

		self.layout.render(f, main_chunks[1], focused && matches!(self.focus, Focus::Layout))?;
		Ok(())
	}
}
