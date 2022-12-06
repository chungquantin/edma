use anyhow::Result;
use tui::{
	backend::Backend,
	layout::Rect,
	style::{Color, Modifier, Style},
	Frame,
};
use tui_tree_widget::{Tree, TreeItem, TreeState};

use crate::{
	config::Config,
	events::{EventState, Key},
	utils::{get_absolute_path, StatefulTree},
};

use super::{container::render_container, RenderAbleComponent};

enum Focus {
	TreeView,
	Container,
}

pub struct FileExplorerComponent<'a> {
	config: Config,
	pub tree: StatefulTree<'a>,
	focus: Focus,
	selected_index: usize,
	selected_file: Option<String>,
	is_toggled: bool,
}

fn build_tree<'a>(config: Config) -> StatefulTree<'a> {
	let paths = config.paths.to_vec();
	let name = String::from("RocksDB") + " (" + &paths.len().to_string() + " databases)";
	let mut item = TreeItem::new_leaf(name);
	for path in paths {
		let separator = ':';
		let chunk = path.split(separator).nth(1).unwrap();
		let path = get_absolute_path(chunk);
		let child = TreeItem::new_leaf(path);
		item.add_child(child);
	}

	StatefulTree::with_items(vec![item])
}

impl<'a> FileExplorerComponent<'a> {
	pub fn selected_file(&self) -> Option<String> {
		self.selected_file.clone()
	}

	pub fn is_selected(&self) -> bool {
		self.selected_file.is_some()
	}

	pub fn new(config: Config) -> Self {
		FileExplorerComponent {
			tree: build_tree(config.clone()),
			config,
			focus: Focus::Container,
			selected_index: 0,
			selected_file: None,
			is_toggled: false,
		}
	}

	fn reset_tree(&mut self) {
		self.selected_file = None;
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
				Key::Char('\n' | ' ') => {
					if self.is_dir() {
						self.tree.toggle();
						self.is_toggled = !self.is_toggled;
					}
					return Ok(EventState::Consumed);
				}
				Key::Esc => {
					self.tree.state = TreeState::default();
					self.focus = Focus::Container;
					self.reset_tree();
					return Ok(EventState::Consumed);
				}
				Key::Enter => {
					// Index at banner, not database
					if !self.is_dir() {
						let index = self.selected_index.saturating_sub(1);
						// self.tree.select(index);
						let separator = ':';
						let path = self.config.paths[index].clone();
						let chunk = path.split(separator).nth(1).unwrap();
						let abs = get_absolute_path(chunk);
						self.selected_file = Some(abs);
					} else {
						self.reset_tree();
					}
					return Ok(EventState::Consumed);
				}
				Key::Left => {
					self.tree.left();
					self.reset_tree();
					return Ok(EventState::Consumed);
				}
				Key::Down => {
					self.tree.down();
					if self.is_toggled {
						self.selected_index = self.selected_index.saturating_add(1);
					}
					return Ok(EventState::Consumed);
				}
				Key::Up => {
					self.tree.up();
					if !self.is_dir() {
						self.selected_index = self.selected_index.saturating_sub(1);
					}
					return Ok(EventState::Consumed);
				}
				Key::Home => {
					self.tree.first();
					self.selected_index = 0;
					return Ok(EventState::Consumed);
				}
				Key::End => {
					self.tree.last();
					self.selected_index = self.tree.items.len().saturating_sub(1);
					return Ok(EventState::Consumed);
				}
				_ => {}
			}
		}

		Ok(EventState::NotConsumed)
	}
}

impl<'a> RenderAbleComponent for FileExplorerComponent<'a> {
	fn render<B: Backend>(
		&self,
		f: &mut Frame<B>,
		rect: Rect,
		focused: bool,
	) -> Result<(), anyhow::Error> {
		let tree_widget = Tree::new(self.tree.items.clone())
			.block(render_container("Explorer", focused))
			.highlight_style(
				Style::default().fg(Color::Black).bg(Color::Yellow).add_modifier(Modifier::BOLD),
			);

		f.render_stateful_widget(tree_widget, rect, &mut self.tree.state.clone());

		Ok(())
	}
}
