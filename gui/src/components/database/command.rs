use crate::{
	components::{render_container, RenderAbleComponent},
	config::Config,
	constants::{BORDER_TYPE, HIGHLIGHT_COLOR},
	events::{EventState, Key},
};
use anyhow::Result;
use tui::{
	backend::Backend,
	layout::Rect,
	style::{Color, Modifier, Style},
	text::Text,
	widgets::{Block, Borders, Paragraph},
	Frame,
};
use tui_textarea::{Input, Key as InputKey, TextArea};

enum Focus {
	Container,
	Textarea,
}

#[derive(Clone, Debug)]
pub struct Command {
	pub token: String,
	pub value: String,
}

pub struct CommandComponent {
	config: Config,
	text: Vec<char>,
	focus: Focus,
	invalid: (bool, String),
	pub commands: Vec<Command>,
}

impl CommandComponent {
	pub fn new(config: Config) -> Self {
		CommandComponent {
			config,
			text: vec![],
			focus: Focus::Container,
			invalid: (false, "".to_string()),
			commands: vec![],
		}
	}

	fn set_invalid(&mut self, invalid: bool, err: &str) {
		self.invalid = (invalid, err.to_string());
	}

	fn add_command(&mut self, command: Command) {
		self.commands.push(command);
	}

	fn handle_command(&mut self) {
		let mapped: Vec<_> = self.text.iter().map(|t| t.to_string()).collect();
		let complete = mapped.join("");
		let splitted = complete.split(' ');
		for token in splitted {
			match token {
				t if token.starts_with("COLUMN") | token.starts_with("TABLE") => {
					let value = t.split('=').nth(1);
					match value {
						Some(v) => self.add_command(Command {
							token: "COLUMN".to_string(),
							value: v.to_string(),
						}),
						None => {
							return self.set_invalid(true, "No COLUMN value found");
						}
					}
				}
				_ => return self.set_invalid(true, "Mismatch command"),
			}
		}

		self.set_invalid(false, "");
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

impl RenderAbleComponent for CommandComponent {
	fn render<B: Backend>(
		&self,
		f: &mut Frame<B>,
		rect: Rect,
		focused: bool,
	) -> Result<(), anyhow::Error> {
		let keycode = match self.focus {
			Focus::Container => "ENTER",
			Focus::Textarea => "ESC",
		};
		let label = &format!("Command [{}]", keycode);
		if matches!(self.focus, Focus::Container) && self.text.is_empty() {
			let mut placeholder = Text::from("Press Enter to write a command");
			placeholder.patch_style(Style::default().fg(Color::DarkGray));

			let widget = Paragraph::new(placeholder).block(render_container(label, focused));
			f.render_widget(widget, rect);
		} else if matches!(self.focus, Focus::Textarea) || !self.text.is_empty() {
			let style = Style::default().bg(HIGHLIGHT_COLOR).add_modifier(Modifier::BOLD);
			let mut textarea = TextArea::default();
			textarea.set_cursor_style(style);

			let (invalid, err) = &self.invalid;
			if *invalid {
				let label = &format!("{} [{}]", err, keycode);
				let container = Block::default()
					.borders(Borders::ALL)
					.style(Style::default().fg(Color::Red))
					.title(label.clone())
					.border_type(BORDER_TYPE);
				textarea.set_style(Style::default().fg(Color::Red));
				textarea.set_block(container);
			} else {
				let container = render_container(label, focused);
				textarea.set_block(container);
			};

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
