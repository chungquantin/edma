use easy_cast::CastFloat;
use std::convert::TryFrom;
use tui::{
	backend::Backend,
	buffer::Buffer,
	layout::{Margin, Rect},
	style::{Color, Style},
	symbols::{block::FULL, line::DOUBLE_VERTICAL},
	widgets::Widget,
	Frame,
};

struct Scrollbar {
	max: u16,
	pos: u16,
	style_bar: Style,
	style_pos: Style,
	inside: bool,
	border: bool,
}

impl Scrollbar {
	fn new(max: usize, pos: usize, border: bool, inside: bool) -> Self {
		Self {
			max: u16::try_from(max).unwrap_or_default(),
			pos: u16::try_from(pos).unwrap_or_default(),
			style_pos: Style::default(),
			style_bar: Style::default(),
			inside,
			border,
		}
	}
}

impl Widget for Scrollbar {
	fn render(self, area: Rect, buf: &mut Buffer) {
		if area.height <= 2 {
			return;
		}

		if self.max == 0 {
			return;
		}

		let right = if self.inside {
			area.right().saturating_sub(1)
		} else {
			area.right()
		};
		if right <= area.left() {
			return;
		};

		let (bar_top, bar_height) = {
			let scrollbar_area = area.inner(&Margin {
				horizontal: 0,
				vertical: u16::from(self.border),
			});

			(scrollbar_area.top(), scrollbar_area.height)
		};

		for y in bar_top..(bar_top + bar_height) {
			buf.set_string(right, y, DOUBLE_VERTICAL, self.style_bar);
		}

		let progress = f32::from(self.pos) / f32::from(self.max);
		let progress = if progress > 1.0 {
			1.0
		} else {
			progress
		};
		let pos = f32::from(bar_height) * progress;

		let pos: u16 = pos.cast_nearest();
		let pos = pos.saturating_sub(1);

		buf.set_string(right, bar_top + pos, FULL, self.style_pos);
	}
}

pub fn draw_scrollbar<B: Backend>(
	f: &mut Frame<B>,
	r: Rect,
	max: usize,
	pos: usize,
	border: bool,
	inside: bool,
) {
	let mut widget = Scrollbar::new(max, pos, border, inside);
	widget.style_pos = Style::default().fg(Color::Blue);
	f.render_widget(widget, r);
}
