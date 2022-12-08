use crate::{
	components::{render_container, RenderAbleComponent},
	config::Config,
	constants::HIGHLIGHT_COLOR,
	events::{EventState, Key},
};
use anyhow::Result;
use tui::{
	backend::Backend,
	layout::Rect,
	style::{Color, Modifier, Style},
	text::Text,
	widgets::Paragraph,
	Frame,
};
use tui_textarea::{Input, Key as InputKey, TextArea};

enum Focus {
	Container,
	Textarea,
}

pub struct CommandEditorComponent {
	config: Config,
	text: Vec<char>,
	focus: Focus,
}

impl CommandEditorComponent {
	pub fn new(config: Config) -> Self {
		CommandEditorComponent {
			config,
			text: vec![],
			focus: Focus::Container,
		}
	}

	pub async fn event(&mut self, key: Key) -> Result<EventState> {
		match self.focus {
			Focus::Container => {
				if key == Key::Enter {
					self.focus = Focus::Textarea;
					return Ok(EventState::Consumed);
				}
			}
			Focus::Textarea => match key {
				Key::Esc => {
					self.focus = Focus::Container;
					return Ok(EventState::Consumed);
				}
				Key::Enter => {
					return Ok(EventState::Consumed);
				}
				Key::Char(v) => {
					self.text.push(v);
					return Ok(EventState::Consumed);
				}
				Key::Backspace => {
					self.text.pop();
					return Ok(EventState::Consumed);
				}
				_ => {}
			},
		}

		Ok(EventState::NotConsumed)
	}
}

impl RenderAbleComponent for CommandEditorComponent {
	fn render<B: Backend>(
		&self,
		f: &mut Frame<B>,
		rect: Rect,
		focused: bool,
	) -> Result<(), anyhow::Error> {
		match self.focus {
			Focus::Container => {
				let mut placeholder = Text::from("Press Enter to write a command");
				placeholder.patch_style(Style::default().fg(Color::DarkGray));

				let widget = Paragraph::new(placeholder).block(render_container("Editor", focused));
				f.render_widget(widget, rect);
			}
			Focus::Textarea => {
				let style = Style::default().bg(HIGHLIGHT_COLOR).add_modifier(Modifier::BOLD);
				let mut textarea = TextArea::default();
				textarea.set_cursor_style(style);
				textarea.set_block(render_container("Editor", focused));

				for c in self.text.iter() {
					textarea.input(Input {
						key: InputKey::Char(*c),
						ctrl: false,
						alt: false,
					});
				}

				let textarea_widget = textarea.widget();

				f.render_widget(textarea_widget, rect);
			}
		}

		Ok(())
	}
}
