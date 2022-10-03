use clap::Parser;
use crossterm::event::{poll, read, Event, KeyCode};
use game::{Direction, Game};
use std::time::Duration;

mod game;
mod ui;

/// Game of snake.
#[derive(Parser)]
struct Cli {
    /// Width and height of the map
    #[arg(short, long, default_value_t = 25)]
    map_size: u16,

    /// Autopilot mode
    #[arg(short, long, default_value_t = false)]
    autopilot: bool,
}

fn main() {
    let args = Cli::parse();
    let mut game = Game::new(args.map_size);

    ui::init().unwrap();
    ui::draw(&game.tiles, game.steps, game.snake.len() as u32).unwrap();

    loop {
        if poll(Duration::from_millis(150)).unwrap() {
            let event = read().unwrap();
            if event == Event::Key(KeyCode::Esc.into()) {
                break;
            } else if event == Event::Key(KeyCode::Up.into()) {
                game.set_direction(Direction::Up);
            } else if event == Event::Key(KeyCode::Down.into()) {
                game.set_direction(Direction::Down);
            } else if event == Event::Key(KeyCode::Left.into()) {
                game.set_direction(Direction::Left);
            } else if event == Event::Key(KeyCode::Right.into()) {
                game.set_direction(Direction::Right);
            };
        }

        if !game.end {
            game.step();
            ui::draw(&game.tiles, game.steps, game.snake.len() as u32).unwrap();
        }
    }

    ui::reset().unwrap();
}
