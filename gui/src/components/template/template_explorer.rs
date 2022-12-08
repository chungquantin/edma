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

enum Focus {
	Container,
	List,
}

pub struct TemplateExplorerComponent<'a> {
	config: Config,
	pub list: StatefulList<'a>,
	focus: Focus,
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
	StatefulList::with_items(items, None)
}

impl<'a> TemplateExplorerComponent<'a> {
	pub fn new(config: Config) -> Self {
		TemplateExplorerComponent {
			list: build_list(config.clone()),
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

impl<'a> RenderAbleComponent for TemplateExplorerComponent<'a> {
	fn render<B: Backend>(
		&self,
		f: &mut Frame<B>,
		rect: Rect,
		focused: bool,
	) -> Result<(), anyhow::Error> {
		let list = List::new(self.list.items.clone())
			.block(render_container("Templates", focused))
			.highlight_style(Style::default().fg(HIGHLIGHT_COLOR).add_modifier(Modifier::BOLD));

		f.render_stateful_widget(list, rect, &mut self.list.state.clone());
		Ok(())
	}
}
