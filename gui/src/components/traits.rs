use anyhow::Result;
use tui::{backend::Backend, layout::Rect, Frame};

pub trait RenderAbleComponent {
	fn render<B: Backend>(&self, f: &mut Frame<B>, rect: Rect, focused: bool) -> Result<()>;
}
