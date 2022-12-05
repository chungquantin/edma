use crate::components::RenderAbleComponent;
use chrono::prelude::*;
use components::{FileTabComponent, HomeTabComponent, MenuContainerComponent};
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
	backend::CrosstermBackend,
	layout::{Constraint, Direction, Layout},
	Terminal,
};

mod components;
mod constants;

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

#[derive(Serialize, Deserialize, Clone)]
struct Pet {
	id: usize,
	name: String,
	category: String,
	age: usize,
	created_at: DateTime<Utc>,
}

#[derive(Copy, Clone, Debug)]
pub enum MenuItem {
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

			if last_tick.elapsed() >= tick_rate && tx.send(Event::Tick).is_ok() {
				last_tick = Instant::now();
			}
		}
	});

	let stdout = io::stdout();
	let backend = CrosstermBackend::new(stdout);
	let mut terminal = Terminal::new(backend)?;
	terminal.clear()?;

	let mut menu = MenuContainerComponent::new(MenuItem::Home);
	loop {
		terminal.draw(|f| {
			let window = f.size();
			let main_chunks = Layout::default()
				.direction(Direction::Vertical)
				.constraints(
					[Constraint::Length(3), Constraint::Min(2), Constraint::Length(3)].as_ref(),
				)
				.split(window);
			let (top, mid) = (main_chunks[0], main_chunks[1]);

			menu.render(f, top, false).unwrap();

			let home_tab = HomeTabComponent::new();
			let file_tab = FileTabComponent::new();
			match menu.active_menu_item {
				MenuItem::Home => home_tab.render(f, mid, false).unwrap(),
				MenuItem::File => file_tab.render(f, mid, false).unwrap(),
			};
		})?;

		match rx.recv()? {
			Event::Input(event) => match event.code {
				KeyCode::Char('q') => {
					disable_raw_mode()?;
					terminal.show_cursor()?;
					break;
				}
				KeyCode::Char('h') => {
					menu.set_active(MenuItem::Home);
				}
				KeyCode::Char('f') => {
					menu.set_active(MenuItem::File);
				}
				_ => {}
			},
			Event::Tick => {}
		}
	}

	Ok(())
}
