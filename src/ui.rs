use crate::game::Tile;
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

pub fn draw(map: &Vec<Vec<Tile>>, score: u32, snake_length: u32) -> crossterm::Result<()> {
    // We use two characters to represent a tile. So we need to make sure to double
    // the x value when we actually draw the tiles.

    // adjust x+y to center map on screen
    let (rows, cols) = size()?;
    let map_size = map.len() as u16;
    let x_adjust = (rows - map_size * 2) / 2;
    let y_adjust = (cols - map_size) / 2;

    // drawp map
    for (x, v) in map.iter().enumerate() {
        for (y, tile) in v.iter().enumerate() {
            queue!(
                stdout(),
                cursor::MoveTo(x as u16 * 2 + x_adjust, y as u16 + y_adjust),
                style::PrintStyledContent(tile_to_symbol(x as u16, y as u16, map_size, tile))
            )?
        }
    }

    // draw score and snake length
    let len_str = format!("Snake length: {}", snake_length);
    queue!(
        stdout(),
        cursor::MoveTo(x_adjust, y_adjust - 1),
        Print(format!("Score: {}", score)),
        cursor::MoveTo(
            x_adjust + map_size * 2 - len_str.chars().count() as u16 - 1,
            y_adjust - 1
        ),
        Print(len_str)
    )?;

    stdout().flush()
}

fn tile_to_symbol(x: u16, y: u16, size: u16, tile: &Tile) -> StyledContent<&str> {
    match tile {
        Tile::Free => "  ".attribute(Attribute::Reset),
        Tile::Snake => "██".green(),
        Tile::Food => "██".yellow(),
        Tile::Obstacle => {
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
        Tile::Crash => "××".red().on_white(),
    }
}
