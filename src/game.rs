use int_enum::{IntEnum, IntEnumError};
use rand::prelude::*;
use std::collections::VecDeque;
use std::fmt;

#[repr(u8)]
#[derive(Copy, Clone, Debug, PartialEq)]
pub enum Tile {
    Free,
    Snake,
    Food,
    Obstacle,
    Crash,
}

#[repr(u8)]
#[derive(Clone, Copy, Debug, Eq, PartialEq, IntEnum)]
pub enum Direction {
    Up = 0,
    Down = 1,
    Left = 2,
    Right = 3,
}

#[derive(Debug)]
pub struct Point(u16, u16);

impl Point {
    pub fn x(&self) -> usize {
        self.0 as usize
    }
    pub fn y(&self) -> usize {
        self.1 as usize
    }
}

#[derive(Debug)]
pub struct Game {
    pub map: Vec<Vec<Tile>>,
    snake: VecDeque<Point>,
}

#[derive(Debug)]
pub enum Error {
    SnakeCrash,
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Error::SnakeCrash => f.write_str("The Snake crashed."),
        }
    }
}

impl Game {
    pub fn new(size: u16) -> Game {
        // create map
        let mut map = vec![vec![Tile::Free; size as usize]; size as usize];
        for x in 0..=size - 1 {
            for y in 0..=size - 1 {
                let tile = if x == 0 || y == 0 || x == size - 1 || y == size - 1 {
                    Tile::Obstacle
                } else {
                    Tile::Free
                };
                map[x as usize][y as usize] = tile;
            }
        }

        // spawn snake
        let mut rng = thread_rng();
        let mut snake = VecDeque::new();
        let x = rng.gen_range(2..=size - 3) as u16;
        let y = rng.gen_range(2..=size - 3) as u16;
        map[x as usize][y as usize] = Tile::Snake;
        snake.push_front(Point(x, y));

        Game { map, snake }
    }

    pub fn advance_snake(&mut self, direction: Direction) -> Result<(), Error> {
        let prev = self.snake.front().unwrap();
        let next = match direction {
            Direction::Up => Point(prev.0, prev.1 - 1),
            Direction::Down => Point(prev.0, prev.1 + 1),
            Direction::Left => Point(prev.0 - 1, prev.1),
            Direction::Right => Point(prev.0 + 1, prev.1),
        };

        let tail = self.snake.pop_back().unwrap();
        self.map[tail.x()][tail.y()] = Tile::Free;

        if self.map[next.x()][next.y()] == Tile::Obstacle {
            self.map[next.x()][next.y()] = Tile::Crash;
            return Err(Error::SnakeCrash);
        } else {
            self.snake.push_front(Point(next.0, next.1));
            self.map[next.x()][next.y()] = Tile::Snake;
        }
        Ok(())
    }
}

pub fn random_direction() -> Result<Direction, IntEnumError<Direction>> {
    let mut rng = thread_rng();
    Direction::from_int(rng.gen_range(0..=3) as u8)
}
