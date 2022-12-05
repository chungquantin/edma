use tui::{
	backend::Backend,
	layout::Rect,
	widgets::{Block, Borders},
	Frame,
};
use tui_tree_widget::{Tree, TreeItem, TreeState};

use super::{container::render_container, RenderAbleComponent};

pub struct FileExplorerComponent {}

impl FileExplorerComponent {
	pub fn new() -> Self {
		FileExplorerComponent {}
	}
}
impl RenderAbleComponent for FileExplorerComponent {
	fn render<B: Backend>(
		&self,
		f: &mut Frame<B>,
		rect: Rect,
		focused: bool,
	) -> Result<(), anyhow::Error> {
		let mut item = TreeItem::new_leaf("parent");
		let child = TreeItem::new_leaf("child");
		item.add_child(child);
		let items = vec![item];
		let mut state = TreeState::default();

		let tree_widget = Tree::new(items.clone())
			.block(Block::default().borders(Borders::ALL).title("Tree Widget"));

		f.render_stateful_widget(tree_widget, rect, &mut state);
		f.render_widget(render_container("Explorer", focused), rect);
		Ok(())
	}
}
