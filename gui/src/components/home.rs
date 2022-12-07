use anyhow::Result;
use tui::{
	backend::Backend,
	layout::{Alignment, Constraint, Direction, Layout, Rect},
	style::{Color, Style},
	text::Text,
	widgets::{Block, Paragraph, Wrap},
	Frame,
};

use crate::{config::Config, constants::BANNER};

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
		let chunks = Layout::default()
			.direction(Direction::Vertical)
			.constraints([Constraint::Length(7), Constraint::Length(93)].as_ref())
			.margin(2)
			.split(rect);

		let welcome = render_container("Welcome!", focused);
		f.render_widget(welcome, rect);

		// let changelog = include_str!("../../CHANGELOG.md").to_string();
		let changelog = "".to_string();

		// If debug mode show the "Unreleased" header. Otherwise it is a release so there should be no
		// unreleased features
		let clean_changelog = if cfg!(debug_assertions) {
			changelog
		} else {
			changelog.replace("\n## [Unreleased]\n", "")
		};

		// Banner text with correct styling
		let mut top_text = Text::from(BANNER);
		top_text.patch_style(Style::default().fg(Color::White));

		let bottom_text_raw = format!(
    "{}{}",
    "\nPlease report any bugs or missing features to https://github.com/Rigellute/spotify-tui\n\n",
    clean_changelog
  );
		let bottom_text = Text::from(bottom_text_raw.as_str());

		// Contains the banner
		let top_text = Paragraph::new(top_text)
			.style(Style::default().fg(Color::White))
			.alignment(Alignment::Center)
			.block(Block::default());
		f.render_widget(top_text, chunks[0]);

		// CHANGELOG
		let bottom_text = Paragraph::new(bottom_text)
			.style(Style::default().fg(Color::White))
			.block(Block::default())
			.alignment(Alignment::Center)
			.wrap(Wrap {
				trim: false,
			});
		// .scroll((0, 0));
		f.render_widget(bottom_text, chunks[1]);

		Ok(())
	}
}
