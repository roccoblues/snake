use crossterm::event::{poll, read, Event, KeyCode};
use game::{Direction, Game};
use std::time::Duration;

mod game;
mod ui;

const SIZE: u16 = 25;

fn main() {
    ui::init().unwrap();

    let mut game = Game::new(SIZE);
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
