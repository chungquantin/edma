use anyhow::Result;
use tui::{
	backend::Backend,
	layout::{Constraint, Direction, Layout},
};

use crate::{
	components::{
		EventState, FileTabComponent, HomeTabComponent, MenuContainerComponent, RenderAbleComponent,
	},
	events::Key,
	MenuItem,
};

pub struct AppComponent {
	home: HomeTabComponent,
	file: FileTabComponent,
	menu: MenuContainerComponent,
}

const DEFAULT_ACTIVE_TAB: MenuItem = MenuItem::Home;

impl AppComponent {
	pub fn new() -> Self {
		AppComponent {
			home: HomeTabComponent::new(),
			file: FileTabComponent::new(),
			menu: MenuContainerComponent::new(DEFAULT_ACTIVE_TAB),
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

		self.menu.render(f, top, false)?;

		match self.menu.active_menu_item {
			MenuItem::Home => self.home.render(f, mid, false)?,
			MenuItem::File => self.file.render(f, mid, false)?,
		};

		Ok(())
	}

	pub async fn event(&mut self, key: Key) -> anyhow::Result<EventState> {
		if self.components_event(key).await?.is_consumed() {
			return Ok(EventState::Consumed);
		};

		if self.move_focus(key)?.is_consumed() {
			return Ok(EventState::Consumed);
		};

		Ok(EventState::NotConsumed)
	}

	async fn components_event(&mut self, key: Key) -> Result<EventState> {
		if self.menu.event(key).await?.is_consumed() {
			return Ok(EventState::Consumed);
		}
		Ok(EventState::NotConsumed)
	}

	fn move_focus(&mut self, _key: Key) -> Result<EventState> {
		Ok(EventState::NotConsumed)
	}
}
