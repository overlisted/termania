use tui::{buffer::Buffer, layout::Rect, style::Color, widgets::Widget};

pub struct MapRowsView<'a> {
    pub rows: Vec<&'a [bool; 4]>,
    pub shift: usize,
    pub going_down: bool,
    pub column_colors: [Color; 4],
}

impl Widget for MapRowsView<'_> {
    fn render(self, area: Rect, buf: &mut Buffer) {
        for rel_y in 0..area.height {
            let row_i = self.shift as usize + rel_y as usize;
            let y = if self.going_down {
                area.y + area.height - rel_y + area.y - 1
            } else {
                rel_y + area.y
            };

            if row_i < self.rows.len() {
                for x in 0..4 {
                    buf.get_mut(area.x + x, y)
                        .set_bg(if self.rows[row_i][x as usize] {
                            self.column_colors[x as usize]
                        } else {
                            Color::Reset
                        });
                }
            }
        }
    }
}
