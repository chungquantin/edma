use anyhow::Result;
use db::{Datastore, KeyValuePair, SimpleTransaction, TagBucket};
use tui::{
	backend::Backend,
	layout::{Alignment, Constraint, Direction, Layout, Rect},
	style::{Color, Modifier, Style},
	text::{Span, Spans},
	widgets::{Cell, Paragraph, Row, Table, TableState},
	Frame,
};

use crate::{
	components::{render_container, RenderAbleComponent},
	config::Config,
	constants::HIGHLIGHT_COLOR,
	events::{EventState, Key},
	ui::StatefulTable,
};

use super::PreviewComponent;

enum Focus {
	Table,
	Container,
}

pub struct DatabaseEditorComponent<'a> {
	config: Config,
	preview: PreviewComponent<'a>,
	table: StatefulTable,
	err: Option<String>,
	pairs: Vec<KeyValuePair>,
	focus: Focus,
}

fn build_table(pairs: Vec<KeyValuePair>) -> StatefulTable {
	let mut items = vec![];
	for (index, (key, value)) in pairs.iter().enumerate() {
		let index = format!("{:?}", index);
		let key = format!("{:?}", key.to_vec());
		let value = format!("{:?}", value.to_vec());
		items.push(vec![index, key, value])
	}
	StatefulTable::default()
		.with_items(items.to_vec())
		.with_headers(vec!["#", "Key", "Value"])
		.build()
}

impl DatabaseEditorComponent<'_> {
	async fn suffix_scan_from_path(
		&mut self,
		tags: TagBucket,
		path: &str,
		prefix: Vec<u8>,
	) -> Vec<KeyValuePair> {
		let mut result = vec![];
		let ds = Datastore::new(path);
		let tx = ds.transaction(false).await.unwrap();
		let data = tx.suffix_iterate(prefix, tags).await;
		self.clear_err();
		match data {
			Ok(pairs) => {
				for pair in pairs {
					result.push(pair.unwrap());
				}
			}
			Err(err) => {
				self.set_err(err.to_string());
			}
		}
		result
	}

	async fn prefix_scan_from_path(
		&mut self,
		tags: TagBucket,
		path: &str,
		prefix: Vec<u8>,
	) -> Vec<KeyValuePair> {
		let mut result = vec![];
		let ds = Datastore::new(path);
		let tx = ds.transaction(false).await.unwrap();
		let data = tx.prefix_iterate(prefix, tags).await;
		self.clear_err();
		match data {
			Ok(pairs) => {
				for pair in pairs {
					result.push(pair.unwrap());
				}
			}
			Err(err) => {
				self.set_err(err.to_string());
			}
		}
		result
	}

	async fn scan_from_path(&mut self, tags: TagBucket, path: &str) -> Vec<KeyValuePair> {
		let mut result = vec![];
		let ds = Datastore::new(path);
		let tx = ds.transaction(false).await.unwrap();
		let data = tx.iterate(tags).await;
		self.clear_err();
		match data {
			Ok(pairs) => {
				for pair in pairs {
					result.push(pair.unwrap());
				}
			}
			Err(err) => {
				self.set_err(err.to_string());
			}
		}

		result
	}

	pub async fn prefix_scan_database(
		&mut self,
		tags: TagBucket,
		name: &str,
		path: &str,
		prefix: Vec<u8>,
	) {
		let db_path = format!("{}:{}", name, path);
		let pairs = self.prefix_scan_from_path(tags, &db_path, prefix).await;
		self.table = build_table(pairs.to_vec());
		self.pairs = pairs;
	}

	pub async fn suffix_scan_database(
		&mut self,
		tags: TagBucket,
		name: &str,
		path: &str,
		suffix: Vec<u8>,
	) {
		let db_path = format!("{}:{}", name, path);
		let pairs = self.suffix_scan_from_path(tags, &db_path, suffix).await;
		self.table = build_table(pairs.to_vec());
		self.pairs = pairs;
	}

	pub async fn scan_database(&mut self, tags: TagBucket, name: &str, path: &str) {
		let db_path = format!("{}:{}", name, path);
		let pairs = self.scan_from_path(tags, &db_path).await;
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
			pairs: vec![],
			table: StatefulTable::default(),
			focus: Focus::Container,
			err: None,
			config,
		}
	}

	pub fn clear_err(&mut self) {
		self.err = None;
	}

	pub fn set_err(&mut self, err: String) {
		self.err = Some(err);
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

		match self.focus {
			Focus::Container => {
				if key == self.config.key_config.enter && !self.table.items.is_empty() {
					self.focus = Focus::Table;
					return self.handle_next();
				}
			}
			Focus::Table => match key {
				k if k == self.config.key_config.enter => return self.handle_enter(),
				k if k == self.config.key_config.escape => return self.handle_escape(),
				_ if key == self.config.key_config.up && matches!(self.focus, Focus::Table) => {
					return self.handle_prev()
				}
				k if k == self.config.key_config.down => return self.handle_next(),
				_ => {}
			},
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

		if !self.pairs_empty() && self.err.is_none() {
			if self.table.state.selected().is_some() && self.preview.pair().is_some() {
				chunks = Layout::default()
					.direction(Direction::Vertical)
					.constraints([Constraint::Percentage(60), Constraint::Percentage(40)])
					.split(rect);
				self.preview.render(f, chunks[1], focused).unwrap();
			}

			let header_cells = self
				.table
				.headers
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
				.highlight_style(
					Style::default()
						.bg(HIGHLIGHT_COLOR)
						.fg(Color::Black)
						.add_modifier(Modifier::BOLD),
				)
				.widths(&[
					Constraint::Percentage(5),
					Constraint::Percentage(35),
					Constraint::Percentage(60),
				]);
			f.render_stateful_widget(table, chunks[0], &mut self.table.state.clone());
		} else {
			let text =
				self.err.clone().unwrap_or_else(|| "No data found in this database".to_string());
			let not_found_widget = Paragraph::new(vec![
				Spans::from(vec![Span::raw("")]),
				Spans::from(vec![Span::raw(text)]),
			])
			.alignment(Alignment::Center)
			.block(render_container("Editor", focused));
			f.render_widget(not_found_widget, chunks[0]);
		};

		Ok(())
	}
}
