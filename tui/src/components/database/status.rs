use crate::{
	components::{render_container, RenderAbleComponent},
	config::Config,
	constants::DEFAULT_STATUS_TEXT,
};
use tui::{
	backend::Backend,
	layout::Rect,
	text::{Span, Spans},
	widgets::Paragraph,
	Frame,
};

pub struct StatusComponent<'a> {
	config: Config,
	text: Span<'a>,
}

impl<'a> StatusComponent<'a> {
	pub fn new(config: Config) -> Self {
		StatusComponent {
			config,
			text: Span::raw(DEFAULT_STATUS_TEXT),
		}
	}

	pub fn set_text(&mut self, text: Span<'a>) {
		self.text = text;
	}

	pub fn reset(&mut self) {
		self.text = Span::raw(DEFAULT_STATUS_TEXT);
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
