use anyhow::Result;
use app::AppComponent;
use config::Config;
use crossterm::{
	execute,
	terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
};
use events::{Event, Events, Key};
use std::io;
use tui::{backend::CrosstermBackend, Terminal};

mod app;
mod components;
mod config;
mod constants;
mod events;
mod utils;

#[tokio::main]
async fn main() -> Result<()> {
	setup_terminal()?;

	let stdout = io::stdout();
	let backend = CrosstermBackend::new(stdout);
	let mut terminal = Terminal::new(backend)?;
	let events = Events::new(200);
	let config = Config::new();

	let mut app = AppComponent::new(config);
	terminal.clear()?;

	loop {
		terminal.draw(|f| {
			if let Err(err) = app.render(f) {
				println!("Error thrown: {:?}", err);
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

	shutdown_terminal()?;
	terminal.show_cursor()?;

	Ok(())
}

fn setup_terminal() -> Result<()> {
	enable_raw_mode()?;
	let mut stdout = io::stdout();
	execute!(stdout, EnterAlternateScreen)?;
	Ok(())
}

fn shutdown_terminal() -> Result<()> {
	disable_raw_mode()?;

	let mut stdout = io::stdout();
	execute!(stdout, LeaveAlternateScreen)?;
	Ok(())
}
