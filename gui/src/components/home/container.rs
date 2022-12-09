use anyhow::{Ok, Result};
use tui::{
	backend::Backend,
	layout::{Alignment, Constraint, Direction, Layout, Rect},
	style::{Color, Style},
	text::{Span, Spans, Text},
	widgets::{Block, Paragraph, Wrap},
	Frame,
};

use crate::{
	components::{render_container, RenderAbleComponent},
	config::Config,
	constants::{BANNER, HIGHLIGHT_COLOR},
	events::{EventState, Key},
};

enum Focus {
	Container,
	Inner,
}

const SMALL_SCROLL: u16 = 1;

pub struct HomeTabComponent {
	config: Config,
	scroll_position: u16,
	focus: Focus,
}

impl HomeTabComponent {
	pub fn new(config: Config) -> Self {
		HomeTabComponent {
			config,
			scroll_position: 0,
			focus: Focus::Container,
		}
	}

	pub async fn event(&mut self, key: Key) -> Result<EventState> {
		match self.focus {
			Focus::Container => {
				if key == self.config.key_config.enter {
					self.focus = Focus::Inner;
					return Ok(EventState::Consumed);
				}
			}
			Focus::Inner => match key {
				k if k == self.config.key_config.up => {
					self.scroll_position = self.scroll_position.saturating_sub(SMALL_SCROLL);
					return Ok(EventState::Consumed);
				}
				k if k == self.config.key_config.down => {
					self.scroll_position = self.scroll_position.saturating_add(SMALL_SCROLL);
					return Ok(EventState::Consumed);
				}
				k if k == self.config.key_config.escape => {
					self.focus = Focus::Container;
					return Ok(EventState::Consumed);
				}
				_ => {}
			},
		}
		Ok(EventState::NotConsumed)
	}
}

impl RenderAbleComponent for HomeTabComponent {
	fn render<B: Backend>(
		&self,
		f: &mut Frame<B>,
		rect: Rect,
		focused: bool,
	) -> Result<(), anyhow::Error> {
		let chunks = Layout::default()
			.direction(Direction::Horizontal)
			.constraints([Constraint::Percentage(40), Constraint::Percentage(60)].as_ref())
			.margin(2)
			.split(rect);

		let vstack = Layout::default()
			.direction(Direction::Vertical)
			.constraints([Constraint::Length(7), Constraint::Length(3)].as_ref())
			.margin(2)
			.split(chunks[0]);

		let keycode = match self.focus {
			Focus::Container => "ENTER",
			Focus::Inner => "ESC",
		};
		let label = format!("Welcome! [{}]", keycode);
		let welcome = render_container(&label, focused);
		f.render_widget(welcome, rect);

		let changelog = include_str!("../../../../CHANGELOG.md").to_string();

		let bottom_text_raw = format!(
			"{}{}",
			"\nPlease report any bugs or missing features to https://github.com/nomadiz/Edma\n\n",
			changelog
		);
		let bottom_text = Text::from(bottom_text_raw.as_str());

		// Banner text with correct styling
		let mut top_text = Text::from(BANNER);
		top_text.patch_style(Style::default().fg(HIGHLIGHT_COLOR));
		// Contains the banner
		let top_text = Paragraph::new(top_text)
			.style(Style::default().fg(Color::White))
			.alignment(Alignment::Center)
			.block(Block::default());
		f.render_widget(top_text, vstack[0]);

		let description = vec![
			Spans::from(vec![Span::raw(">---------------------------------------<")]),
			Spans::from(vec![Span::raw("Embedded Databases Management for All")]),
			Spans::from(vec![Span::raw("")]),
			Spans::from(vec![Span::raw("Built by @nomadiz")]),
		];
		let subtitle_text = Paragraph::new(description)
			.style(Style::default().fg(Color::White))
			.alignment(Alignment::Center)
			.block(Block::default())
			.wrap(Wrap {
				trim: true,
			});
		f.render_widget(subtitle_text, vstack[1]);

		let bottom_text = Paragraph::new(bottom_text)
			.style(Style::default().fg(Color::White))
			.block(Block::default())
			.wrap(Wrap {
				trim: false,
			})
			.scroll((self.scroll_position, 0));
		f.render_widget(bottom_text, chunks[1]);

		Ok(())
	}
}
