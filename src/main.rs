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
    ui::draw_map(&game.tiles())?;
    // ui::draw_score(game.steps());

    let mut crash = false;

    loop {
        if !crash {
            match game.step() {
                Err(game::Error::SnakeCrash) => crash = true,
                _ => {}
            }
            ui::draw_map(&game.tiles())?
        }

        if poll(Duration::from_millis(100))? {
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

    ui::reset()?;
    Ok(())
}
