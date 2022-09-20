use crossterm::event::{poll, read, Event, KeyCode};
use game::{random_direction, Direction, Game};
use std::time::Duration;

mod game;
mod ui;

const SIZE: u16 = 20;

fn main() {
    ui::init().unwrap();

    let mut game = Game::new(SIZE);
    ui::draw(&game.map.tiles()).unwrap();

    let mut direction = random_direction().unwrap();
    let mut crash = false;

    loop {
        if !crash {
            crash = !game.advance_snake(direction).is_ok();
            ui::draw(&game.map.tiles()).unwrap();
        }

        if poll(Duration::from_millis(150)).unwrap() {
            let event = read().unwrap();
            if event == Event::Key(KeyCode::Esc.into()) {
                break;
            }
            if event == Event::Key(KeyCode::Up.into()) {
                direction = Direction::Up;
            }
            if event == Event::Key(KeyCode::Down.into()) {
                direction = Direction::Down;
            }
            if event == Event::Key(KeyCode::Left.into()) {
                direction = Direction::Left;
            }
            if event == Event::Key(KeyCode::Right.into()) {
                direction = Direction::Right;
            }
        }
    }

    ui::reset().unwrap();
}
