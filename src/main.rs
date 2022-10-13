use clap::Parser;
use crossbeam_channel::{select, tick, unbounded};
use game::Direction;
use std::thread;
use std::time::Duration;
use ui::Input;

mod game;
mod path;
mod ui;

/// Game of snake
#[derive(Parser)]
struct Cli {
    /// Snake advance interval in ms
    #[arg(short, long, default_value_t = 150)]
    interval: u64,

    /// Width and height of the grid
    #[arg(short, long, default_value_t = 20)]
    grid_size: usize,

    /// Autopilot mode
    #[arg(short, long, default_value_t = false)]
    autopilot: bool,
}

fn main() {
    env_logger::init();

    let args = Cli::parse();

    let mut end = false;
    let mut paused = false;
    let mut steps = 0;
    let mut grid = game::create_grid(args.grid_size);
    let mut snake = game::spawn_snake(&mut grid);
    let obstacle_count = grid.len() * grid.len() / 12;
    game::spawn_obstacles(&mut grid, obstacle_count);
    game::spawn_food(&mut grid);

    let ticks = tick(Duration::from_millis(args.interval));

    ui::init().unwrap();
    ui::draw(&grid, steps, snake.len()).unwrap();

    // Spawn thread to handle ui input.
    let (s, ui_input) = unbounded();
    thread::spawn(move || loop {
        s.send(ui::read_input()).unwrap();
    });

    let mut direction = game::random_direction();
    let mut path: Vec<Direction> = Vec::new();

    loop {
        select! {
            recv(ticks) -> _ => {
                if !end && !paused{
                    // In autopilot mode calculate the path to the food as a list of directions.
                    if args.autopilot {
                        if path.is_empty() {
                            path = path::solve(&grid, *snake.front().unwrap());
                        }
                        // Pop the next direction from the path.
                        // If it is empty (no path found), continue in the current
                        // direction and try again after the next step.
                        direction = path.pop().unwrap_or(direction);
                    }

                    // Advance the snake one step.
                    if game::step(&mut grid, &mut snake, direction).is_err() {
                        end = true
                    }
                    steps +=1;
                    ui::draw(&grid, steps, snake.len()).unwrap();
                }
            }
            recv(ui_input) -> msg => {
                match msg.unwrap() {
                    Input::Exit => break,
                    Input::North => direction = Direction::North,
                    Input::South => direction = Direction::South,
                    Input::East => direction = Direction::East,
                    Input::West => direction = Direction::West,
                    Input::Pause => paused ^= true,
                    Input::Unknown => {},
                }
            }
        }
    }

    ui::reset().unwrap();
}
