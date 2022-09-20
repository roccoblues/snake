use crossterm::event::{poll, read, Event, KeyCode};
use game::{random_direction, Direction, Map};
use std::error::Error;
use std::time::Duration;

mod game;
mod ui;

const SIZE: u16 = 20;

fn main() {
    env_logger::init();

    ui::init().unwrap();

    let mut map = Map::new(SIZE);
    ui::draw(&map).unwrap();

    let mut direction = random_direction().unwrap();

    loop {
        map.advance_snake(direction);
        ui::draw(&map).unwrap();

        if poll(Duration::from_millis(200)).unwrap() {
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
