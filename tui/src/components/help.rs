use anyhow::Result;
use tui::{
	backend::Backend,
	layout::{Alignment, Constraint, Direction, Layout, Rect},
	style::{Color, Modifier, Style},
	text::{Span, Spans},
	widgets::{Block, BorderType, Borders, Clear, Paragraph},
	Frame,
};

use crate::{components::RenderAbleComponent, events::EventState, events::Key};

pub struct HelpModalComponent {
	open: bool,
}

impl HelpModalComponent {
	pub fn new() -> Self {
		HelpModalComponent { open: false }
	}

	pub fn is_open(&self) -> bool {
		self.open
	}

	pub fn toggle(&mut self) {
		self.open = !self.open;
	}

	pub fn close(&mut self) {
		self.open = false;
	}

	pub async fn event(&mut self, key: Key) -> Result<EventState> {
		if !self.open {
			return Ok(EventState::NotConsumed);
		}

		match key {
			Key::Char('/') | Key::Esc => {
				self.close();
				Ok(EventState::Consumed)
			}
			_ => Ok(EventState::NotConsumed),
		}
	}
}

fn help_content() -> Vec<Spans<'static>> {
	let key_style = Style::default().fg(Color::Yellow).add_modifier(Modifier::BOLD);
	let desc_style = Style::default().fg(Color::White);
	let separator = Span::raw("  ");

	let row = |key: &str, desc: &str| {
		Spans::from(vec![
			Span::styled(format!("  {:<14}", key), key_style),
			separator.clone(),
			Span::styled(desc.to_string(), desc_style),
		])
	};

	vec![
		Spans::from(vec![Span::styled(
			"  Keybindings",
			Style::default().fg(Color::Cyan).add_modifier(Modifier::BOLD),
		)]),
		Spans::from(vec![Span::raw("")]),
		row("h", "Switch to Home tab"),
		row("d", "Switch to Database tab"),
		row("l", "Switch to Layout tab"),
		row("q", "Quit EDMA"),
		Spans::from(vec![Span::raw("")]),
		Spans::from(vec![Span::styled(
			"  Database Tab",
			Style::default().fg(Color::Cyan).add_modifier(Modifier::BOLD),
		)]),
		Spans::from(vec![Span::raw("")]),
		row("Enter", "Focus explorer / editor / command"),
		row("Esc", "Return to container focus"),
		row("Up/Down", "Navigate list or table"),
		row("Left/Right", "Move between panels"),
		row("9/0", "Select database up/down"),
		Spans::from(vec![Span::raw("")]),
		Spans::from(vec![Span::styled(
			"  Command Editor",
			Style::default().fg(Color::Cyan).add_modifier(Modifier::BOLD),
		)]),
		Spans::from(vec![Span::raw("")]),
		row("COLUMN=<cf>", "Set column family (RocksDB)"),
		row("TREE=<tree>", "Set tree (sled)"),
		row("PREFIX=<val>", "Prefix scan"),
		row("SUFFIX=<val>", "Suffix scan"),
		Spans::from(vec![Span::raw("")]),
		Spans::from(vec![Span::styled(
			"  Layout Tab",
			Style::default().fg(Color::Cyan).add_modifier(Modifier::BOLD),
		)]),
		Spans::from(vec![Span::raw("")]),
		row("h/j", "Key layout up/down"),
		row("k/l", "Value layout up/down"),
		Spans::from(vec![Span::raw("")]),
		Spans::from(vec![Span::raw("")]),
		Spans::from(vec![Span::styled(
			"  Press / or Esc to close",
			Style::default().fg(Color::DarkGray),
		)]),
	]
}

impl RenderAbleComponent for HelpModalComponent {
	fn render<B: Backend>(
		&self,
		f: &mut Frame<B>,
		rect: Rect,
		_focused: bool,
	) -> Result<(), anyhow::Error> {
		if !self.open {
			return Ok(());
		}

		let content = help_content();
		let content_height = content.len() as u16 + 2; // +2 for borders
		let modal_height = content_height.min(rect.height.saturating_sub(4));
		let modal_width = (rect.width * 3 / 4).min(70);

		let modal_area = Layout::default()
			.direction(Direction::Horizontal)
			.constraints([
				Constraint::Length((rect.width - modal_width) / 2),
				Constraint::Length(modal_width),
				Constraint::Min(0),
			])
			.split(rect)[1];

		let modal_area = Layout::default()
			.direction(Direction::Vertical)
			.constraints([
				Constraint::Length((rect.height - modal_height) / 2),
				Constraint::Length(modal_height),
				Constraint::Min(0),
			])
			.split(modal_area)[1];

		f.render_widget(Clear, modal_area);

		let block = Block::default()
			.borders(Borders::ALL)
			.border_type(BorderType::Rounded)
			.border_style(Style::default().fg(Color::Cyan))
			.title(Spans::from(vec![Span::styled(
				" Help ",
				Style::default().fg(Color::Cyan).add_modifier(Modifier::BOLD),
			)]))
			.title_alignment(Alignment::Center);

		let paragraph = Paragraph::new(content).block(block);
		f.render_widget(paragraph, modal_area);

		Ok(())
	}
}
