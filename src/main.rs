mod flat_intersperse;
mod map;
mod states;
mod widgets;

use crate::{
    map::Map,
    states::{gaming::Gaming, state::GameState},
};
use argh::FromArgs;
use backtrace::Backtrace;
use crossterm::{
    cursor::{Hide, Show},
    event,
    execute,
    terminal::{
        disable_raw_mode,
        enable_raw_mode,
        EnterAlternateScreen,
        LeaveAlternateScreen,
        SetTitle,
    },
};
use std::{io::stdout, panic, panic::PanicInfo, time::Duration};
use tokio::time::interval;
use tui::{backend::CrosstermBackend, Terminal};

const MAP: [[bool; 4]; 39] = [
    [false, true, true, false],
    [false; 4],
    [true, false, false, true],
    [false; 4],
    [false, true, true, false],
    [false; 4],
    [true, false, false, true],
    [false; 4],
    [false, true, true, false],
    [false; 4],
    [true, false, false, true],
    [false; 4],
    [false; 4],
    [false; 4],
    [false; 4],
    [false; 4],
    [false; 4],
    [false, true, true, false],
    [false; 4],
    [true, false, false, true],
    [false; 4],
    [false, true, true, false],
    [false; 4],
    [true, false, false, true],
    [false; 4],
    [false, true, true, false],
    [false; 4],
    [true, false, false, true],
    [false; 4],
    [false; 4],
    [false; 4],
    [false; 4],
    [false; 4],
    [false; 4],
    [true, false, false, false],
    [true, false, false, false],
    [false, true, false, false],
    [false, false, true, false],
    [false, false, false, true],
];

#[derive(FromArgs)]
/// A simple reimplementation of Quaver/osu!mania that can run in a terminal.
struct Config {
    /// the framerate that the game will rerender (and process input) at.
    #[argh(option, short = 'f')]
    fps: u64,

    /// the gap between beats in terminal cells. similar to scroll speed in other mania games.
    #[argh(option, short = 'g')]
    gap: usize,

    /// whether the rows should fall down instead of elevating up
    #[argh(switch, short = 'd')]
    going_down: bool,
}

fn on_panic(info: &PanicInfo) {
    execute!(stdout(), LeaveAlternateScreen, Show).unwrap();
    disable_raw_mode().unwrap();

    println!("{}", info);

    for frame in Backtrace::new().frames() {
        if frame.symbols().iter().any(|it| {
            it.name()
                .map_or(false, |it| format!("{}", it).contains("termania"))
        }) {
            println!(
                "at {:?}",
                frame
                    .symbols()
                    .iter()
                    .map(|it| format!("{}", it.name().unwrap()))
                    .reduce(|a, b| format!("{} \ninlined into {}", a, b))
                    .unwrap_or_else(|| "???".into())
            )
        }
    }
}

#[tokio::main]
async fn main() {
    let config: Config = argh::from_env();

    let backend = CrosstermBackend::new(stdout());
    let mut terminal = Terminal::new(backend).unwrap();

    enable_raw_mode().unwrap();
    execute!(
        terminal.backend_mut(),
        EnterAlternateScreen,
        Hide,
        SetTitle("termania")
    )
    .unwrap();

    panic::set_hook(Box::new(on_panic));

    let mut state: Box<dyn GameState> = Box::new(Gaming::new(
        Map {
            rows: MAP.to_vec(),
            bpm: 120.0,
            base_gap: 2,
            length: Duration::from_millis(20000),
        },
        config.gap,
        config.going_down,
    ));

    let mut frame_interval = interval(Duration::from_millis(1000 / config.fps));
    loop {
        frame_interval.tick().await;

        while event::poll(Duration::from_secs(0)).unwrap() {
            state.on_input_event(event::read().unwrap());
        }

        terminal.draw(|f| state.draw(f)).unwrap();

        if state.should_quit() {
            break;
        }

        if let Some(next_state) = state.next_state() {
            state = next_state;
        }
    }

    execute!(terminal.backend_mut(), LeaveAlternateScreen, Show).unwrap();
    disable_raw_mode().unwrap();
}
