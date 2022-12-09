use crate::{
	components::{render_container, LayoutTabComponent, MenuItem, RenderAbleComponent},
	config::Config,
	constants::{Focus, NO_DATABASES_BANNER},
	events::EventState,
};
use anyhow::Result;
use tui::{
	backend::Backend,
	layout::{Alignment, Constraint, Direction, Layout},
	style::{Color, Style},
	text::{Span, Spans, Text},
	widgets::{Block, Paragraph},
};

use crate::{
	components::{DatabaseTabComponent, HomeTabComponent, MenuContainerComponent},
	events::Key,
};

pub struct AppComponent<'a> {
	home: HomeTabComponent,
	database: DatabaseTabComponent<'a>,
	menu: MenuContainerComponent,
	layout: LayoutTabComponent<'a>,
	focus: Focus,
	config: Config,
}

impl<'a> AppComponent<'a> {
	pub fn new(config: Config) -> Self {
		AppComponent {
			home: HomeTabComponent::new(config.clone()),
			database: DatabaseTabComponent::new(config.clone()),
			menu: MenuContainerComponent::new(config.clone()),
			layout: LayoutTabComponent::new(config.clone()),
			focus: Focus::MenuContainer,
			config,
		}
	}

	pub fn render<B: Backend>(&self, f: &mut tui::Frame<B>) -> Result<()> {
		let window = f.size();

		if self.config.databases.is_empty() {
			let chunks = Layout::default()
				.direction(Direction::Vertical)
				.constraints([Constraint::Length(7), Constraint::Length(93)].as_ref())
				.margin(2)
				.split(window);

			let welcome = render_container("No databases found in config file!", true);
			f.render_widget(welcome, window);

			// Banner text with correct styling
			let mut top_text = Text::from(NO_DATABASES_BANNER);
			top_text.patch_style(Style::default().fg(Color::White));
			// Contains the banner
			let top_text = Paragraph::new(top_text)
				.style(Style::default().fg(Color::White))
				.alignment(Alignment::Center)
				.block(Block::default());
			f.render_widget(top_text, chunks[0]);

			let title_text = Paragraph::new(vec![
				Spans::from(vec![Span::raw("------------------------------------")]),
				Spans::from(vec![Span::raw(format!("Config path: {}", self.config.path))]),
			])
			.style(Style::default().fg(Color::White))
			.block(Block::default())
			.alignment(Alignment::Center);
			f.render_widget(title_text, chunks[1]);
		} else {
			let main_chunks = Layout::default()
				.direction(Direction::Vertical)
				.constraints(
					[Constraint::Length(3), Constraint::Percentage(80), Constraint::Length(3)]
						.as_ref(),
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
				MenuItem::Layout => {
					self.layout.render(f, mid, matches!(self.focus(), Focus::LayoutTabBody))?
				}
			};
		}
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

	async fn components_event(&mut self, key: Key) -> Result<EventState> {
		match self.focus {
			Focus::MenuContainer => {
				if self.menu.event(key).await?.is_consumed() {
					return Ok(EventState::Consumed);
				}
			}
			Focus::HomeTabBody => {
				if self.home.event(key).await?.is_consumed() {
					return Ok(EventState::Consumed);
				}
			}
			Focus::DatabaseTabBody => {
				if self.database.event(key).await?.is_consumed() {
					return Ok(EventState::Consumed);
				}
			}
			Focus::LayoutTabBody => {
				if self.layout.event(key).await?.is_consumed() {
					return Ok(EventState::Consumed);
				}
			}
			_ => {}
		}
		Ok(EventState::NotConsumed)
	}

	async fn move_focus(&mut self, key: Key) -> Result<EventState> {
		match self.focus {
			Focus::MenuContainer => {
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
				if key == Key::Up {
					self.focus = Focus::MenuContainer
				}
			}
			Focus::LayoutTabBody => {
				if key == Key::Up {
					self.focus = Focus::MenuContainer
				}
			}
		}
		Ok(EventState::NotConsumed)
	}
}
