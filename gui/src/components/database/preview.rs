use anyhow::Result;
use db::KeyValuePair;
use tui::{
	backend::Backend,
	layout::{Constraint, Direction, Layout, Rect},
	style::{Modifier, Style},
	text::{Span, Spans},
	widgets::{List, ListItem, ListState, Paragraph, Wrap},
	Frame,
};

use crate::{
	components::{render_container, RenderAbleComponent},
	config::Config,
	constants::HIGHLIGHT_COLOR,
	events::{EventState, Key},
	ui::StatefulList,
	utils::FromLayoutVariant,
};

pub struct PreviewComponent<'a> {
	config: Config,
	pair: Option<KeyValuePair>,
	key_layout: StatefulList<'a>,
	value_layout: StatefulList<'a>,
}

fn build_list(config: Config) -> StatefulList<'static> {
	let items: Vec<_> = config
		.templates
		.iter()
		.map(|layout| {
			ListItem::new(Spans::from(vec![Span::styled(layout.name.clone(), Style::default())]))
		})
		.collect();

	let mut list_state = ListState::default();
	list_state.select(Some(0));
	StatefulList::with_items(items, Some(list_state))
}

impl PreviewComponent<'_> {
	pub fn new(config: Config) -> Self {
		PreviewComponent {
			key_layout: build_list(config.clone()),
			value_layout: build_list(config.clone()),
			pair: None,
			config,
		}
	}

	pub fn pair(&self) -> Option<KeyValuePair> {
		self.pair.clone()
	}

	pub fn set_pair(&mut self, pair: Option<KeyValuePair>) {
		self.pair = pair;
	}

	fn deserialize_key(&self, layout: &StatefulList, raw: Vec<u8>) -> Vec<(String, String)> {
		let selected_layout = layout.state.selected();
		let default = ("*".to_string(), format!("{:?}", raw));
		let mut data = vec![default];
		if let Some(layout) = selected_layout {
			let index = layout;
			let l = &self.config.templates[index];
			let mut items: Vec<(String, String)> = vec![];
			for item in l.layout.iter() {
				let slice = raw[item.from..std::cmp::min(item.to, raw.len())].to_vec();
				let converted = slice.from_variant(item.variant.clone());
				items.push((item.name.clone(), converted));
			}
			data = items;
		}
		data
	}

	fn render_layout<B: Backend>(
		&self,
		f: &mut Frame<B>,
		rect: Rect,
		focused: bool,
		title: &str,
		layout: &StatefulList,
	) {
		let list = List::new(layout.items.clone())
			.block(render_container(title, focused))
			.highlight_style(Style::default().fg(HIGHLIGHT_COLOR).add_modifier(Modifier::BOLD));

		f.render_stateful_widget(list, rect, &mut layout.state.clone());
	}

	fn render_preview<B: Backend>(
		&self,
		f: &mut Frame<B>,
		rect: Rect,
		focused: bool,
		title: &str,
		layout: &StatefulList,
		bytes: Vec<u8>,
	) {
		let values = self.deserialize_key(layout, bytes);
		let mut spans = vec![];
		for (name, item) in values.iter() {
			spans.push(Span::styled(name, Style::default().fg(HIGHLIGHT_COLOR)));
			spans.push(Span::raw(":"));
			spans.push(Span::raw(item));
		}
		let content = Paragraph::new(vec![Spans::from(spans)])
			.wrap(Wrap {
				trim: true,
			})
			.block(render_container(title, focused));

		f.render_widget(content, rect);
	}

	fn render_key_layout<B: Backend>(&self, f: &mut Frame<B>, rect: Rect, focused: bool) {
		self.render_layout(f, rect, focused, "Key Layout", &self.key_layout);
	}

	fn render_key_preview<B: Backend>(&self, f: &mut Frame<B>, rect: Rect, focused: bool) {
		self.render_preview(
			f,
			rect,
			focused,
			"Key Preview",
			&self.key_layout,
			self.pair.clone().unwrap().0,
		);
	}

	fn render_value_layout<B: Backend>(&self, f: &mut Frame<B>, rect: Rect, focused: bool) {
		self.render_layout(f, rect, focused, "Value Layout", &self.value_layout);
	}

	fn render_value_preview<B: Backend>(&self, f: &mut Frame<B>, rect: Rect, focused: bool) {
		self.render_preview(
			f,
			rect,
			focused,
			"Value Preview",
			&self.value_layout,
			self.pair.clone().unwrap().1,
		);
	}

	pub async fn event(&mut self, key: Key) -> Result<EventState> {
		match key {
			Key::Char('h') => {
				self.key_layout.previous();
				return Ok(EventState::Consumed);
			}
			Key::Char('j') => {
				self.key_layout.next();
				return Ok(EventState::Consumed);
			}
			Key::Char('k') => {
				self.value_layout.previous();
				return Ok(EventState::Consumed);
			}
			Key::Char('l') => {
				self.value_layout.next();
				return Ok(EventState::Consumed);
			}
			_ => {}
		}
		Ok(EventState::NotConsumed)
	}
}

impl RenderAbleComponent for PreviewComponent<'_> {
	fn render<B: Backend>(
		&self,
		f: &mut Frame<B>,
		rect: Rect,
		focused: bool,
	) -> Result<(), anyhow::Error> {
		let main_chunks = Layout::default()
			.direction(Direction::Horizontal)
			.constraints([
				Constraint::Percentage(20),
				Constraint::Percentage(30),
				Constraint::Percentage(20),
				Constraint::Percentage(30),
			])
			.split(rect);

		self.render_key_layout(f, main_chunks[0], focused);
		self.render_key_preview(f, main_chunks[1], focused);
		self.render_value_layout(f, main_chunks[2], focused);
		self.render_value_preview(f, main_chunks[3], focused);

		Ok(())
	}
}
