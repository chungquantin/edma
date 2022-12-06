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
	explorer::FileExplorerComponent, EditorComponent, RenderAbleComponent, StatusComponent,
};

enum Focus {
	Explorer,
	Editor,
}

pub struct FileTabComponent<'a> {
	focus: Focus,
	config: Config,
	explorer: FileExplorerComponent<'a>,
	editor: EditorComponent,
	status: StatusComponent<'a>,
}

impl<'a> FileTabComponent<'a> {
	pub fn new(config: Config) -> Self {
		FileTabComponent {
			explorer: FileExplorerComponent::new(config.clone()),
			editor: EditorComponent::new(config.clone()),
			status: StatusComponent::new(config.clone()),
			focus: Focus::Explorer,
			config,
		}
	}

	pub async fn event(&mut self, key: Key) -> Result<EventState> {
		match self.focus {
			Focus::Explorer => {
				if self.explorer.event(key).await?.is_consumed() {
					if self.explorer.is_selected() {
						let get_selected_file = self.explorer.selected_file();
						self.status.set_text(Span::raw(get_selected_file.unwrap()));
					} else {
						self.status.reset();
					}

					if key == Key::Right {
						self.focus = Focus::Editor;
						return Ok(EventState::Consumed);
					}
					return Ok(EventState::Consumed);
				}
				Ok(EventState::NotConsumed)
			}
			Focus::Editor => {
				if key == Key::Left {
					self.focus = Focus::Explorer;
					return Ok(EventState::Consumed);
				}
				Ok(EventState::NotConsumed)
			}
		}
	}
}

impl<'a> RenderAbleComponent for FileTabComponent<'a> {
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
