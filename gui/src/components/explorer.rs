use anyhow::Result;
use tui::{
	backend::Backend,
	layout::Rect,
	style::{Color, Modifier, Style},
	Frame,
};
use tui_tree_widget::{Tree, TreeItem};

use crate::{config::Config, events::Key, utils::StatefulTree};

use super::{container::render_container, EventState, RenderAbleComponent};

enum Focus {
	TreeView,
	Container,
}

pub struct FileExplorerComponent<'a> {
	config: Config,
	tree: StatefulTree<'a>,
	focus: Focus,
}

fn build_tree<'a>(config: Config) -> StatefulTree<'a> {
	let mut item = TreeItem::new_leaf("rocksdb");
	let paths = config.paths.to_vec();
	for path in paths {
		let child = TreeItem::new_leaf(path);
		item.add_child(child);
	}
	
	StatefulTree::with_items(vec![item])
}

impl<'a> FileExplorerComponent<'a> {
	pub fn new(config: Config) -> Self {
		FileExplorerComponent {
			tree: build_tree(config.clone()),
			config,
			focus: Focus::Container,
		}
	}

	pub async fn event(&mut self, key: Key) -> Result<EventState> {
		if matches!(self.focus, Focus::Container) && key == Key::Enter {
  				self.focus = Focus::TreeView;
  				self.tree.first();
  			}
		if matches!(self.focus, Focus::TreeView) {
			match key {
				Key::Char('\n' | ' ') => {
					self.tree.toggle();
					return Ok(EventState::Consumed);
				}
				Key::Left => {
					self.tree.left();
					return Ok(EventState::Consumed);
				}
				Key::Right => {
					self.tree.right();
					return Ok(EventState::Consumed);
				}
				Key::Down => {
					self.tree.down();
					return Ok(EventState::Consumed);
				}
				Key::Up => {
					self.tree.up();
					return Ok(EventState::Consumed);
				}
				Key::Home => {
					self.tree.first();
					return Ok(EventState::Consumed);
				}
				Key::End => {
					self.tree.last();
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
				Style::default()
					.fg(Color::Black)
					.bg(Color::LightGreen)
					.add_modifier(Modifier::BOLD),
			)
			.highlight_symbol(">> ");

		f.render_stateful_widget(tree_widget, rect, &mut self.tree.state.clone());

		Ok(())
	}
}
