use crate::{
	components::{render_container, RenderAbleComponent},
	config::Config,
	constants::HIGHLIGHT_COLOR,
	events::{EventState, Key},
	ui::StatefulList,
	utils::get_absolute_path,
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

enum Focus {
	Container,
	List,
}

pub struct DatabaseExplorerComponent<'a> {
	config: Config,
	pub list: StatefulList<'a>,
	focus: Focus,
}

fn build_list(config: Config, database: String) -> StatefulList<'static> {
	let databases: Vec<_> = config.databases.get(&database).unwrap().to_vec();
	let items: Vec<_> = databases
		.iter()
		.map(|database| {
			ListItem::new(Spans::from(vec![Span::styled(
				get_absolute_path(&database.path.clone()),
				Style::default(),
			)]))
		})
		.collect();

	let mut list_state = ListState::default();
	list_state.select(Some(0));
	StatefulList::with_items(items, None)
}

impl<'a> DatabaseExplorerComponent<'a> {
	pub fn state(&self) -> ListState {
		self.list.state.clone()
	}

	pub fn set_database(&mut self, database: String) {
		self.list = build_list(self.config.clone(), database);
	}

	pub fn new(config: Config) -> Self {
		let list = if !config.databases.is_empty() {
			let databases: Vec<_> = config.databases.keys().collect();
			let db = databases[0].to_string();
			build_list(config.clone(), db)
		} else {
			StatefulList::default()
		};
		DatabaseExplorerComponent {
			list,
			config,
			focus: Focus::Container,
		}
	}

	pub async fn event(&mut self, key: Key) -> Result<EventState> {
		match self.focus {
			Focus::Container => {
				if key == Key::Enter {
					self.focus = Focus::List;
					self.list.first();
					return Ok(EventState::Consumed);
				}
			}
			Focus::List => match key {
				Key::Esc => {
					self.list.state = ListState::default();
					self.focus = Focus::Container;
					return Ok(EventState::Consumed);
				}
				Key::Up => {
					self.list.previous();
					return Ok(EventState::Consumed);
				}
				Key::Down => {
					self.list.next();
					return Ok(EventState::Consumed);
				}
				_ => {}
			},
		}
		Ok(EventState::NotConsumed)
	}
}

impl<'a> RenderAbleComponent for DatabaseExplorerComponent<'a> {
	fn render<B: Backend>(
		&self,
		f: &mut Frame<B>,
		rect: Rect,
		focused: bool,
	) -> Result<(), anyhow::Error> {
		let keycode = match self.focus {
			Focus::Container => "ENTER",
			Focus::List => "ESC",
		};
		let label = &format!("Explorer [{}]", keycode);
		let list = List::new(self.list.items.clone())
			.block(render_container(label, focused))
			.highlight_style(Style::default().fg(HIGHLIGHT_COLOR).add_modifier(Modifier::BOLD));

		f.render_stateful_widget(list, rect, &mut self.list.state.clone());
		Ok(())
	}
}
