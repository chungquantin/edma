use anyhow::Result;
use tui::{backend::Backend, layout::Rect, Frame};

mod container;
mod explorer;
mod file;
mod home;
mod menu;
mod tab;

pub use container::*;
pub use explorer::*;
pub use file::*;
pub use home::*;
pub use menu::*;
pub use tab::*;

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
