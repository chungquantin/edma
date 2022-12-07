use anyhow::Result;
use tui::{
	backend::Backend,
	layout::Rect,
	style::{Modifier, Style},
	Frame,
};
use tui_tree_widget::{Tree, TreeItem, TreeState};

use crate::{
	config::Config,
	constants::HIGHLIGHT_COLOR,
	events::{EventState, Key},
	ui::StatefulTree,
	utils::get_db_absolute_path,
};

use super::{container::render_container, RenderAbleComponent};

enum Focus {
	TreeView,
	Container,
}

type DatabaseInfo = (String, String);

pub struct DatabaseExplorerComponent<'a> {
	config: Config,
	pub tree: StatefulTree<'a>,
	focus: Focus,
	selected_index: usize,
	selected_database: Option<DatabaseInfo>,
	is_toggled: bool,
}

fn build_tree<'a>(config: Config) -> StatefulTree<'a> {
	let paths = config.databases.to_vec();
	let name = format!("RocksDB ({} databases)", paths.len());
	let mut item = TreeItem::new_leaf(name);
	for path in paths {
		let (_, path) = get_db_absolute_path(&path);
		let child = TreeItem::new_leaf(path);
		item.add_child(child);
	}

	StatefulTree::with_items(vec![item])
}

impl<'a> DatabaseExplorerComponent<'a> {
	pub fn selected_file(&self) -> Option<DatabaseInfo> {
		self.selected_database.clone()
	}

	pub fn is_selected(&self) -> bool {
		self.selected_database.is_some()
	}

	pub fn new(config: Config) -> Self {
		DatabaseExplorerComponent {
			tree: build_tree(config.clone()),
			config,
			focus: Focus::Container,
			selected_index: 0,
			selected_database: None,
			is_toggled: false,
		}
	}

	fn handle_toggle(&mut self) -> Result<EventState> {
		if self.is_dir() {
			self.tree.toggle();
			self.is_toggled = !self.is_toggled;
		}
		Ok(EventState::Consumed)
	}

	fn handle_escape(&mut self) -> Result<EventState> {
		self.tree.state = TreeState::default();
		self.focus = Focus::Container;
		self.selected_index = 0;
		self.selected_database = None;
		self.is_toggled = false;
		self.reset_tree();
		Ok(EventState::Consumed)
	}

	fn handle_select(&mut self) -> Result<EventState> {
		if !self.is_dir() {
			let index = self.selected_index.saturating_sub(1);
			let path = self.config.databases[index].clone();
			let database = get_db_absolute_path(&path);
			self.selected_database = Some(database);
		} else {
			self.reset_tree();
		}
		Ok(EventState::Consumed)
	}

	fn handle_select_first(&mut self) -> Result<EventState> {
		self.tree.first();
		self.selected_index = 0;
		Ok(EventState::Consumed)
	}

	fn handle_select_last(&mut self) -> Result<EventState> {
		self.tree.last();
		self.selected_index = self.tree.items.len().saturating_sub(1);
		Ok(EventState::Consumed)
	}

	fn handle_up(&mut self) -> Result<EventState> {
		self.tree.up();
		if !self.is_dir() {
			self.selected_index = self.selected_index.saturating_sub(1);
		}
		self.handle_select()
	}

	fn handle_down(&mut self) -> Result<EventState> {
		self.tree.down();
		if self.is_toggled {
			self.selected_index =
				std::cmp::min(self.selected_index.saturating_add(1), self.config.databases.len());
		}
		self.handle_select()
	}

	fn reset_tree(&mut self) {
		self.selected_database = None;
		self.selected_index = 0;
	}

	fn is_dir(&self) -> bool {
		self.selected_index == 0
	}

	pub async fn event(&mut self, key: Key) -> Result<EventState> {
		if matches!(self.focus, Focus::Container) && key == Key::Enter {
			self.focus = Focus::TreeView;
			self.tree.first();
			return Ok(EventState::Consumed);
		}

		if matches!(self.focus, Focus::TreeView) {
			match key {
				Key::Char('\n' | ' ') => return self.handle_toggle(),
				Key::Esc => return self.handle_escape(),
				Key::Down => return self.handle_down(),
				Key::Up => return self.handle_up(),
				Key::Home => return self.handle_select_first(),
				Key::End => return self.handle_select_last(),
				_ => {}
			}
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
		let tree_widget = Tree::new(self.tree.items.clone())
			.block(render_container("Explorer", focused))
			.highlight_style(Style::default().fg(HIGHLIGHT_COLOR).add_modifier(Modifier::BOLD));

		f.render_stateful_widget(tree_widget, rect, &mut self.tree.state.clone());

		Ok(())
	}
}
