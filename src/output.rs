use crate::types::{Grid, Point, Tile};
use crossterm::style::{Attribute, Print, StyledContent, Stylize};
use crossterm::terminal::{
    disable_raw_mode, enable_raw_mode, size, Clear, ClearType, EnterAlternateScreen,
    LeaveAlternateScreen,
};
use crossterm::{cursor, execute, style};
use std::io::stdout;

pub const MIN_GRID_WIDTH: u16 = 12;
pub const MIN_GRID_HEIGHT: u16 = 11;

pub struct Screen {
    grid_width: u16,
    grid_height: u16,
    x_adjust: u16,
    y_adjust: u16,
}

impl Screen {
    pub fn new(grid_width: u16, grid_height: u16) -> Self {
        // We use two characters to represent a tile. So we need to make sure to double
        // the x value when we actually draw the grid.

        // Calculate x and y adjustment needed to center the grid on screen.
        let (cols, rows) = size().unwrap();
        let x_adjust = (cols - grid_width * 2) / 2;
        let y_adjust = (rows - grid_height + 1) / 2;

        Screen {
            grid_width,
            grid_height,
            x_adjust,
            y_adjust,
        }
    }

    pub fn draw_grid(&self, grid: &Grid) {
        for x in 0..self.grid_width {
            for y in 0..self.grid_height {
                let x = x as usize;
                let y = y as usize;
                self.draw_tile((x, y), grid[x][y])
            }
        }
    }

    pub fn draw_steps(&self, steps: u32) {
        execute!(
            stdout(),
            cursor::MoveTo(self.x_adjust, self.y_adjust - 1),
            Print(format!("Steps: {}", steps)),
        )
        .unwrap();
    }

    pub fn draw_length(&self, length: usize) {
        let len_str = format!("Snake length: {}", length);
        execute!(
            stdout(),
            cursor::MoveTo(
                self.x_adjust + self.grid_width * 2 - len_str.chars().count() as u16,
                self.y_adjust - 1
            ),
            Print(len_str),
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

pub fn max_grid_width() -> u16 {
    let (cols, _) = size().unwrap();
    cols / 2
}

pub fn max_grid_height() -> u16 {
    let (_, rows) = size().unwrap();
    rows - 1
}

pub fn init() {
    enable_raw_mode().unwrap();
    execute!(
        stdout(),
        EnterAlternateScreen,
        Clear(ClearType::All),
        cursor::Hide,
    )
    .unwrap();
}

pub fn reset() {
    execute!(stdout(), cursor::Show, LeaveAlternateScreen).unwrap();
    disable_raw_mode().unwrap();
}
