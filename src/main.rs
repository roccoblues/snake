use clap::Parser;
use crossbeam_channel::{select, tick, unbounded};
use game::{Direction, Game};
use std::thread;
use std::time::Duration;
use ui::Input;

mod game;
mod path;
mod ui;

// snake advance and ui redraw interval
const SPEED: Duration = Duration::from_millis(150);

/// Game of snake.
#[derive(Parser)]
struct Cli {
    /// Width and height of the grid
    #[arg(short, long, default_value_t = 25)]
    grid_size: usize,

    /// Autopilot mode
    #[arg(short, long, default_value_t = false)]
    autopilot: bool,
}

fn main() {
    let args = Cli::parse();
    let mut game = Game::new(args.grid_size);

    let mut paused = false;
    let mut direction = game::random_direction();

    let ticks = tick(SPEED);

    ui::init().unwrap();
    ui::draw(&game.grid, game.steps, game.snake.len()).unwrap();

    // spawn thread to handle ui input
    let (s, ui_input) = unbounded();
    thread::spawn(move || loop {
        s.send(ui::read_input()).unwrap();
    });

    // game loop
    loop {
        select! {
            recv(ticks) -> _ => {
                if !game.end && !paused{
                    game.step(direction);
                     ui::draw(&game.grid, game.steps, game.snake.len()).unwrap();
                }
            }
            recv(ui_input) -> msg => {
                match msg.unwrap() {
                    Input::Exit => break,
                    Input::North => direction= Direction::North,
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
