use anyhow::Result;
use db::{Datastore, KeyValuePair, SimpleTransaction};
use tui::{
	backend::Backend,
	layout::{Alignment, Constraint, Direction, Layout, Rect},
	style::{Color, Modifier, Style},
	text::{Span, Spans},
	widgets::{Cell, Paragraph, Row, Table, TableState},
	Frame,
};

use crate::{
	config::Config,
	constants::HIGHLIGHT_COLOR,
	events::{EventState, Key},
	ui::StatefulTable,
};

use super::{container::render_container, PreviewComponent, RenderAbleComponent};

enum Focus {
	Table,
	Container,
}

pub struct DatabaseEditorComponent<'a> {
	config: Config,
	preview: PreviewComponent<'a>,
	table: StatefulTable,
	pairs: Vec<KeyValuePair>,
	focus: Focus,
	// scroll: VerticalScroll,
}

async fn scan_from_path(path: &str) -> Vec<KeyValuePair> {
	let mut result = vec![];
	let ds = Datastore::new(path);
	let tx = ds.transaction(false).unwrap();
	let data = tx.iterate(None).await.unwrap();
	for pair in data {
		result.push(pair.unwrap());
	}
	result
}

fn build_table(pairs: Vec<KeyValuePair>) -> StatefulTable {
	let mut items = vec![];
	for (index, (key, value)) in pairs.iter().enumerate() {
		let index = format!("{:?}", index);
		let key = format!("{:?}", key.to_vec());
		let value = format!("{:?}", value.to_vec());
		items.push(vec![index, key, value])
	}
	StatefulTable::with_items(items.to_vec())
}

impl DatabaseEditorComponent<'_> {
	pub async fn scan_database(&mut self, name: &str, path: &str) {
		let db_path = format!("{}:{}", name, path);
		let pairs = scan_from_path(&db_path).await;
		self.table = build_table(pairs.to_vec());
		self.pairs = pairs;
	}

	fn pairs_empty(&self) -> bool {
		self.pairs.is_empty()
	}

	fn generate_label(&self) -> String {
		format!("Editor ({} key-value pairs)", self.pairs.len())
	}

	pub fn new(config: Config) -> Self {
		DatabaseEditorComponent {
			preview: PreviewComponent::new(config.clone()),
			// scroll: VerticalScroll::new(false, false),
			pairs: vec![],
			table: StatefulTable::default(),
			focus: Focus::Container,
			config,
		}
	}

	fn update_preview(&mut self) {
		match self.table.state.selected() {
			Some(selected) if !self.pairs_empty() => {
				let pair = Some(self.pairs[selected].clone());
				self.preview.set_pair(pair)
			}
			_ => self.preview.set_pair(None),
		}
	}

	fn handle_enter(&mut self) -> Result<EventState> {
		self.table.state = TableState::default();
		self.focus = Focus::Table;
		self.update_preview();
		Ok(EventState::Consumed)
	}

	fn handle_escape(&mut self) -> Result<EventState> {
		self.table.state = TableState::default();
		self.focus = Focus::Container;
		self.update_preview();
		Ok(EventState::Consumed)
	}

	fn handle_prev(&mut self) -> Result<EventState> {
		self.table.previous();
		self.update_preview();
		Ok(EventState::Consumed)
	}

	fn handle_next(&mut self) -> Result<EventState> {
		self.table.next();
		self.focus = Focus::Table;
		self.update_preview();
		Ok(EventState::Consumed)
	}

	pub async fn event(&mut self, key: Key) -> Result<EventState> {
		if self.preview.event(key).await?.is_consumed() {
			return Ok(EventState::Consumed);
		}

		match key {
			Key::Enter => return self.handle_enter(),
			Key::Esc => return self.handle_escape(),
			_ if matches!(key, Key::Up) && matches!(self.focus, Focus::Table) => {
				return self.handle_prev()
			}
			Key::Down => return self.handle_next(),
			_ => {}
		}

		Ok(EventState::NotConsumed)
	}
}

impl RenderAbleComponent for DatabaseEditorComponent<'_> {
	fn render<B: Backend>(
		&self,
		f: &mut Frame<B>,
		rect: Rect,
		focused: bool,
	) -> Result<(), anyhow::Error> {
		let mut chunks = Layout::default()
			.direction(Direction::Vertical)
			.constraints([Constraint::Percentage(100), Constraint::Percentage(0)])
			.split(rect);

		if !self.pairs_empty() {
			if self.table.state.selected().is_some() && self.preview.pair().is_some() {
				chunks = Layout::default()
					.direction(Direction::Vertical)
					.constraints([Constraint::Percentage(60), Constraint::Percentage(40)])
					.split(rect);
				self.preview.render(f, chunks[1], focused).unwrap();
			}

			let header_cells = ["#", "Key", "Value"]
				.iter()
				.map(|h| Cell::from(*h).style(Style::default().fg(Color::Black)));
			let normal_style = Style::default().bg(Color::DarkGray);
			let header = Row::new(header_cells).style(normal_style).height(1).bottom_margin(1);

			let rows = self.table.items.iter().map(|item| {
				let height = item
					.iter()
					.map(|content| content.chars().filter(|c| *c == '\n').count())
					.max()
					.unwrap_or(0) + 1;
				let cells = item.iter().map(|c| Cell::from(c.clone()));
				Row::new(cells).height(height as u16).bottom_margin(1)
			});

			let label = self.generate_label();
			let table = Table::new(rows)
				.header(header)
				.block(render_container(&label, focused))
				.highlight_style(Style::default().fg(HIGHLIGHT_COLOR).add_modifier(Modifier::BOLD))
				.widths(&[
					Constraint::Percentage(5),
					Constraint::Percentage(35),
					Constraint::Percentage(60),
				]);
			f.render_stateful_widget(table, chunks[0], &mut self.table.state.clone());
		} else {
			let not_found_widget = Paragraph::new(vec![
				Spans::from(vec![Span::raw("")]),
				Spans::from(vec![Span::raw("No data found in this database")]),
			])
			.alignment(Alignment::Center)
			.block(render_container("Home", focused));
			f.render_widget(not_found_widget, chunks[0]);
		};

		Ok(())
	}
}
