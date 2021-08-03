use std::fmt::Display;
use tui::{buffer::Buffer, layout::Rect, style::Style, widgets::Widget};

pub struct Label<T: Display>(pub T, pub Style);

impl<T: Display> Widget for Label<T> {
    fn render(self, area: Rect, buf: &mut Buffer) {
        let width = area.width as usize;
        let str = &format!("{:^w$}", self.0, w = width)[..width];

        buf.set_string(area.x, area.y, str, self.1);
    }
}
