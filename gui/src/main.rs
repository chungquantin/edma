use chrono::prelude::*;
use crossterm::{
	event::{self, Event as CEvent, KeyCode},
	terminal::{disable_raw_mode, enable_raw_mode},
};
use serde::{Deserialize, Serialize};
use std::io;
use std::sync::mpsc;
use std::thread;
use std::time::{Duration, Instant};
use thiserror::Error;
use tui::{
	backend::{Backend, CrosstermBackend},
	layout::{Alignment, Constraint, Direction, Layout, Rect},
	style::{Color, Modifier, Style},
	text::{Span, Spans},
	widgets::{Block, BorderType, Borders, Paragraph, Tabs},
	Frame, Terminal,
};

#[derive(Error, Debug)]
pub enum Error {
	#[error("error reading the DB file: {0}")]
	ReadDBError(#[from] io::Error),
	#[error("error parsing the DB file: {0}")]
	ParseDBError(#[from] serde_json::Error),
}

enum Event<I> {
	Input(I),
	Tick,
}

const BORDER_TYPE: BorderType = BorderType::Rounded;
const PRIMARY_COLOR: Color = Color::DarkGray;

#[derive(Serialize, Deserialize, Clone)]
struct Pet {
	id: usize,
	name: String,
	category: String,
	age: usize,
	created_at: DateTime<Utc>,
}

#[derive(Copy, Clone, Debug)]
enum MenuItem {
	Home,
	File,
}

impl From<MenuItem> for usize {
	fn from(input: MenuItem) -> usize {
		match input {
			MenuItem::Home => 0,
			MenuItem::File => 1,
		}
	}
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
	enable_raw_mode().expect("can run in raw mode");

	// Handling multi-threaded IO event
	let (tx, rx) = mpsc::channel();
	let tick_rate = Duration::from_millis(200);
	thread::spawn(move || {
		let mut last_tick = Instant::now();
		loop {
			let timeout = tick_rate
				.checked_sub(last_tick.elapsed())
				.unwrap_or_else(|| Duration::from_secs(0));

			if event::poll(timeout).expect("poll works") {
				if let CEvent::Key(key) = event::read().expect("can read events") {
					tx.send(Event::Input(key)).expect("can send events");
				}
			}

			if last_tick.elapsed() >= tick_rate {
				if let Ok(_) = tx.send(Event::Tick) {
					last_tick = Instant::now();
				}
			}
		}
	});

	let stdout = io::stdout();
	let backend = CrosstermBackend::new(stdout);
	let mut terminal = Terminal::new(backend)?;
	terminal.clear()?;

	let menu_titles = vec!["EDMA", "Home", "File", "Help", "Github", "Quit"];
	let mut active_menu_item = MenuItem::Home;

	loop {
		terminal.draw(|rect| {
			let window = rect.size();
			let v_layout = Layout::default()
				.direction(Direction::Vertical)
				.constraints(
					[Constraint::Length(3), Constraint::Min(2), Constraint::Length(3)].as_ref(),
				)
				.split(window);

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
								Style::default()
									.fg(Color::Yellow)
									.add_modifier(Modifier::UNDERLINED),
							),
							Span::styled(rest, Style::default().fg(Color::White)),
						])
					}
				})
				.collect();

			let tabs = Tabs::new(menu)
				.select(active_menu_item.into())
				.block(render_area("Menu", PRIMARY_COLOR))
				.divider(Span::raw("|"));

			rect.render_widget(tabs, v_layout[0]);

			match active_menu_item {
				MenuItem::Home => HomeTabComponent::new().render(rect, v_layout[1]),
				MenuItem::File => FileTabComponent::new().render(rect, v_layout[1]),
			}
		})?;

		match rx.recv()? {
			Event::Input(event) => match event.code {
				KeyCode::Char('q') => {
					disable_raw_mode()?;
					terminal.show_cursor()?;
					break;
				}
				KeyCode::Char('h') => active_menu_item = MenuItem::Home,
				KeyCode::Char('f') => active_menu_item = MenuItem::File,
				_ => {}
			},
			Event::Tick => {}
		}
	}

	Ok(())
}

fn render_area<'a>(title: &'a str, color: Color) -> Block<'a> {
	Block::<'a>::default()
		.borders(Borders::ALL)
		.style(Style::default().fg(color))
		.title(title)
		.border_type(BORDER_TYPE)
}

struct FileTabComponent {}

impl FileTabComponent {
	pub fn new() -> Self {
		FileTabComponent {}
	}

	pub fn render<B: Backend>(self, f: &mut Frame<B>, area: Rect) {
		let h_layout = Layout::default()
			.direction(Direction::Horizontal)
			.constraints([Constraint::Percentage(20), Constraint::Percentage(80)])
			.split(area);

		f.render_widget(render_area("Explorer", PRIMARY_COLOR), h_layout[0]);
		f.render_widget(render_area("", PRIMARY_COLOR), h_layout[1]);
	}
}

struct HomeTabComponent {}

impl HomeTabComponent {
	pub fn new() -> Self {
		HomeTabComponent {}
	}

	pub fn render<B: Backend>(self, f: &mut Frame<B>, area: Rect) {
		let home = Paragraph::new(vec![
			Spans::from(vec![Span::raw("")]),
			Spans::from(vec![Span::raw("EDMA")]),
			Spans::from(vec![Span::raw("Embedded Database Management for All")]),
		])
		.alignment(Alignment::Center)
		.block(render_area("Home", PRIMARY_COLOR));

		f.render_widget(home, area)
	}
}
