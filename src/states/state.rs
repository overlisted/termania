use crossterm::event::Event;
use std::io::Stdout;
use tui::{backend::CrosstermBackend, Frame};

pub trait GameState {
    // TODO add a separate method that runs before `terminal.draw` if we get flickering
    fn draw(&mut self, frame: &mut Frame<CrosstermBackend<Stdout>>);
    fn next_state(&mut self) -> Option<Box<dyn GameState>>;
    fn on_input_event(&mut self, event: Event);
    fn should_quit(&self) -> bool {
        false
    }
}
