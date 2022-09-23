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

impl Direction {
    fn opposite(&self) -> Direction {
        match self {
            Direction::Up => Direction::Down,
            Direction::Down => Direction::Up,
            Direction::Left => Direction::Right,
            Direction::Right => Direction::Left,
        }
    }
}

#[derive(Debug)]
pub enum Error {
    SnakeCrash,
    InvalidDirection,
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Error::SnakeCrash => f.write_str("The Snake crashed."),
            Error::InvalidDirection => f.write_str("Invalid direction."),
        }
    }
}

struct Snake {
    body: VecDeque<(u16, u16)>,
    direction: Direction,
}

impl Snake {
    fn new(x: u16, y: u16) -> Snake {
        let mut body = VecDeque::new();
        body.push_front((x, y));
        Snake {
            body,
            direction: random_direction().unwrap(),
        }
    }

    fn head(&self) -> (u16, u16) {
        *self.body.front().unwrap()
    }

    fn remove_tail(&mut self) -> (u16, u16) {
        self.body.pop_back().unwrap()
    }

    fn advance(&mut self) {
        let (x, y) = self.head();
        self.body.push_front(match self.direction {
            Direction::Up => (x, y - 1),
            Direction::Down => (x, y + 1),
            Direction::Left => (x - 1, y),
            Direction::Right => (x + 1, y),
        })
    }
}

struct Map {
    tiles: Vec<Vec<Tile>>,
    size: u16,
}

impl Map {
    fn new(size: u16) -> Map {
        let mut tiles = vec![vec![Tile::Free; size.into()]; size.into()];
        for x in 0..=size - 1 {
            for y in 0..=size - 1 {
                let tile = if x == 0 || y == 0 || x == size - 1 || y == size - 1 {
                    Tile::Obstacle
                } else {
                    Tile::Free
                };
                tiles[x as usize][y as usize] = tile;
            }
        }
        Map { tiles, size }
    }

    fn tile(&self, x: u16, y: u16) -> Tile {
        self.tiles[x as usize][y as usize]
    }

    fn set_tile(&mut self, x: u16, y: u16, tile: Tile) {
        self.tiles[x as usize][y as usize] = tile;
    }

    fn spawn_food(&mut self) {
        let (x, y) = self.random_empty_point(0);
        self.set_tile(x, y, Tile::Food);
    }

    fn spawn_obstacles(&mut self) {
        for _ in 0..=self.size / 3 {
            let (x, y) = self.random_empty_point(0);
            self.set_tile(x, y, Tile::Obstacle);
        }
    }

    fn spawn_snake(&mut self) -> (u16, u16) {
        let (x, y) = self.random_empty_point(3);
        self.set_tile(x, y, Tile::Snake);
        (x, y)
    }

    fn random_empty_point(&self, distance: u16) -> (u16, u16) {
        let mut rng = thread_rng();
        loop {
            let x = rng.gen_range(distance + 1..=self.size - distance - 1) as u16;
            let y = rng.gen_range(distance + 1..=self.size - distance - 1) as u16;
            if self.tile(x, y) == Tile::Free {
                break (x, y);
            }
        }
    }
}

pub struct Game {
    map: Map,
    snake: Snake,
}

impl Game {
    pub fn new(size: u16) -> Game {
        let mut map = Map::new(size);
        map.spawn_food();
        map.spawn_obstacles();
        let (x, y) = map.spawn_snake();
        let snake = Snake::new(x, y);
        Game { map, snake }
    }

    pub fn step(&mut self) -> Result<(), Error> {
        self.snake.advance();

        let (head_x, head_y) = self.snake.head();
        match self.map.tile(head_x, head_y) {
            Tile::Obstacle | Tile::Snake => {
                self.map.set_tile(head_x, head_y, Tile::Crash);
                return Err(Error::SnakeCrash);
            }
            Tile::Food => self.map.spawn_food(),
            _ => {
                let (tail_x, tail_y) = self.snake.remove_tail();
                self.map.set_tile(tail_x, tail_y, Tile::Free);
            }
        }

        self.map.set_tile(head_x, head_y, Tile::Snake);

        Ok(())
    }

    pub fn tiles(&self) -> &Vec<Vec<Tile>> {
        &self.map.tiles
    }

    pub fn change_direction(&mut self, direction: Direction) -> Result<(), Error> {
        if self.snake.direction.opposite() == direction {
            return Err(Error::InvalidDirection);
        }
        self.snake.direction = direction;
        Ok(())
    }
}

fn random_direction() -> Result<Direction, IntEnumError<Direction>> {
    let mut rng = thread_rng();
    Direction::from_int(rng.gen_range(0..=3) as u8)
}
