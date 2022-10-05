use crate::game::Cell;
use crossterm::{
    cursor, execute, queue, style,
    style::{Attribute, Print, StyledContent, Stylize},
    terminal::{
        disable_raw_mode, enable_raw_mode, size, Clear, ClearType, EnterAlternateScreen,
        LeaveAlternateScreen,
    },
};
use std::io::{stdout, Write};

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

pub fn draw(map: &Vec<Vec<Cell>>, steps: u32, snake_length: u32) -> crossterm::Result<()> {
    // We use two characters to represent a cell. So we need to make sure to double
    // the x value when we actually draw the cells.

    // adjust x+y to center map on screen
    let (rows, cols) = size()?;
    let map_size = map.len() as u16;
    let x_adjust = (rows - map_size * 2) / 2;
    let y_adjust = (cols - map_size) / 2;

    // drawp map
    for (x, v) in map.iter().enumerate() {
        for (y, cell) in v.iter().enumerate() {
            queue!(
                stdout(),
                cursor::MoveTo(x as u16 * 2 + x_adjust, y as u16 + y_adjust),
                style::PrintStyledContent(cell_to_symbol(x as u16, y as u16, map_size, cell))
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
            x_adjust + map_size * 2 - len_str.chars().count() as u16 - 1,
            y_adjust - 1
        ),
        Print(len_str)
    )?;

    stdout().flush()
}

fn cell_to_symbol(x: u16, y: u16, size: u16, cell: &Cell) -> StyledContent<&str> {
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
