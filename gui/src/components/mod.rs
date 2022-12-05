use anyhow::Result;
use tui::{
	backend::Backend,
	layout::{Alignment, Constraint, Direction, Layout, Rect},
	style::{Color, Modifier, Style},
	text::{Span, Spans},
	widgets::{Block, Borders, Paragraph, Tabs},
	Frame,
};

use crate::{
	constants::{BORDER_TYPE, PRIMARY_COLOR},
	events::Key,
	MenuItem,
};

fn render_area<'a>(title: &'a str, color: Color) -> Block<'a> {
	Block::<'a>::default()
		.borders(Borders::ALL)
		.style(Style::default().fg(color))
		.title(title)
		.border_type(BORDER_TYPE)
}

pub trait RenderAbleComponent {
	fn render<B: Backend>(&self, f: &mut Frame<B>, rect: Rect, focused: bool) -> Result<()>;
}

#[derive(Debug)]
pub struct MenuContainerComponent {
	pub active_menu_item: MenuItem,
}

#[derive(PartialEq, Eq, Debug)]
pub enum EventState {
	Consumed,
	NotConsumed,
}

impl EventState {
	pub fn is_consumed(&self) -> bool {
		*self == Self::Consumed
	}
}

impl MenuContainerComponent {
	pub fn new(active_menu_item: MenuItem) -> Self {
		MenuContainerComponent {
			active_menu_item,
		}
	}

	pub fn set_active(&mut self, active_menu_item: MenuItem) {
		self.active_menu_item = active_menu_item;
	}

	pub async fn event(&mut self, key: Key) -> Result<EventState> {
		if key == Key::Char('h') {
			self.set_active(MenuItem::Home);
			return Ok(EventState::Consumed);
		}
		if key == Key::Char('f') {
			self.set_active(MenuItem::File);
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
		_focused: bool,
	) -> Result<(), anyhow::Error> {
		let menu_titles = vec!["EDMA", "Home", "File", "Help", "Github", "Quit"];

		let menu = menu_titles
			.iter()
			.enumerate()
			.map(|(index, t)| {
				if index == 0 {
					Spans::from(vec![Span::styled(*t, Style::default().fg(Color::Yellow))])
				} else {
					let (first, rest) = t.split_at(1);
					Spans::from(vec![
						Span::styled(
							first,
							Style::default().fg(Color::Yellow).add_modifier(Modifier::UNDERLINED),
						),
						Span::styled(rest, Style::default().fg(Color::White)),
					])
				}
			})
			.collect();

		let tabs = Tabs::new(menu)
			.select(self.active_menu_item.into())
			.block(render_area("Menu", PRIMARY_COLOR))
			.divider(Span::raw("|"));

		f.render_widget(tabs, area);
		Ok(())
	}
}

#[derive(Debug)]
pub struct FileTabComponent {}

impl FileTabComponent {
	pub fn new() -> Self {
		FileTabComponent {}
	}
}
impl RenderAbleComponent for FileTabComponent {
	fn render<B: Backend>(
		&self,
		f: &mut Frame<B>,
		rect: Rect,
		_focused: bool,
	) -> Result<(), anyhow::Error> {
		let h_layout = Layout::default()
			.direction(Direction::Horizontal)
			.constraints([Constraint::Percentage(20), Constraint::Percentage(80)])
			.split(rect);

		f.render_widget(render_area("Explorer", PRIMARY_COLOR), h_layout[0]);
		f.render_widget(render_area("Database", PRIMARY_COLOR), h_layout[1]);

		Ok(())
	}
}

#[derive(Debug)]
pub struct HomeTabComponent {}

impl HomeTabComponent {
	pub fn new() -> Self {
		HomeTabComponent {}
	}
}
impl RenderAbleComponent for HomeTabComponent {
	fn render<B: Backend>(
		&self,
		f: &mut Frame<B>,
		rect: Rect,
		_focused: bool,
	) -> Result<(), anyhow::Error> {
		let home = Paragraph::new(vec![
			Spans::from(vec![Span::raw("")]),
			Spans::from(vec![Span::raw("EDMA")]),
			Spans::from(vec![Span::raw("Embedded Database Management for All")]),
		])
		.alignment(Alignment::Center)
		.block(render_area("Home", PRIMARY_COLOR));

		f.render_widget(home, rect);
		Ok(())
	}
}
