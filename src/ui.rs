use crate::game::{Map, Tile};
use crossterm::{
    cursor, execute, queue, style,
    style::Attribute,
    style::StyledContent,
    style::Stylize,
    terminal::{
        disable_raw_mode, enable_raw_mode, Clear, ClearType, EnterAlternateScreen,
        LeaveAlternateScreen,
    },
};
use log::debug;
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

pub fn draw(map: &Map) -> crossterm::Result<()> {
    debug!("{:?}", map);
    queue!(stdout(), cursor::MoveTo(0, 0))?;
    for (x, v) in map.tiles().iter().enumerate() {
        for (y, tile) in v.iter().enumerate() {
            queue!(
                stdout(),
                cursor::MoveTo(x as u16 * 2, y as u16),
                style::PrintStyledContent(tile_to_symbol(tile))
            )?
        }
    }
    stdout().flush()
}

pub fn tile_to_symbol(tile: &Tile) -> StyledContent<&str> {
    match tile {
        Tile::Free => "  ".attribute(Attribute::Reset),
        Tile::Snake => "██".green(),
        Tile::Food => "░░".yellow(),
        Tile::Obstacle => "▓▓".white(),
        Tile::Crash => "××".red(),
    }
}
