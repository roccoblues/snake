use clap::{Parser, ValueEnum};
use game::Direction;
use std::sync::atomic;
use std::sync::mpsc::channel;
use std::sync::Arc;
use std::thread;
use std::time::Duration;
use ui::Input;

mod game;
mod path;
mod ui;

#[derive(Copy, Clone, PartialEq, Eq, PartialOrd, Ord, ValueEnum)]
enum Mode {
    /// You control the snake
    Normal,
    /// You control the snake and it gets faster with every food eaten
    Arcade,
    /// The computer controls the snake
    Autopilot,
}

/// Game of snake
#[derive(Parser)]
struct Cli {
    /// Snake advance interval in ms
    #[arg(short, long, default_value_t = 150)]
    interval: u64,

    /// Width and height of the grid
    #[arg(short, long, default_value_t = 20)]
    grid_size: usize,

    /// Mode
    #[arg(value_enum, default_value_t = Mode::Normal)]
    mode: Mode,
}

fn main() {
    env_logger::init();

    let args = Cli::parse();

    let mut end = false;
    let mut paused = false;
    let mut steps = 0;
    let obstacle_count = args.grid_size * args.grid_size / 20;

    let mut grid = game::create_grid(args.grid_size);
    let mut snake = game::spawn_snake(&mut grid);
    // game::spawn_obstacles(&mut grid, obstacle_count);
    game::spawn_food(&mut grid);

    ui::init().unwrap();
    ui::draw(&grid, steps, snake.len()).unwrap();

    let (tx, rx) = channel();

    // Spawn thread to handle ui input.
    let ui_tx = tx.clone();
    thread::spawn(move || loop {
        ui_tx.send(ui::read_input()).unwrap();
    });

    // Spawn thread to send ticks.
    let interval = Arc::new(atomic::AtomicU64::new(args.interval));
    let int_clone = Arc::clone(&interval);
    let tick_tx = tx.clone();
    thread::spawn(move || loop {
        thread::sleep(Duration::from_millis(
            int_clone.load(atomic::Ordering::Relaxed),
        ));
        tick_tx.send(Input::Step).unwrap();
    });

    let mut direction = game::random_direction();
    let mut path: Vec<Direction> = Vec::new();

    loop {
        match rx.recv().unwrap() {
            Input::Unknown => {}
            Input::Exit => break,
            Input::ChangeDirection(d) => direction = d,
            Input::Pause => paused ^= true,
            Input::Step => {
                if !end && !paused {
                    // In autopilot mode calculate the path to the food as a list of directions.
                    if args.mode == Mode::Autopilot {
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
                    steps += 1;
                    ui::draw(&grid, steps, snake.len()).unwrap();

                    if args.mode == Mode::Arcade {
                        let i = interval.load(atomic::Ordering::Relaxed);
                        interval.store(i - 40, atomic::Ordering::Relaxed);
                    }
                }
            }
        }
    }

    ui::reset().unwrap();
}
