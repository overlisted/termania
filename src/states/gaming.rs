use crate::{
    flat_intersperse::FlatIntersperse,
    map::Map,
    states::state::GameState,
    widgets::map_rows_view::MapRowsView,
};
use crossterm::event::{Event, KeyCode};
use std::io::Stdout;
use tokio::time::Instant;
use tui::{
    backend::CrosstermBackend,
    layout::Rect,
    style::Color,
    widgets::{Block, Borders},
    Frame,
};

const COLUMN_COLORS: [Color; 4] = [Color::Red, Color::Yellow, Color::Green, Color::Blue];

pub struct Gaming {
    map: Map,
    started_on: Instant,
    gap: usize,
    going_down: bool,
    requested_quit: bool,
}

impl Gaming {
    pub fn new(map: Map, gap: usize, going_down: bool) -> Gaming {
        Gaming {
            map,
            started_on: Instant::now(),
            gap,
            going_down,
            requested_quit: false,
        }
    }
}

fn center_rect(rect: Rect, background: &Rect) -> Rect {
    Rect::new(
        background.width / 2 - rect.width / 2,
        background.height / 2 - rect.height / 2,
        rect.width,
        rect.height,
    )
}

impl GameState for Gaming {
    fn draw(&mut self, frame: &mut Frame<CrosstermBackend<Stdout>>) {
        let base_shift = self.started_on.elapsed().as_secs_f64() / 60.0 * self.map.bpm as f64;
        let decimal_shift = base_shift * self.gap as f64;
        let empty_row = [false; 4];

        frame.render_widget(
            Block::default().borders(Borders::LEFT | Borders::RIGHT),
            center_rect(Rect::new(0, 0, 8, frame.size().height), &frame.size()),
        );

        frame.render_widget(
            MapRowsView {
                rows: FlatIntersperse::new(self.map.rows.iter(), vec![&empty_row; self.gap - 1])
                    .collect(),
                shift: decimal_shift.round() as usize,
                going_down: self.going_down,
                column_colors: COLUMN_COLORS,
            },
            center_rect(Rect::new(0, 0, 4, frame.size().height), &frame.size()),
        );
    }

    fn next_state(&mut self) -> Option<Box<dyn GameState>> {
        None
    }

    fn on_input_event(&mut self, event: Event) {
        if let Event::Key(event) = event {
            match event.code {
                KeyCode::Char('q') => {
                    self.requested_quit = true;
                }
                _ => {}
            }
        }
    }

    fn should_quit(&self) -> bool {
        self.requested_quit || self.started_on.elapsed() > self.map.length
    }
}
