use tui::{buffer::Buffer, layout::Rect, style::Style, widgets::Widget};

pub struct MultiLineTextBuffer<'a> {
	buffer_text: &'a str,
	// TODO: also encompass the area (Rect) that it gets rendered on
	// rect: Rect,
}

impl<'a> MultiLineTextBuffer<'a> {
	pub fn new(buffer_text: &'a str) -> Self {
		Self { buffer_text }
	}
}

impl<'a> Widget for MultiLineTextBuffer<'a> {
	fn render(self, area: Rect, buf: &mut Buffer) {
		let buf_area = buf.area();
		buf.set_string(
			area.top(),
			area.left(),
			self.buffer_text,
			Style::default(),
		);
		// println!("==> {area:?} area.top(): {}, area.left(): {}, area.bottom(): {}, area.right(): {}", area.top(), area.left(), area.bottom(), area.right());
		// todo!()
	}
}
