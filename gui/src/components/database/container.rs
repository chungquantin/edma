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
	database_explorer::DatabaseExplorerComponent, CommandComponent, DatabaseEditorComponent,
	DatabaseSelectionComponent, StatusComponent,
};

enum Focus {
	Explorer,
	Editor,
	Command,
}

pub struct DatabaseTabComponent<'a> {
	focus: Focus,
	config: Config,
	command: CommandComponent,
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
			command: CommandComponent::new(config.clone()),
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

	fn get_database_info(&self) -> (String, String, String) {
		let selected_file = self.explorer.state().selected().unwrap_or(0);
		let selected_db = self.get_selected_database();
		let databases = self.config.databases.get(&selected_db).unwrap();
		let database = &databases[selected_file];
		let (name, path) = (selected_db, database.path.clone());
		let abs_p = get_absolute_path(&path);
		(name, path, abs_p)
	}

	async fn handle_command_event(&mut self) {
		let commands = self.command.commands.to_vec();
		let mut cf_handle = None;
		let (name, path, _) = self.get_database_info();
		for command in commands {
			match command.token.as_str() {
				// COLUMN is specified for RocksDB, Redb should be TABLE
				"COLUMN" => {
					let cf = Some(&command.value);
					cf_handle = Some(cf.unwrap().as_bytes().to_vec());
					self.editor.scan_database(cf_handle.clone(), &name, &path).await;
				}
				// PREFIX and SUFFIX scan only support key traversal not value traversal
				"PREFIX" => {
					let prefix = &command.value;
					let bytes = prefix.as_bytes().to_vec();
					self.editor.prefix_scan_database(cf_handle.clone(), &name, &path, bytes).await;
				}
				"SUFFIX" => {
					let suffix = &command.value;
					let bytes = suffix.as_bytes().to_vec();
					self.editor.suffix_scan_database(cf_handle.clone(), &name, &path, bytes).await;
				}
				_ => {}
			}
		}
		self.command.reset_command();
	}

	async fn handle_explorer_event(&mut self) {
		if self.explorer.state().selected().is_some() {
			let (name, path, abs_p) = self.get_database_info();
			self.status.set_text(Span::raw(abs_p));
			self.editor.scan_database(None, &name, &path).await;
		} else {
			self.status.reset();
		}
	}

	pub async fn event(&mut self, key: Key) -> Result<EventState> {
		match self.focus {
			Focus::Explorer => {
				if key == Key::Right {
					self.focus = Focus::Command;
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
			Focus::Command => {
				if self.command.event(key).await?.is_consumed() {
					self.handle_command_event().await;
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
					self.focus = Focus::Command;
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
				Constraint::Length(main_chunks[0].height.saturating_sub(6)),
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
		self.command.render(
			f,
			right_stack_chunks[0],
			focused && matches!(self.focus, Focus::Command),
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
