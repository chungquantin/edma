use tui::{
	backend::Backend,
	layout::Rect,
	text::{Span, Spans},
	widgets::Paragraph,
	Frame,
};

use crate::config::Config;

use super::{container::render_container, RenderAbleComponent};

pub struct StatusComponent<'a> {
	config: Config,
	text: Span<'a>,
}

const DEFAULT_TEXT: &str = "No status displayed...";

impl<'a> StatusComponent<'a> {
	pub fn new(config: Config) -> Self {
		StatusComponent {
			config,
			text: Span::raw(DEFAULT_TEXT),
		}
	}

	pub fn set_text(&mut self, text: Span<'a>) {
		self.text = text;
	}

	pub fn reset(&mut self) {
		self.text = Span::raw(DEFAULT_TEXT);
	}
}

impl<'a> RenderAbleComponent for StatusComponent<'a> {
	fn render<B: Backend>(
		&self,
		f: &mut Frame<B>,
		rect: Rect,
		focused: bool,
	) -> Result<(), anyhow::Error> {
		let status = Paragraph::new(vec![Spans::from(vec![self.text.clone()])])
			.block(render_container("Status", focused));
		f.render_widget(status, rect);
		Ok(())
	}
}
