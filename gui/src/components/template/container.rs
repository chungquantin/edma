use crate::{
	components::RenderAbleComponent,
	config::Config,
	events::{EventState, Key},
};
use anyhow::Result;
use tui::{
	backend::Backend,
	layout::{Constraint, Direction, Layout, Rect},
	Frame,
};

use super::{CommandEditorComponent, TemplateExplorerComponent, TemplateLayoutComponent};

enum Focus {
	Explorer,
	Layout,
	Editor,
}

pub struct LayoutTabComponent<'a> {
	config: Config,
	explorer: TemplateExplorerComponent<'a>,
	layout: TemplateLayoutComponent,
	editor: CommandEditorComponent,
	focus: Focus,
}

impl LayoutTabComponent<'_> {
	pub fn new(config: Config) -> Self {
		LayoutTabComponent {
			explorer: TemplateExplorerComponent::new(config.clone()),
			layout: TemplateLayoutComponent::new(config.clone()),
			editor: CommandEditorComponent::new(config.clone()),
			config,
			focus: Focus::Explorer,
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
					if let Some(selected) = self.explorer.list.state.selected() {
						let template = self.config.templates[selected].clone();
						self.layout.set_layout(Some(template));
					}
					return Ok(EventState::Consumed);
				}
				Ok(EventState::NotConsumed)
			}
			Focus::Layout => {
				if key == Key::Left {
					self.focus = Focus::Explorer;
					return Ok(EventState::Consumed);
				}

				if self.layout.event(key).await?.is_consumed() {
					return Ok(EventState::Consumed);
				}

				if key == Key::Up {
					self.focus = Focus::Editor;
					return Ok(EventState::Consumed);
				}
				Ok(EventState::NotConsumed)
			}
			Focus::Editor => {
				if self.editor.event(key).await?.is_consumed() {
					return Ok(EventState::Consumed);
				}
				if key == Key::Left {
					self.focus = Focus::Explorer;
					return Ok(EventState::Consumed);
				}

				if key == Key::Down {
					self.focus = Focus::Layout;
					return Ok(EventState::Consumed);
				}
				Ok(EventState::NotConsumed)
			}
		}
	}
}

impl RenderAbleComponent for LayoutTabComponent<'_> {
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
		// let stack_chunks = Layout::default()
		// 	.direction(Direction::Vertical)
		// 	.constraints([Constraint::Length(3), Constraint::Length(main_chunks[0].height - 3)])
		// 	.split(main_chunks[1]);

		self.explorer.render(
			f,
			main_chunks[0],
			focused && matches!(self.focus, Focus::Explorer),
		)?;

		// self.editor.render(f, stack_chunks[0], focused && matches!(self.focus, Focus::Editor))?;
		self.layout.render(f, main_chunks[1], focused && matches!(self.focus, Focus::Layout))?;
		Ok(())
	}
}
