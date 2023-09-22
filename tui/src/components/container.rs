use tui::{
	style::{Color, Style},
	widgets::{Block, Borders},
};

use crate::constants::{BORDER_TYPE, PRIMARY_COLOR};

pub fn render_container<'a>(title: &'a str, focused: bool) -> Block<'a> {
	Block::<'a>::default()
		.borders(Borders::ALL)
		.style(Style::default().fg(if focused {
			Color::White
		} else {
			PRIMARY_COLOR
		}))
		.title(title)
		.border_type(BORDER_TYPE)
}
