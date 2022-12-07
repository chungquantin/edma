use crate::{
	components::{MenuItem, RenderAbleComponent},
	config::Config,
	constants::Focus,
	events::EventState,
};
use anyhow::Result;
use tui::{
	backend::Backend,
	layout::{Constraint, Direction, Layout},
};

use crate::{
	components::{DatabaseTabComponent, HomeTabComponent, MenuContainerComponent},
	events::Key,
};

pub struct AppComponent<'a> {
	home: HomeTabComponent,
	database: DatabaseTabComponent<'a>,
	menu: MenuContainerComponent,
	focus: Focus,
	config: Config,
}

impl<'a> AppComponent<'a> {
	pub fn new(config: Config) -> Self {
		AppComponent {
			home: HomeTabComponent::new(config.clone()),
			database: DatabaseTabComponent::new(config.clone()),
			menu: MenuContainerComponent::new(config.clone()),
			focus: Focus::MenuContainer,
			config,
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
			MenuItem::Database => {
				self.database.render(f, mid, matches!(self.focus(), Focus::DatabaseTabBody))?
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
			Focus::DatabaseTabBody => {
				if self.database.event(key).await?.is_consumed() {
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
