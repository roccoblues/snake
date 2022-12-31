use std::sync::atomic::{self, AtomicU16};
use std::sync::mpsc::Sender;
use std::sync::Arc;
use std::thread;
use std::time::Duration;

use crate::types::Direction;
use crossterm::event;
use crossterm::event::{Event, KeyCode};

#[repr(u8)]
#[derive(Debug, Eq, PartialEq)]
pub enum Input {
    Pause,
    Exit,
    Step,
    ChangeDirection(Direction),
    IncreaseSpeed,
    DecreaseSpeed,
    Unknown,
}

// Spawns a thread that reads input events and sends them to the channel.
pub fn handle(tx: Sender<Input>) {
    thread::spawn(move || loop {
        tx.send(read()).unwrap();
    });
}

// Spawns a thread that sends Input::Step at the specified interval.
pub fn send_ticks(tx: Sender<Input>, interval: Arc<AtomicU16>) {
    thread::spawn(move || loop {
        thread::sleep(Duration::from_millis(
            interval.load(atomic::Ordering::Relaxed).into(),
        ));
        tx.send(Input::Step).unwrap();
    });
}

// Waits for an UI event and returns the corresponding Input enum.
fn read() -> Input {
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
