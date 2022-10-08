use crate::game::{Cell, Grid};
use crossterm::event::{read, Event, KeyCode};
use crossterm::style::{Attribute, Print, StyledContent, Stylize};
use crossterm::terminal::{
    disable_raw_mode, enable_raw_mode, size, Clear, ClearType, EnterAlternateScreen,
    LeaveAlternateScreen,
};
use crossterm::{cursor, execute, queue, style};
use std::io::{stdout, Write};
use std::time::Duration;

const INPUT_TIMEOUT: Duration = Duration::from_millis(100);

#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub enum Input {
    North,
    South,
    East,
    West,
    Pause,
    Exit,
    Unknown,
}

pub fn init() -> crossterm::Result<()> {
    enable_raw_mode()?;
    execute!(
        stdout(),
        EnterAlternateScreen,
        Clear(ClearType::All),
        cursor::Hide,
    )?;
    Ok(())
}

pub fn reset() -> crossterm::Result<()> {
    execute!(stdout(), cursor::Show, LeaveAlternateScreen)?;
    disable_raw_mode()?;
    Ok(())
}

pub fn draw(grid: &Grid, steps: u32, snake_length: u32) -> crossterm::Result<()> {
    // We use two characters to represent a cell. So we need to make sure to double
    // the x value when we actually draw the grid.

    // adjust x+y to center grid on screen
    let (rows, cols) = size()?;
    let size = grid.len() as u16;
    let x_adjust = (rows - size * 2) / 2;
    let y_adjust = (cols - size) / 2;

    // drawp grid
    for (x, v) in grid.iter().enumerate() {
        for (y, cell) in v.iter().enumerate() {
            queue!(
                stdout(),
                cursor::MoveTo(x as u16 * 2 + x_adjust, y as u16 + y_adjust),
                style::PrintStyledContent(cell_to_symbol(x, y, size as usize, cell))
            )?
        }
    }

    // draw steps and snake length
    let len_str = format!("Snake length: {}", snake_length);
    queue!(
        stdout(),
        cursor::MoveTo(x_adjust, y_adjust - 1),
        Print(format!("Steps: {}", steps)),
        cursor::MoveTo(
            x_adjust + size * 2 - len_str.chars().count() as u16 - 1,
            y_adjust - 1
        ),
        Print(len_str)
    )?;

    stdout().flush()
}

fn cell_to_symbol(x: usize, y: usize, size: usize, cell: &Cell) -> StyledContent<&str> {
    match cell {
        Cell::Free => "  ".attribute(Attribute::Reset),
        Cell::Snake => "██".green(),
        Cell::Food => "██".yellow(),
        Cell::Obstacle => {
            if x == 0 {
                // first column
                if y == 0 {
                    "╔══".magenta()
                } else if y == size - 1 {
                    "╚══".magenta()
                } else {
                    "║".magenta()
                }
            } else if x == size - 1 {
                // last column
                if y == 0 {
                    "╗".magenta()
                } else if y == size - 1 {
                    "╝".magenta()
                } else {
                    "║".magenta()
                }
            } else if (y == 0 || y == size - 1) && (x > 0 || x < size - 1) {
                // fill first+last row
                "══".magenta()
            } else {
                "▓▓".white()
            }
        }
        Cell::Crash => "××".red().on_white(),
    }
}

pub fn read_input() -> Input {
    let event = read().unwrap();
    match event {
        Event::Key(key_event) => match key_event.code {
            KeyCode::Esc | KeyCode::Char('q') => Input::Exit,
            KeyCode::Up => Input::North,
            KeyCode::Down => Input::South,
            KeyCode::Right => Input::East,
            KeyCode::Left => Input::West,
            KeyCode::Char(' ') => Input::Pause,
            _ => Input::Unknown,
        },
        _ => Input::Unknown,
    }
}
