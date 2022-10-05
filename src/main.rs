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
    ui::draw(&game.cells, game.steps, game.snake.len() as u32).unwrap();

    loop {
        if poll(Duration::from_millis(150)).unwrap() {
            let event = read().unwrap();
            match event {
                Event::Key(key_event) => match key_event.code {
                    KeyCode::Esc | KeyCode::Char('q') => break,
                    KeyCode::Up => game.set_direction(Direction::Up),
                    KeyCode::Down => game.set_direction(Direction::Down),
                    KeyCode::Right => game.set_direction(Direction::Right),
                    KeyCode::Left => game.set_direction(Direction::Left),
                    KeyCode::Char(' ') => pause(),
                    _ => {}
                },
                _ => {}
            }
        }

        if !game.end {
            game.step();
            ui::draw(&game.cells, game.steps, game.snake.len() as u32).unwrap();
        }
    }

    ui::reset().unwrap();
}

fn pause() {
    loop {
        if poll(Duration::from_millis(100)).unwrap() {
            let event = read().unwrap();
            if event == Event::Key(KeyCode::Esc.into())
                || event == Event::Key(KeyCode::Char(' ').into())
            {
                break;
            }
        }
    }
}
