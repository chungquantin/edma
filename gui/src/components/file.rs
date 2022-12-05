use anyhow::Result;
use tui::{
	backend::Backend,
	layout::{Constraint, Direction, Layout, Rect},
	Frame,
};

use crate::events::Key;

use super::{
	container::render_container, explorer::FileExplorerComponent, EventState, RenderAbleComponent,
};

enum Focus {
	Explorer,
	Database,
}

pub struct FileTabComponent {
	focus: Focus,
}

impl FileTabComponent {
	pub fn new() -> Self {
		FileTabComponent {
			focus: Focus::Explorer,
		}
	}

	pub async fn event(&mut self, key: Key) -> Result<EventState> {
		match self.focus {
			Focus::Explorer => {
				if key == Key::Right {
					self.focus = Focus::Database
				}
			}
			Focus::Database => {
				if key == Key::Left {
					self.focus = Focus::Explorer
				}
			}
		}
		Ok(EventState::NotConsumed)
	}
}
impl RenderAbleComponent for FileTabComponent {
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

		FileExplorerComponent::new().render(
			f,
			stack_chunks[0],
			focused && matches!(self.focus, Focus::Explorer),
		)?;

		f.render_widget(
			render_container("Database", focused && matches!(self.focus, Focus::Database)),
			stack_chunks[1],
		);

		Ok(())
	}
}
