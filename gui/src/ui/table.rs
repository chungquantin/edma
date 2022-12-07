use tui::widgets::TableState;

#[derive(Default, Clone)]
pub struct StatefulTable {
	pub state: TableState,
	pub headers: Vec<&'static str>,
	pub items: Vec<Vec<String>>,
}

impl StatefulTable {
	pub fn with_items(&mut self, items: Vec<Vec<String>>) -> &mut Self {
		self.items = items;
		self
	}

	pub fn with_headers(&mut self, headers: Vec<&'static str>) -> &mut Self {
		self.headers = headers;
		self
	}

	pub fn build(&self) -> Self {
		self.clone()
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
