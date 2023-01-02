use crossterm::style::{Attribute, Print, StyledContent, Stylize};
use crossterm::terminal::{self, Clear, ClearType, EnterAlternateScreen, LeaveAlternateScreen};
use crossterm::{cursor, execute, style};
use std::io::stdout;

use crate::types::{Point, Tile};

pub const MIN_GRID_WIDTH: u16 = 12;
pub const MIN_GRID_HEIGHT: u16 = 11;

pub struct Screen {
    width: u16,
    x_adjust: u16,
    y_adjust: u16,
}

impl Screen {
    pub fn new(width: u16, height: u16) -> Self {
        // We use two characters to represent a tile. So we need to make sure to double
        // the x value when we actually draw the grid.

        // Make sure we start with a blank screen.
        execute!(stdout(), Clear(ClearType::All),).unwrap();

        // Calculate x and y adjustment needed to center the grid on screen.
        let (cols, rows) = terminal::size().unwrap();
        let x_adjust = (cols - width * 2) / 2;
        let y_adjust = (rows - height + 1) / 2;

        Screen {
            width,
            x_adjust,
            y_adjust,
        }
    }

    pub fn draw_text_left(&self, str: String) {
        execute!(
            stdout(),
            cursor::MoveTo(self.x_adjust, self.y_adjust - 1),
            Print(str),
        )
        .unwrap();
    }

    pub fn draw_text_right(&self, str: String) {
        execute!(
            stdout(),
            cursor::MoveTo(
                self.x_adjust + self.width * 2 - str.chars().count() as u16,
                self.y_adjust - 1
            ),
            Print(str),
        )
        .unwrap()
    }

    pub fn draw_tile(&self, p: Point, tile: Tile) {
        // We use two characters to represent a tile. So we need to make sure to double
        // the x value when we actually draw the grid.
        let (x, y) = p;
        execute!(
            stdout(),
            cursor::MoveTo(x as u16 * 2 + self.x_adjust, y as u16 + self.y_adjust),
            style::PrintStyledContent(tile_to_symbol(tile)),
        )
        .unwrap()
    }
}

// Returns the actual characters to be drawn for the given tile.
fn tile_to_symbol(tile: Tile) -> StyledContent<&'static str> {
    match tile {
        Tile::Free => "  ".attribute(Attribute::Reset),
        Tile::Snake => "██".green(),
        Tile::Food => "██".yellow(),
        Tile::Obstacle => "▓▓".white(),
        Tile::Crash => "XX".red().on_white(),
    }
}

pub fn max_grid_size() -> (u16, u16) {
    let (cols, rows) = terminal::size().unwrap();
    (cols / 2, rows - 1)
}

pub fn init() {
    terminal::enable_raw_mode().unwrap();
    execute!(stdout(), EnterAlternateScreen, cursor::Hide,).unwrap();
}

pub fn reset() {
    execute!(stdout(), cursor::Show, LeaveAlternateScreen).unwrap();
    terminal::disable_raw_mode().unwrap();
}
