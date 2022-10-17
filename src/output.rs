use crate::game::{Grid, Tile};
use crossterm::style::{Attribute, Print, StyledContent, Stylize};
use crossterm::terminal::{
    disable_raw_mode, enable_raw_mode, size, Clear, ClearType, EnterAlternateScreen,
    LeaveAlternateScreen,
};
use crossterm::{cursor, execute, style};
use std::io::stdout;

pub struct Screen {
    grid_width: u16,
    grid_height: u16,
    x_adjust: u16,
    y_adjust: u16,
}

impl Screen {
    pub fn new(grid_width: u16, grid_height: u16) -> Self {
        enable_raw_mode().unwrap();
        execute!(
            stdout(),
            EnterAlternateScreen,
            Clear(ClearType::All),
            cursor::Hide,
        )
        .unwrap();

        // Check if the terminal size fits the grid plus score information.
        let (cols, rows) = size().unwrap();
        assert!(cols > grid_width * 2, "Terminal width isn't enough!");
        assert!(rows > grid_height + 1, "Terminal height isn't enough!");

        // Calculate x and y adjustment needed to center the grid on screen.
        let x_adjust = (cols - grid_width * 2) / 2;
        let y_adjust = (rows - grid_height) / 2;

        Screen {
            grid_width,
            grid_height,
            x_adjust,
            y_adjust,
        }
    }

    pub fn draw_grid(&self, grid: &Grid) {
        for (x, v) in grid.iter().enumerate() {
            for (y, tile) in v.iter().enumerate() {
                self.draw(x as u16, y as u16, *tile)
            }
        }
    }

    pub fn draw_score(&self, steps: u32, snake_length: usize) {
        let len_str = format!("Snake length: {}", snake_length);
        execute!(
            stdout(),
            cursor::MoveTo(self.x_adjust, self.y_adjust - 1),
            Print(format!("Steps: {}", steps)),
            cursor::MoveTo(
                self.x_adjust + self.grid_width * 2 - len_str.chars().count() as u16 - 1,
                self.y_adjust - 1
            ),
            Print(len_str)
        )
        .unwrap();
    }

    pub fn draw(&self, x: u16, y: u16, tile: Tile) {
        // We use two characters to represent a tile. So we need to make sure to double
        // the x value when we actually draw the grid.
        execute!(
            stdout(),
            cursor::MoveTo(x * 2 + self.x_adjust, y + self.y_adjust),
            style::PrintStyledContent(self.tile_to_symbol(x, y, tile))
        )
        .unwrap()
    }

    // Returns the actual characters to be drawn for the given tile.
    fn tile_to_symbol(&self, x: u16, y: u16, tile: Tile) -> StyledContent<&str> {
        match tile {
            Tile::Free => "  ".attribute(Attribute::Reset),
            Tile::Snake => "██".green(),
            Tile::Food => "██".yellow(),
            Tile::Obstacle => {
                if x == 0 {
                    // first column
                    if y == 0 {
                        "╔══".magenta()
                    } else if y == self.grid_height - 1 {
                        "╚══".magenta()
                    } else {
                        "║".magenta()
                    }
                } else if x == self.grid_width - 1 {
                    // last column
                    if y == 0 {
                        "╗".magenta()
                    } else if y == self.grid_height - 1 {
                        "╝".magenta()
                    } else {
                        "║".magenta()
                    }
                } else if (y == 0 || y == self.grid_height - 1)
                    && (x > 0 || x < self.grid_width - 1)
                {
                    // fill first+last row
                    "══".magenta()
                } else {
                    "▓▓".white()
                }
            }
            Tile::Crash => "××".red().on_white(),
        }
    }
}

impl Drop for Screen {
    fn drop(&mut self) {
        execute!(stdout(), cursor::Show, LeaveAlternateScreen).unwrap();
        disable_raw_mode().unwrap();
    }
}
