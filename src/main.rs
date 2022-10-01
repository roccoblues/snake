use crossterm::event::{poll, read, Event, KeyCode};
use game::{Direction, Game};
use std::error::Error;
use std::time::Duration;

mod game;
mod ui;

const SIZE: u16 = 25;

fn main() -> Result<(), Box<dyn Error>> {
    ui::init()?;

    let mut game = Game::new(SIZE);
    ui::draw_map(game.tiles())?;

    loop {
        if !game.end() {
            game.step();
            ui::draw_map(game.tiles())?
        }

        if poll(Duration::from_millis(150))? {
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
    }

    ui::reset()?;
    Ok(())
}
