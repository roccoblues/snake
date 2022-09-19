use crossterm::{
    cursor,
    event::{poll, read, Event, KeyCode},
    execute, queue, style,
    style::Stylize,
    terminal::{
        disable_raw_mode, enable_raw_mode, Clear, ClearType, EnterAlternateScreen,
        LeaveAlternateScreen,
    },
};
use int_enum::IntEnum;
use rand::prelude::*;
use std::io::{stdout, Write};
use std::time::Duration;

#[repr(u8)]
#[derive(Copy, Clone, Debug)]
enum Tile {
    Free,
    Snake,
    Food,
    Obstacle,
}

#[repr(u8)]
#[derive(Clone, Copy, Debug, Eq, PartialEq, IntEnum)]
enum Direction {
    Up = 0,
    Down = 1,
    Left = 2,
    Right = 3,
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
    let mut snake_x = rng.gen_range(1..=COLS - 2) as u16;
    let mut snake_y = rng.gen_range(1..=ROWS - 1) as u16;
    let mut direction = Direction::from_int(rng.gen_range(0..=3) as u8).unwrap();
    let mut prev_direction = direction;
    let mut symbol: &str;

    // map[snake_start_x][snake_start_y] = Tile::Snake;

    loop {
        prev_direction = direction;

        if poll(Duration::from_millis(200)).unwrap() {
            let event = read().unwrap();
            if event == Event::Key(KeyCode::Esc.into()) {
                break;
            }
            if event == Event::Key(KeyCode::Up.into()) {
                direction = Direction::Up;
            }
            if event == Event::Key(KeyCode::Down.into()) {
                direction = Direction::Down;
            }
            if event == Event::Key(KeyCode::Left.into()) {
                direction = Direction::Left;
            }
            if event == Event::Key(KeyCode::Right.into()) {
                direction = Direction::Right;
            }
        }

        match direction {
            Direction::Up => {
                symbol = "█";
                if prev_direction == Direction::Up {
                    snake_y -= 1;
                } else if prev_direction == Direction::Right {
                    snake_x += 1;
                } else if prev_direction == Direction::Left {
                    snake_x -= 1;
                }
            }
            Direction::Down => {
                symbol = "█";
                snake_y += 1;
                if prev_direction == Direction::Right {
                    snake_x += 1;
                } else if prev_direction == Direction::Left {
                    snake_x -= 1;
                }
            }
            Direction::Right => {
                symbol = "▄▄";
                snake_x += 1;
                if prev_direction == Direction::Up {
                    snake_y -= 1;
                }
            }
            Direction::Left => {
                symbol = "▄▄";
                if prev_direction == Direction::Left {
                    snake_x -= 1;
                }
            }
        }

        execute!(
            stdout(),
            cursor::MoveTo(snake_x, snake_y),
            style::PrintStyledContent(symbol.green())
        )
        .unwrap();
    }

    execute!(stdout(), cursor::Show, LeaveAlternateScreen).unwrap();
    disable_raw_mode().unwrap()
}
