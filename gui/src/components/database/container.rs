use crate::{
	components::RenderAbleComponent,
	config::Config,
	events::{EventState, Key},
	utils::get_absolute_path,
};
use anyhow::Result;
use tui::{
	backend::Backend,
	layout::{Constraint, Direction, Layout, Rect},
	text::Span,
	Frame,
};

use super::{
	database_explorer::DatabaseExplorerComponent, DatabaseEditorComponent,
	DatabaseSelectionComponent, StatusComponent, TextareaComponent,
};

enum Focus {
	Explorer,
	Editor,
	Textarea,
}

pub struct DatabaseTabComponent<'a> {
	focus: Focus,
	config: Config,
	textarea: TextareaComponent,
	databases: DatabaseSelectionComponent<'a>,
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
			databases: DatabaseSelectionComponent::new(config.clone()),
			textarea: TextareaComponent::new(config.clone()),
			focus: Focus::Explorer,
			config,
		}
	}

	fn get_selected_database(&self) -> String {
		let database_index = self.databases.state().selected().unwrap();
		let databases: Vec<_> = self.config.databases.keys().collect();
		let selected_database = databases[database_index];
		selected_database.clone()
	}

	async fn handle_explorer_event(&mut self) {
		if self.explorer.state().selected().is_some() {
			let selected_file = self.explorer.state().selected().unwrap_or(0);
			let selected_db = self.get_selected_database();
			let databases = self.config.databases.get(&selected_db).unwrap();
			let database = &databases[selected_file];
			let (name, path) = (selected_db, database.path.clone());
			let abs_p = get_absolute_path(&path.to_string());
			self.status.set_text(Span::raw(abs_p));
			self.editor.scan_database(&name, &path).await;
		} else {
			self.status.reset();
		}
	}

	pub async fn event(&mut self, key: Key) -> Result<EventState> {
		match self.focus {
			Focus::Explorer => {
				if key == Key::Right {
					self.focus = Focus::Textarea;
					return Ok(EventState::Consumed);
				}
				if self.databases.event(key).await?.is_consumed() {
					let databases: Vec<_> = self.config.databases.keys().collect();
					let selected = self.databases.state().selected().unwrap();
					let db = databases[selected].to_string();
					self.explorer.set_database(db);
					return Ok(EventState::Consumed);
				}

				if self.explorer.event(key).await?.is_consumed() {
					self.handle_explorer_event().await;
					return Ok(EventState::Consumed);
				}
				Ok(EventState::NotConsumed)
			}
			Focus::Textarea => {
				if self.textarea.event(key).await?.is_consumed() {
					return Ok(EventState::Consumed);
				}
				if key == Key::Left {
					self.focus = Focus::Explorer;
					return Ok(EventState::Consumed);
				}

				if key == Key::Down {
					self.focus = Focus::Editor;
					return Ok(EventState::Consumed);
				}
				Ok(EventState::NotConsumed)
			}
			Focus::Editor => {
				if self.editor.event(key).await?.is_consumed() {
					return Ok(EventState::Consumed);
				}
				if key == Key::Left {
					self.focus = Focus::Explorer;
					return Ok(EventState::Consumed);
				}

				if key == Key::Up {
					self.focus = Focus::Textarea;
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

		let left_stack_chunks = Layout::default()
			.direction(Direction::Vertical)
			.constraints([Constraint::Percentage(30), Constraint::Percentage(70)])
			.split(main_chunks[0]);

		let right_stack_chunks = Layout::default()
			.direction(Direction::Vertical)
			.constraints([
				Constraint::Length(3),
				Constraint::Length(main_chunks[0].height - 6),
				Constraint::Length(2),
			])
			.split(main_chunks[1]);

		self.databases.render(
			f,
			left_stack_chunks[0],
			focused && matches!(self.focus, Focus::Explorer),
		)?;
		self.explorer.render(
			f,
			left_stack_chunks[1],
			focused && matches!(self.focus, Focus::Explorer),
		)?;
		self.textarea.render(
			f,
			right_stack_chunks[0],
			focused && matches!(self.focus, Focus::Textarea),
		)?;
		self.editor.render(
			f,
			right_stack_chunks[1],
			focused && matches!(self.focus, Focus::Editor),
		)?;
		self.status.render(f, right_stack_chunks[2], false)?;

		Ok(())
	}
}
