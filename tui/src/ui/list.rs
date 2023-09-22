use tui::widgets::{ListItem, ListState};

#[derive(Default)]
pub struct StatefulList<'a> {
	pub state: ListState,
	pub items: Vec<ListItem<'a>>,
}

impl<'a> StatefulList<'a> {
	pub fn with_items(items: Vec<ListItem<'a>>, state: Option<ListState>) -> Self {
		Self {
			state: state.unwrap_or_default(),
			items,
		}
	}

	pub fn first(&mut self) {
		self.state.select(Some(0));
	}

	pub fn next(&mut self) {
		let i = match self.state.selected() {
			Some(i) => {
				if i >= self.items.len().saturating_sub(1) {
					0
				} else {
					i.saturating_add(1)
				}
			}
			None => 0,
		};
		self.state.select(Some(i));
	}

	pub fn previous(&mut self) {
		let i = match self.state.selected() {
			Some(i) => {
				if i == 0 {
					self.items.len().saturating_sub(1)
				} else {
					i.saturating_sub(1)
				}
			}
			None => 0,
		};
		self.state.select(Some(i));
	}
}
