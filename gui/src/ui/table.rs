use tui::widgets::TableState;

#[derive(Default)]
pub struct StatefulTable {
	pub state: TableState,
	pub items: Vec<Vec<String>>,
}

impl StatefulTable {
	pub fn with_items(items: Vec<Vec<String>>) -> Self {
		Self {
			state: TableState::default(),
			items,
		}
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
