use crossterm::{
    cursor,
    event::{poll, read, Event, KeyCode},
    execute, queue, style,
    style::Stylize,
    terminal::{
        disable_raw_mode, enable_raw_mode, Clear, ClearType, EnterAlternateScreen,
        LeaveAlternateScreen,
    },
    tty::IsTty,
    Command,
};
use rand::prelude::*;
use std::io::{stdin, stdout, Write};
use std::thread;
use std::time::Duration;

#[derive(Copy, Clone, Debug)]
enum Tile {
    Free,
    Snake,
    Food,
    Obstacle,
}

enum Direction {
    Up,
    Down,
    Left,
    Right,
}

const ROWS: u16 = 15;
const COLS: u16 = 30;

fn main() {
    enable_raw_mode().unwrap();

    execute!(
        stdout(),
        EnterAlternateScreen,
        Clear(ClearType::All),
        cursor::Hide,
        cursor::MoveTo(0, 0),
        // style::SetForegroundColor(style::Color::Red)
    )
    .unwrap();

    for row in 0..=ROWS {
        for col in 0..=COLS {
            let sym = match (row, col) {
                (0, 0) => "┌",
                (0, COLS) => "┐",
                (ROWS, 0) => "└",
                (ROWS, COLS) => "┘",
                (0, 0..=u16::MAX) => "─",
                (ROWS, 0..=u16::MAX) => "─",
                (0..=u16::MAX, COLS) => "│",
                (0..=u16::MAX, 0) => "│",
                _ => " ",
            };
            queue!(stdout(), cursor::MoveTo(col, row), style::Print(sym)).unwrap();
        }
    }
    stdout().flush().unwrap();

    // let mut map = [[Tile::Free; ROWS]; COLS];

    let mut rng = thread_rng();
    let snake_start_x = rng.gen_range(1..=COLS - 2) as u16;
    let snake_start_y = rng.gen_range(1..=ROWS - 1) as u16;

    // map[snake_start_x][snake_start_y] = Tile::Snake;

    execute!(
        stdout(),
        cursor::MoveTo(snake_start_x, snake_start_y),
        style::PrintStyledContent("▄▄".green())
    )
    .unwrap();

    loop {
        if poll(Duration::from_millis(100)).unwrap() {
            let event = read().unwrap();
            if event == Event::Key(KeyCode::Esc.into()) {
                break;
            }
        }
    }

    execute!(stdout(), cursor::Show, LeaveAlternateScreen).unwrap();
    disable_raw_mode().unwrap()
}
