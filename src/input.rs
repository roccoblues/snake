use crate::game::Direction;
use crossterm::event;
use crossterm::event::{Event, KeyCode};

#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub enum Input {
    Pause,
    Exit,
    Step,
    ChangeDirection(Direction),
    IncreaseSpeed,
    DecreaseSpeed,
    Unknown,
}

// Waits for an ui event and returns the corresponding Input enum.
pub fn read() -> Input {
    let e = event::read().unwrap();
    match e {
        Event::Key(key_event) => match key_event.code {
            KeyCode::Esc | KeyCode::Char('q') => Input::Exit,
            KeyCode::Up => Input::ChangeDirection(Direction::North),
            KeyCode::Down => Input::ChangeDirection(Direction::South),
            KeyCode::Right => Input::ChangeDirection(Direction::East),
            KeyCode::Left => Input::ChangeDirection(Direction::West),
            KeyCode::Char(' ') => Input::Pause,
            KeyCode::Char('+') => Input::IncreaseSpeed,
            KeyCode::Char('-') => Input::DecreaseSpeed,
            _ => Input::Unknown,
        },
        _ => Input::Unknown,
    }
}
