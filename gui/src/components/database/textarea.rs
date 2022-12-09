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

pub struct TextareaComponent {
	config: Config,
	text: Vec<char>,
	focus: Focus,
}

impl TextareaComponent {
	pub fn new(config: Config) -> Self {
		TextareaComponent {
			config,
			text: vec![],
			focus: Focus::Container,
		}
	}

	fn handle_command(&mut self) {
		unimplemented!()
	}

	pub async fn event(&mut self, key: Key) -> Result<EventState> {
		match self.focus {
			Focus::Container => {
				if key == self.config.key_config.enter {
					self.focus = Focus::Textarea;
					return Ok(EventState::Consumed);
				}
			}
			Focus::Textarea => match key {
				k if k == self.config.key_config.escape => {
					self.focus = Focus::Container;
					return Ok(EventState::Consumed);
				}
				k if k == self.config.key_config.enter => {
					self.handle_command();
					return Ok(EventState::Consumed);
				}
				Key::Char(v) => {
					self.text.push(v);
					return Ok(EventState::Consumed);
				}
				k if k == self.config.key_config.backspace => {
					self.text.pop();
					return Ok(EventState::Consumed);
				}
				_ => {}
			},
		}

		Ok(EventState::NotConsumed)
	}
}

impl RenderAbleComponent for TextareaComponent {
	fn render<B: Backend>(
		&self,
		f: &mut Frame<B>,
		rect: Rect,
		focused: bool,
	) -> Result<(), anyhow::Error> {
		if matches!(self.focus, Focus::Container) && self.text.is_empty() {
			let mut placeholder = Text::from("Press Enter to write a command");
			placeholder.patch_style(Style::default().fg(Color::DarkGray));

			let keycode = match self.focus {
				Focus::Container => "ENTER",
				Focus::Textarea => "ESC",
			};
			let label = &format!("Command [{}]", keycode);
			let widget = Paragraph::new(placeholder).block(render_container(label, focused));
			f.render_widget(widget, rect);
		} else if matches!(self.focus, Focus::Textarea) || !self.text.is_empty() {
			let style = Style::default().bg(HIGHLIGHT_COLOR).add_modifier(Modifier::BOLD);
			let mut textarea = TextArea::default();
			textarea.set_cursor_style(style);
			let keycode = match self.focus {
				Focus::Container => "ENTER",
				Focus::Textarea => "ESC",
			};
			let label = &format!("Command [{}]", keycode);
			textarea.set_block(render_container(label, focused));

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

		Ok(())
	}
}
