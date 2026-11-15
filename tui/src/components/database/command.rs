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
	text::{Span, Spans, Text},
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

const COMMAND_KEYWORDS: &[&str] = &["COLUMN=", "TABLE=", "TREE=", "PREFIX=", "SUFFIX="];

/// Find the best matching keyword for the current input token.
/// Returns the full keyword (including '=') if the input is a prefix of it.
fn get_completion(input: &str) -> Option<String> {
	// Get the last token being typed (after the last space)
	let last_token = input.rsplit(' ').next().unwrap_or("");
	if last_token.is_empty() {
		return None;
	}

	// Find keywords that start with the current input (case-insensitive)
	let upper = last_token.to_uppercase();
	for keyword in COMMAND_KEYWORDS {
		if keyword.starts_with(&upper) && *keyword != upper {
			return Some(keyword.to_string());
		}
	}
	None
}

pub struct CommandComponent {
	config: Config,
	text: Vec<char>,
	focus: Focus,
	invalid: (bool, String),
	pub commands: Vec<Command>,
	completion: Option<String>,
}

impl CommandComponent {
	pub fn new(config: Config) -> Self {
		CommandComponent {
			config,
			text: vec![],
			focus: Focus::Container,
			invalid: (false, "".to_string()),
			commands: vec![],
			completion: None,
		}
	}

	fn set_invalid(&mut self, invalid: bool, err: &str) {
		self.invalid = (invalid, err.to_string());
	}

	fn add_command(&mut self, command: Command) {
		self.commands.push(command);
	}

	fn update_completion(&mut self) {
		let input: String = self.text.iter().collect();
		self.completion = get_completion(&input);
	}

	fn accept_completion(&mut self) {
		if let Some(ref keyword) = self.completion {
			// Replace the last token with the full keyword
			let input: String = self.text.iter().collect();
			let last_space = input.rfind(' ');
			let prefix = match last_space {
				Some(idx) => &input[..=idx],
				None => "",
			};
			let new_text = format!("{}{}", prefix, keyword);
			self.text = new_text.chars().collect();
			self.completion = None;
		}
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
				t if token.starts_with("TREE") => {
					let value = t.split('=').nth(1);
					match value {
						Some(v) => self.add_command(Command {
							token: "TREE".to_string(),
							value: v.to_string(),
						}),
						None => {
							return self.set_invalid(true, "No TREE value found");
						}
					}
				}
				t if token.starts_with("PREFIX") => {
					let value = t.split('=').nth(1);
					match value {
						Some(v) => self.add_command(Command {
							token: "PREFIX".to_string(),
							value: v.replace('"', "").to_string(),
						}),
						None => {
							return self.set_invalid(true, "No PREFIX value found");
						}
					}
				}
				t if token.starts_with("SUFFIX") => {
					let value = t.split('=').nth(1);
					match value {
						Some(v) => self.add_command(Command {
							token: "SUFFIX".to_string(),
							value: v.replace('"', "").to_string(),
						}),
						None => {
							return self.set_invalid(true, "No SUFFIX value found");
						}
					}
				}
				_ => return self.set_invalid(true, "Mismatch command"),
			}
		}

		self.set_invalid(false, "");
	}

	pub fn reset_command(&mut self) {
		self.commands = vec![];
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
				Key::Tab => {
					self.accept_completion();
					return Ok(EventState::Consumed);
				}
				Key::Char(v) => {
					self.text.push(v);
					self.update_completion();
					return Ok(EventState::Consumed);
				}
				k if k == self.config.key_config.backspace => {
					self.text.pop();
					self.update_completion();
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

			// Show completion suggestion as shadow text in the title
			let hint_label = if !*invalid {
				self.completion.as_ref().map(|completion| {
					let input: String = self.text.iter().collect();
					let last_token = input.rsplit(' ').next().unwrap_or("");
					let remaining = &completion[last_token.len()..];
					format!("{} [{}] (Tab: {})", label, keycode, remaining)
				})
			} else {
				None
			};

			if let Some(ref hint) = hint_label {
				let container = render_container(hint, focused);
				textarea.set_block(container);
			}

			let textarea_widget = textarea.widget();
			f.render_widget(textarea_widget, rect);
		}

		Ok(())
	}
}
