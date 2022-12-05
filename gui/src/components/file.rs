use anyhow::Result;
use tui::{
	backend::Backend,
	layout::{Constraint, Direction, Layout, Rect},
	Frame,
};

use crate::{config::Config, events::Key};

use super::{explorer::FileExplorerComponent, EditorComponent, EventState, RenderAbleComponent};

enum Focus {
	Explorer,
	Editor,
}

pub struct FileTabComponent<'a> {
	focus: Focus,
	config: Config,
	explorer: FileExplorerComponent<'a>,
	editor: EditorComponent,
}

impl<'a> FileTabComponent<'a> {
	pub fn new(config: Config) -> Self {
		FileTabComponent {
			explorer: FileExplorerComponent::new(config.clone()),
			editor: EditorComponent::new(config.clone()),
			focus: Focus::Explorer,
			config,
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
		let stack_chunks = Layout::default()
			.direction(Direction::Horizontal)
			.constraints([Constraint::Percentage(20), Constraint::Percentage(80)])
			.split(rect);

		self.explorer.render(
			f,
			stack_chunks[0],
			focused && matches!(self.focus, Focus::Explorer),
		)?;
		self.editor.render(f, stack_chunks[1], focused && matches!(self.focus, Focus::Editor))?;

		Ok(())
	}
}
