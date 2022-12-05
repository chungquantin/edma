use crate::{
	components::{MenuItem, RenderAbleComponent},
	config::Config,
	constants::Focus,
};
use anyhow::Result;
use db::{Datastore, SimpleTransaction};
use tui::{
	backend::Backend,
	layout::{Constraint, Direction, Layout},
};

use crate::{
	components::{EventState, FileTabComponent, HomeTabComponent, MenuContainerComponent},
	events::Key,
};

pub struct AppComponent<'a> {
	home: HomeTabComponent,
	file: FileTabComponent<'a>,
	menu: MenuContainerComponent,
	focus: Focus,
	config: Config,
}

impl<'a> AppComponent<'a> {
	pub fn new(config: Config) -> Self {
		AppComponent {
			home: HomeTabComponent::new(config.clone()),
			file: FileTabComponent::new(config.clone()),
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

	async fn scan_storage(path: &str) -> Vec<(Vec<u8>, Vec<u8>)> {
		let mut result = vec![];
		let ds = Datastore::new(path);
		let tx = ds.transaction(false).unwrap();
		let data = tx.iterate(None).await.unwrap();
		for pair in data {
			result.push(pair.unwrap());
		}
		result
	}
}