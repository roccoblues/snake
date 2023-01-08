use clap::{ArgAction, Parser};

use crate::output;
use crate::snake::{self, MIN_INTERVAL};

/// Game of snake
#[derive(Parser)]
#[command(disable_help_flag = true)]
pub struct Opts {
    /// Snake advance interval in ms
    #[arg(short, long, default_value_t = 175, value_parser = clap::value_parser!(u16).range(MIN_INTERVAL..300))]
    pub interval: u16,

    /// Width of the grid
    #[arg(short = 'w', long, default_value_t = 20, conflicts_with = "fit_grid", value_parser = grid_width_in_range)]
    pub grid_width: u16,

    /// Height of the grid
    #[arg(short = 'h', long, default_value_t = 15, conflicts_with = "fit_grid", value_parser = grid_height_in_range)]
    pub grid_height: u16,

    /// Fit the grid to the screen
    #[arg(short = 'f', long, default_value_t = false)]
    pub fit_grid: bool,

    /// Don't draw obstacles on the grid
    #[arg(short = 'n', long, default_value_t = false)]
    pub no_obstacles: bool,

    /// The computer controls the snake
    #[arg(long, default_value_t = false)]
    pub autopilot: bool,

    /// The snake gets faster with every food eaten
    #[arg(long, default_value_t = false)]
    pub arcade: bool,

    /// Print help information
    #[arg(long = "help", action = ArgAction::Help, value_parser = clap::value_parser!(bool))]
    pub help: (),
}

impl From<Opts> for snake::Config {
    fn from(opts: Opts) -> Self {
        snake::Config {
            autopilot: opts.autopilot,
            arcade: opts.arcade,
            grid_width: opts.grid_width,
            grid_height: opts.grid_height,
            fit_grid: opts.fit_grid,
            no_obstacles: opts.no_obstacles,
            interval: opts.interval,
        }
    }
}

fn grid_width_in_range(s: &str) -> Result<u16, String> {
    let width: u16 = s.parse().map_err(|_| format!("`{}` isn't a number", s))?;
    let (max, _) = output::max_grid_size();
    if (output::MIN_GRID_WIDTH..=max).contains(&width) {
        Ok(width)
    } else {
        Err(format!(
            "Grid width not in range {}-{}",
            output::MIN_GRID_WIDTH,
            max
        ))
    }
}

fn grid_height_in_range(s: &str) -> Result<u16, String> {
    let height: u16 = s.parse().map_err(|_| format!("`{}` isn't a number", s))?;
    let (_, max) = output::max_grid_size();
    if (output::MIN_GRID_HEIGHT..=max).contains(&height) {
        Ok(height)
    } else {
        Err(format!(
            "Grid height not in range {}-{}",
            output::MIN_GRID_HEIGHT,
            max
        ))
    }
}
