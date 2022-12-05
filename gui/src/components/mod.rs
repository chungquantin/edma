use anyhow::Result;
use tui::{backend::Backend, layout::Rect, Frame};

mod container;
mod editor;
mod explorer;
mod file;
mod home;
mod menu;

pub use container::*;
pub use editor::*;
pub use explorer::*;
pub use file::*;
pub use home::*;
pub use menu::*;

pub trait RenderAbleComponent {
	fn render<B: Backend>(&self, f: &mut Frame<B>, rect: Rect, focused: bool) -> Result<()>;
}

#[derive(Debug, PartialEq, Eq)]
pub enum EventState {
	Consumed,
	NotConsumed,
}

impl EventState {
	pub fn is_consumed(&self) -> bool {
		*self == Self::Consumed
	}
}
