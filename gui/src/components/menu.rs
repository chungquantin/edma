use anyhow::Result;
use tui::{
	backend::Backend,
	layout::Rect,
	style::{Color, Modifier, Style},
	text::{Span, Spans},
	widgets::Tabs,
	Frame,
};

use crate::{
	config::Config,
	constants::{Focus, HIGHLIGHT_COLOR},
	events::{EventState, Key},
};

use super::{container::render_container, RenderAbleComponent};

#[derive(Copy, Clone, Debug)]
pub enum MenuItem {
	Home,
	Database,
	Layout,
}

impl From<MenuItem> for usize {
	fn from(input: MenuItem) -> usize {
		match input {
			MenuItem::Home => 0,
			MenuItem::Database => 1,
			MenuItem::Layout => 2,
		}
	}
}

pub struct MenuContainerComponent {
	pub active_menu_item: MenuItem,
	config: Config,
}

const DEFAULT_ACTIVE_TAB: MenuItem = MenuItem::Home;

impl MenuContainerComponent {
	pub fn new(config: Config) -> Self {
		MenuContainerComponent {
			active_menu_item: DEFAULT_ACTIVE_TAB,
			config,
		}
	}

	pub fn active_focus(&self) -> Focus {
		match self.active_menu_item {
			MenuItem::Home => Focus::HomeTabBody,
			MenuItem::Database => Focus::DatabaseTabBody,
			MenuItem::Layout => Focus::LayoutTabBody,
		}
	}

	pub fn set_active(&mut self, active_menu_item: MenuItem) {
		self.active_menu_item = active_menu_item;
	}

	pub async fn event(&mut self, key: Key) -> Result<EventState> {
		if key == Key::Char('h') || key == Key::Char('H') {
			self.set_active(MenuItem::Home);
			return Ok(EventState::Consumed);
		}
		if key == Key::Char('d') || key == Key::Char('D') {
			self.set_active(MenuItem::Database);
			return Ok(EventState::Consumed);
		}
		if key == Key::Char('l') || key == Key::Char('L') {
			self.set_active(MenuItem::Layout);
			return Ok(EventState::Consumed);
		}
		Ok(EventState::NotConsumed)
	}
}

impl RenderAbleComponent for MenuContainerComponent {
	fn render<B: Backend>(
		&self,
		f: &mut Frame<B>,
		area: Rect,
		focused: bool,
	) -> Result<(), anyhow::Error> {
		let menu_titles = vec!["EDMA", "Home", "Database", "Layout", "Quit"];

		let menu = menu_titles
			.iter()
			.enumerate()
			.map(|(index, t)| {
				if index == 0 {
					Spans::from(vec![Span::styled(*t, Style::default().fg(HIGHLIGHT_COLOR))])
				} else {
					let (first, rest) = t.split_at(1);
					Spans::from(vec![
						Span::styled(
							first,
							Style::default().fg(HIGHLIGHT_COLOR).add_modifier(Modifier::UNDERLINED),
						),
						Span::styled(rest, Style::default().fg(Color::White)),
					])
				}
			})
			.collect();

		let tabs = Tabs::new(menu)
			.select(self.active_menu_item.into())
			.block(render_container("Menu", focused))
			.divider(Span::raw("|"));

		f.render_widget(tabs, area);
		Ok(())
	}
}
