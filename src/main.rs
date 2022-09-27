use crossterm::event::{poll, read, Event, KeyCode};
use game::{Direction, Map};
use std::error::Error;
use std::time::Duration;

mod game;
mod ui;

const SIZE: u16 = 25;

fn main() -> Result<(), Box<dyn Error>> {
    ui::init()?;

    let mut map = Map::new(SIZE);

    let mut snake = map.spawn_snake();
    let mut crash = false;

    ui::draw_map(&map.tiles)?;

    loop {
        if !crash {
            match game::step(&mut map, &mut snake) {
                Err(game::Error::SnakeCrash) => crash = true,
                _ => {}
            }
            ui::draw_map(&map.tiles)?
        }

        if poll(Duration::from_millis(150))? {
            let event = read().unwrap();
            if event == Event::Key(KeyCode::Esc.into()) {
                break;
            } else if event == Event::Key(KeyCode::Up.into()) {
                snake.set_direction(Direction::Up);
            } else if event == Event::Key(KeyCode::Down.into()) {
                snake.set_direction(Direction::Down);
            } else if event == Event::Key(KeyCode::Left.into()) {
                snake.set_direction(Direction::Left);
            } else if event == Event::Key(KeyCode::Right.into()) {
                snake.set_direction(Direction::Right);
            };
        }
    }

    ui::reset()?;
    Ok(())
}
