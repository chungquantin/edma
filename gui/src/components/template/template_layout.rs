use anyhow::Result;
use tui::{
	backend::Backend,
	layout::{Constraint, Rect},
	style::{Color, Modifier, Style},
	widgets::{Cell, Row, Table},
	Frame,
};

use crate::{
	components::{render_container, RenderAbleComponent},
	config::Config,
	constants::HIGHLIGHT_COLOR,
	events::{EventState, Key},
	ui::StatefulTable,
	utils::{ByteLayout, LayoutTemplate},
};

pub struct TemplateLayoutComponent {
	config: Config,
	layout: Option<LayoutTemplate>,
	table: Option<StatefulTable>,
}

fn build_table(layouts: Vec<ByteLayout>) -> StatefulTable {
	let mut items = vec![];
	for (index, layout) in layouts.iter().enumerate() {
		items.push(vec![
			index.to_string(),
			layout.name.clone(),
			layout.variant.to_string(),
			if layout.from == usize::MIN {
				"Start".to_string()
			} else {
				layout.from.to_string()
			},
			if layout.to == usize::MAX {
				"End".to_string()
			} else {
				layout.to.to_string()
			},
		])
	}
	StatefulTable::default()
		.with_items(items.to_vec())
		.with_headers(vec!["#", "Name", "Variant", "From", "To"])
		.build()
}

impl TemplateLayoutComponent {
	pub fn set_layout(&mut self, layout: Option<LayoutTemplate>) {
		self.layout = layout.clone();
		if let Some(l) = layout {
			self.table = Some(build_table(l.layout));
		}
	}

	pub fn new(config: Config) -> Self {
		TemplateLayoutComponent {
			config,
			layout: None,
			table: None,
		}
	}

	pub async fn event(&mut self, _key: Key) -> Result<EventState> {
		Ok(EventState::NotConsumed)
	}
}

impl RenderAbleComponent for TemplateLayoutComponent {
	fn render<B: Backend>(
		&self,
		f: &mut Frame<B>,
		rect: Rect,
		focused: bool,
	) -> Result<(), anyhow::Error> {
		if let Some(t) = &self.table {
			let header_cells =
				t.headers.iter().map(|h| Cell::from(*h).style(Style::default().fg(Color::Black)));
			let normal_style = Style::default().bg(Color::DarkGray);
			let header = Row::new(header_cells).style(normal_style).height(1).bottom_margin(1);

			let rows = t.items.iter().map(|item| {
				let height = item
					.iter()
					.map(|content| content.chars().filter(|c| *c == '\n').count())
					.max()
					.unwrap_or(0) + 1;
				let cells = item.iter().map(|c| Cell::from(c.clone()));
				Row::new(cells).height(height as u16).bottom_margin(1)
			});

			let table = Table::new(rows)
				.header(header)
				.block(render_container("Layout", focused))
				.highlight_style(Style::default().fg(HIGHLIGHT_COLOR).add_modifier(Modifier::BOLD))
				.widths(&[
					Constraint::Percentage(5),
					Constraint::Percentage(20),
					Constraint::Percentage(35),
					Constraint::Percentage(20),
					Constraint::Percentage(20),
				]);
			f.render_stateful_widget(table, rect, &mut t.state.clone());
		} else {
			f.render_widget(render_container("Layout", focused), rect);
		}

		Ok(())
	}
}
