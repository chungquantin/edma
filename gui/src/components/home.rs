use anyhow::Result;
use tui::{
	backend::Backend,
	layout::{Alignment, Rect},
	text::{Span, Spans},
	widgets::Paragraph,
	Frame,
};

use crate::config::Config;

use super::{container::render_container, RenderAbleComponent};

pub struct HomeTabComponent {
	config: Config,
}

impl HomeTabComponent {
	pub fn new(config: Config) -> Self {
		HomeTabComponent {
			config,
		}
	}
}
impl RenderAbleComponent for HomeTabComponent {
	fn render<B: Backend>(
		&self,
		f: &mut Frame<B>,
		rect: Rect,
		focused: bool,
	) -> Result<(), anyhow::Error> {
		let home = Paragraph::new(vec![
			Spans::from(vec![Span::raw("")]),
			Spans::from(vec![Span::raw("EDMA")]),
			Spans::from(vec![Span::raw("Embedded Database Management for All")]),
		])
		.alignment(Alignment::Center)
		.block(render_container("Home", focused));

		f.render_widget(home, rect);
		Ok(())
	}
}
