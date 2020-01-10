use num_rational::Ratio;
use tui::buffer::Buffer;
use tui::layout::Rect;
use tui::widgets::Widget;

use crate::update::UpdatableWidget;
use crate::widgets::block;

pub struct NetWidget {
	title: String,
	update_interval: Ratio<u64>,

	interfaces: String,
}

impl NetWidget {
	pub fn new(interfaces: String) -> NetWidget {
		NetWidget {
			title: " Network Usage ".to_string(),
			update_interval: Ratio::from_integer(1),

			interfaces,
		}
	}
}

impl UpdatableWidget for NetWidget {
	fn update(&mut self) {}

	fn get_update_interval(&self) -> Ratio<u64> {
		self.update_interval
	}
}

impl Widget for NetWidget {
	fn draw(&mut self, area: Rect, buf: &mut Buffer) {
		block::new().title(&self.title).draw(area, buf);
	}
}
