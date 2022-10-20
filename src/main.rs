use clap::{ArgAction, Parser};
use game::{create_grid, spawn_food, spawn_obstacles, spawn_snake, Direction, Tile};
use input::Input;
use output::Screen;
use std::sync::atomic::{self, AtomicU16};
use std::sync::mpsc::channel;
use std::sync::Arc;
use std::thread;
use std::time::Duration;

mod game;
mod input;
mod output;
mod path;

/// Game of snake
#[derive(Parser)]
#[command(disable_help_flag = true)]
struct Cli {
    /// Snake advance interval in ms
    #[arg(short, long, default_value_t = 200, value_parser = clap::value_parser!(u16).range(40..300))]
    interval: u16,

    /// Width of the grid
    #[arg(short = 'w', long, default_value_t = 20, value_parser = clap::value_parser!(u16).range(12..120))]
    grid_width: u16,

    /// Height of the grid
    #[arg(short = 'h', long, default_value_t = 15, value_parser = clap::value_parser!(u16).range(9..90))]
    grid_height: u16,

    /// Don't draw obstacles on the grid
    #[arg(short = 'n', long, default_value_t = false)]
    no_obstacles: bool,

    /// The computer controls the snake
    #[arg(long, default_value_t = false)]
    autopilot: bool,

    /// The snake gets faster with every food eaten
    #[arg(long, default_value_t = false)]
    arcade: bool,

    /// Print help information
    #[arg(long = "help", action = ArgAction::Help, value_parser = clap::value_parser!(bool))]
    help: (),
}

fn main() {
    let args = Cli::parse();

    env_logger::init();
    output::init();

    let screen = Screen::new(args.grid_width, args.grid_height);

    let mut end = false;
    let mut paused = false;
    let mut steps = 0;
    let obstacle_count = args.grid_width * args.grid_height / 25;

    let mut grid = create_grid(args.grid_width.into(), args.grid_height.into());
    let mut snake = spawn_snake(&mut grid);
    let mut food = spawn_food(&mut grid);
    if !args.no_obstacles {
        spawn_obstacles(&mut grid, obstacle_count);
    }

    screen.draw_grid(&grid);
    screen.draw_steps(steps);
    screen.draw_length(snake.len());

    let (tx, rx) = channel();

    // Spawn thread to handle ui input.
    let ui_tx = tx.clone();
    thread::spawn(move || loop {
        ui_tx.send(input::read()).unwrap();
    });

    // Spawn thread to send ticks.
    let interval = Arc::new(atomic::AtomicU16::new(args.interval));
    let int_clone = Arc::clone(&interval);
    thread::spawn(move || loop {
        thread::sleep(Duration::from_millis(
            int_clone.load(atomic::Ordering::Relaxed).into(),
        ));
        tx.send(Input::Step).unwrap();
    });

    let mut direction = game::random_direction();
    let mut path: Vec<Direction> = Vec::new();

    loop {
        match rx.recv().unwrap() {
            Input::Unknown => {}
            Input::Exit => break,
            Input::ChangeDirection(d) => {
                // The snake can't reverse direction. So if the new direction is the opposite
                // of the current one we discard it.
                if d != direction.opposite() {
                    direction = d;
                }
            }
            Input::Pause => paused ^= true,
            Input::DecreaseSpeed => {
                if !args.arcade {
                    increase_interval(&interval);
                }
            }
            Input::IncreaseSpeed => {
                if !args.arcade {
                    decrease_interval(&interval);
                }
            }
            Input::Step => {
                if end || paused {
                    continue;
                }

                let head = snake.front().unwrap();

                // In autopilot mode calculate the path to the food as a list of directions.
                if args.autopilot {
                    if path.is_empty() {
                        path = path::solve(&grid, *head, food);
                    }
                    // Pop the next direction from the path.
                    // If it is empty (no path found), continue in the current
                    // direction and try again after the next step.
                    direction = path.pop().unwrap_or(direction);
                }

                // Return point in front of the snake in the given direction.
                let p = game::next_point(*head, direction);
                let (x, y) = p;

                // Check tile in the grid.
                match grid[x][y] {
                    // The snake crashed - end the game.
                    Tile::Obstacle | Tile::Snake => {
                        grid[x][y] = Tile::Crash;
                        screen.draw_tile(p, Tile::Crash);
                        end = true;
                    }
                    // The snake ate - spawn new food.
                    Tile::Food => {
                        snake.push_front(p);
                        grid[x][y] = Tile::Snake;
                        screen.draw_tile(p, Tile::Snake);
                        food = spawn_food(&mut grid);
                        screen.draw_tile(food, Tile::Food);
                        screen.draw_length(snake.len());
                        // In arcade mode we decrease the tick interval with every food eaten
                        // to make the game faster.
                        if args.arcade {
                            decrease_interval(&interval);
                        }
                    }
                    // If the tile is free we pop the tail of the snake to make it look like it is moving.
                    Tile::Free => {
                        snake.push_front(p);
                        grid[x][y] = Tile::Snake;
                        screen.draw_tile(p, Tile::Snake);
                        let tail = snake.pop_back().unwrap();
                        let (tail_x, tail_y) = tail;
                        grid[tail_x][tail_y] = Tile::Free;
                        screen.draw_tile(tail, Tile::Free);
                    }
                    Tile::Crash => unreachable!(),
                }

                steps += 1;
                screen.draw_steps(steps);
            }
        }
    }

    output::reset()
}

fn increase_interval(interval: &Arc<AtomicU16>) {
    let i = interval.load(atomic::Ordering::Relaxed);
    interval.store(i + 5, atomic::Ordering::Relaxed);
}

fn decrease_interval(interval: &Arc<AtomicU16>) {
    let i = interval.load(atomic::Ordering::Relaxed);
    if i > 45 {
        interval.store(i - 5, atomic::Ordering::Relaxed);
    }
}
