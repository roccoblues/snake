use crossterm::event::{poll, read, Event, KeyCode};
use game::{Direction, Error, Game};
use std::time::Duration;

mod game;
mod ui;

const SIZE: u16 = 25;

fn main() {
    ui::init().unwrap();

    let mut game = Game::new(SIZE);
    ui::draw(&game.tiles()).unwrap();

    let mut crash = false;

    loop {
        if !crash {
            match game.step() {
                Err(Error::SnakeCrash) => crash = true,
                _ => {}
            }
            ui::draw(&game.tiles()).unwrap();
        }

        if poll(Duration::from_millis(150)).unwrap() {
            let event = read().unwrap();
            if event == Event::Key(KeyCode::Esc.into()) {
                break;
            } else if event == Event::Key(KeyCode::Up.into()) {
                game.change_direction(Direction::Up).ok();
            } else if event == Event::Key(KeyCode::Down.into()) {
                game.change_direction(Direction::Down).ok();
            } else if event == Event::Key(KeyCode::Left.into()) {
                game.change_direction(Direction::Left).ok();
            } else if event == Event::Key(KeyCode::Right.into()) {
                game.change_direction(Direction::Right).ok();
            };
        }
    }

    ui::reset().unwrap();
}
