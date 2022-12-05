use anyhow::Result;
use app::AppComponent;
use crossterm::{
	terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
	ExecutableCommand,
};
use events::{Event, Events, Key};
use std::io;
use tui::{backend::CrosstermBackend, Terminal};

mod app;
mod components;
mod constants;
mod events;

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

#[tokio::main]
async fn main() -> Result<()> {
	setup_terminal()?;

	let stdout = io::stdout();
	let backend = CrosstermBackend::new(stdout);
	let mut terminal = Terminal::new(backend)?;
	let events = Events::new(250);
	let mut app = AppComponent::new();
	terminal.clear()?;

	loop {
		terminal.draw(|f| {
			if app.render(f).is_err() {
				std::process::exit(1);
			}
		})?;

		match events.next()? {
			Event::Input(key) => match app.event(key).await {
				Ok(state) => {
					if !state.is_consumed() && (key == Key::Char('q')) {
						break;
					}
				}
				Err(_) => unimplemented!(),
			},

			Event::Tick => {}
		}
	}

	shutdown_terminal();
	terminal.show_cursor()?;

	Ok(())
}

fn setup_terminal() -> Result<()> {
	enable_raw_mode()?;
	io::stdout().execute(EnterAlternateScreen)?;
	Ok(())
}

fn shutdown_terminal() {
	let leave_screen = io::stdout().execute(LeaveAlternateScreen).map(|_f| ());

	if let Err(e) = leave_screen {
		eprintln!("leave_screen failed:\n{}", e);
	}

	let leave_raw_mode = disable_raw_mode();

	if let Err(e) = leave_raw_mode {
		eprintln!("leave_raw_mode failed:\n{}", e);
	}
}
