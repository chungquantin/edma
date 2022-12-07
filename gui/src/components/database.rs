use anyhow::Result;
use tui::{
	backend::Backend,
	layout::{Constraint, Direction, Layout, Rect},
	text::Span,
	Frame,
};

use crate::{
	config::Config,
	events::{EventState, Key},
};

use super::{
	explorer::DatabaseExplorerComponent, DatabaseEditorComponent, RenderAbleComponent,
	StatusComponent,
};

enum Focus {
	Explorer,
	Editor,
}

pub struct DatabaseTabComponent<'a> {
	focus: Focus,
	config: Config,
	explorer: DatabaseExplorerComponent<'a>,
	editor: DatabaseEditorComponent<'a>,
	status: StatusComponent<'a>,
}

impl<'a> DatabaseTabComponent<'a> {
	pub fn new(config: Config) -> Self {
		DatabaseTabComponent {
			explorer: DatabaseExplorerComponent::new(config.clone()),
			editor: DatabaseEditorComponent::new(config.clone()),
			status: StatusComponent::new(config.clone()),
			focus: Focus::Explorer,
			config,
		}
	}

	async fn handle_explorer_event(&mut self) {
		if self.explorer.is_selected() {
			let database = self.explorer.selected_file();
			let (name, path) = database.unwrap();
			self.status.set_text(Span::raw(path.clone()));
			self.editor.scan_database(&name, &path).await;
		} else {
			self.status.reset();
		}
	}

	pub async fn event(&mut self, key: Key) -> Result<EventState> {
		match self.focus {
			Focus::Explorer => {
				if key == Key::Right {
					self.focus = Focus::Editor;
					return Ok(EventState::Consumed);
				}

				if self.explorer.event(key).await?.is_consumed() {
					self.handle_explorer_event().await;
					return Ok(EventState::Consumed);
				}
				Ok(EventState::NotConsumed)
			}
			Focus::Editor => {
				if key == Key::Left {
					self.focus = Focus::Explorer;
					return Ok(EventState::Consumed);
				}

				if self.editor.event(key).await?.is_consumed() {
					return Ok(EventState::Consumed);
				}
				Ok(EventState::NotConsumed)
			}
		}
	}
}

impl<'a> RenderAbleComponent for DatabaseTabComponent<'a> {
	fn render<B: Backend>(
		&self,
		f: &mut Frame<B>,
		rect: Rect,
		focused: bool,
	) -> Result<(), anyhow::Error> {
		let main_chunks = Layout::default()
			.direction(Direction::Horizontal)
			.constraints([Constraint::Percentage(20), Constraint::Percentage(80)])
			.split(rect);

		let stack_chunks = Layout::default()
			.direction(Direction::Vertical)
			.constraints([Constraint::Length(main_chunks[0].height - 3), Constraint::Length(2)])
			.split(main_chunks[1]);

		self.explorer.render(
			f,
			main_chunks[0],
			focused && matches!(self.focus, Focus::Explorer),
		)?;
		self.editor.render(f, stack_chunks[0], focused && matches!(self.focus, Focus::Editor))?;
		self.status.render(f, stack_chunks[1], false)?;

		Ok(())
	}
}
