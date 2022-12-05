use crate::{
	components::{MenuItem, RenderAbleComponent},
	constants::Focus,
};
use anyhow::Result;
use tui::{
	backend::Backend,
	layout::{Constraint, Direction, Layout},
};

use crate::{
	components::{EventState, FileTabComponent, HomeTabComponent, MenuContainerComponent},
	events::Key,
};

pub struct AppComponent {
	home: HomeTabComponent,
	file: FileTabComponent,
	menu: MenuContainerComponent,
	focus: Focus,
}

impl AppComponent {
	pub fn new() -> Self {
		AppComponent {
			home: HomeTabComponent::new(),
			file: FileTabComponent::new(),
			menu: MenuContainerComponent::new(),
			focus: Focus::MenuContainer,
		}
	}

	pub fn render<B: Backend>(&self, f: &mut tui::Frame<B>) -> Result<()> {
		let window = f.size();
		let main_chunks = Layout::default()
			.direction(Direction::Vertical)
			.constraints(
				[Constraint::Length(3), Constraint::Min(2), Constraint::Length(3)].as_ref(),
			)
			.split(window);
		let (top, mid) = (main_chunks[0], main_chunks[1]);

		self.menu.render(f, top, matches!(self.focus(), Focus::MenuContainer))?;

		match self.menu.active_menu_item {
			MenuItem::Home => {
				self.home.render(f, mid, matches!(self.focus(), Focus::HomeTabBody))?
			}
			MenuItem::File => {
				self.file.render(f, mid, matches!(self.focus(), Focus::FileTabBody))?
			}
		};

		Ok(())
	}

	fn focus(&self) -> Focus {
		self.focus.clone()
	}

	pub async fn event(&mut self, key: Key) -> anyhow::Result<EventState> {
		if self.components_event(key).await?.is_consumed() {
			return Ok(EventState::Consumed);
		};

		if self.move_focus(key).await?.is_consumed() {
			return Ok(EventState::Consumed);
		};

		Ok(EventState::NotConsumed)
	}

	async fn components_event(&mut self, _key: Key) -> Result<EventState> {
		Ok(EventState::NotConsumed)
	}

	async fn move_focus(&mut self, key: Key) -> Result<EventState> {
		match self.focus {
			Focus::MenuContainer => {
				if self.menu.event(key).await?.is_consumed() {
					return Ok(EventState::Consumed);
				}

				if key == Key::Down {
					self.focus = self.menu.active_focus();
				}
			}
			Focus::HomeTabBody => {
				if key == Key::Up {
					self.focus = Focus::MenuContainer
				}
			}
			Focus::FileTabBody => {
				if self.file.event(key).await?.is_consumed() {
					return Ok(EventState::Consumed);
				}

				if key == Key::Up {
					self.focus = Focus::MenuContainer
				}
			}
		}
		Ok(EventState::NotConsumed)
	}
}
