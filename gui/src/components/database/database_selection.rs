use crate::{
	components::{render_container, RenderAbleComponent},
	config::Config,
	constants::HIGHLIGHT_COLOR,
	events::{EventState, Key},
	ui::StatefulList,
};
use anyhow::Result;
use tui::{
	backend::Backend,
	layout::Rect,
	style::{Modifier, Style},
	text::{Span, Spans},
	widgets::{List, ListItem, ListState},
	Frame,
};

pub struct DatabaseSelectionComponent<'a> {
	config: Config,
	pub list: StatefulList<'a>,
}

fn build_list(config: Config) -> StatefulList<'static> {
	let databases: Vec<_> = config.databases.keys().collect();
	let items: Vec<_> = databases
		.iter()
		.map(|database| {
			let cloned = <&std::string::String>::clone(database).clone();
			ListItem::new(Spans::from(vec![Span::styled(cloned, Style::default())]))
		})
		.collect();

	let mut list_state = ListState::default();
	list_state.select(Some(0));
	let mut state = ListState::default();
	state.select(Some(0));
	StatefulList::with_items(items, Some(state.clone()))
}

impl<'a> DatabaseSelectionComponent<'a> {
	pub fn state(&self) -> ListState {
		self.list.state.clone()
	}

	pub fn new(config: Config) -> Self {
		DatabaseSelectionComponent {
			list: build_list(config.clone()),
			config,
		}
	}

	pub async fn event(&mut self, key: Key) -> Result<EventState> {
		match key {
			Key::Char('9') => {
				self.list.previous();
				return Ok(EventState::Consumed);
			}
			Key::Char('0') => {
				self.list.next();
				return Ok(EventState::Consumed);
			}
			_ => {}
		}
		Ok(EventState::NotConsumed)
	}
}

impl<'a> RenderAbleComponent for DatabaseSelectionComponent<'a> {
	fn render<B: Backend>(
		&self,
		f: &mut Frame<B>,
		rect: Rect,
		focused: bool,
	) -> Result<(), anyhow::Error> {
		let list = List::new(self.list.items.clone())
			.block(render_container("Databases [9-0]", focused))
			.highlight_style(Style::default().fg(HIGHLIGHT_COLOR).add_modifier(Modifier::BOLD));

		f.render_stateful_widget(list, rect, &mut self.list.state.clone());
		Ok(())
	}
}
