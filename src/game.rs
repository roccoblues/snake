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

pub struct Point(u16, u16);

impl Point {
    pub fn x(&self) -> u16 {
        self.0
    }
    pub fn y(&self) -> u16 {
        self.1
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
    body: VecDeque<Point>,
    direction: Direction,
}

impl Snake {
    fn new() -> Snake {
        Snake {
            body: VecDeque::new(),
            direction: random_direction().unwrap(),
        }
    }

    fn head(&self) -> &Point {
        self.body.front().unwrap()
    }

    fn remove_tail(&mut self) -> Point {
        self.body.pop_back().unwrap()
    }

    fn advance(&mut self) {
        let head = self.head();
        let next = match self.direction {
            Direction::Up => Point(head.0, head.1 - 1),
            Direction::Down => Point(head.0, head.1 + 1),
            Direction::Left => Point(head.0 - 1, head.1),
            Direction::Right => Point(head.0 + 1, head.1),
        };
        self.body.push_front(next);
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

    fn tile(&self, point: &Point) -> Tile {
        self.tiles[point.x() as usize][point.y() as usize]
    }

    fn set_tile(&mut self, point: &Point, tile: Tile) {
        self.tiles[point.x() as usize][point.y() as usize] = tile;
    }

    fn random_empty_point(&self, distance: u16) -> Point {
        let mut rng = thread_rng();
        loop {
            let x = rng.gen_range(distance + 1..=self.size - distance - 1) as u16;
            let y = rng.gen_range(distance + 1..=self.size - distance - 1) as u16;
            let point = Point(x, y);
            if self.tile(&point) == Tile::Free {
                break point;
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
        let mut game = Game {
            map: Map::new(size),
            snake: Snake::new(),
        };
        game.spawn_snake();
        game.spawn_food();
        game.spawn_obstacles();
        game
    }

    pub fn step(&mut self) -> Result<(), Error> {
        self.snake.advance();

        match self.map.tile(self.snake.head()) {
            Tile::Obstacle | Tile::Snake => {
                self.map.set_tile(self.snake.head(), Tile::Crash);
                return Err(Error::SnakeCrash);
            }
            Tile::Food => self.spawn_food(),
            _ => {
                let tail = self.snake.remove_tail();
                self.map.set_tile(&tail, Tile::Free);
            }
        }

        self.map.set_tile(self.snake.head(), Tile::Snake);

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

    fn spawn_food(&mut self) {
        self.map
            .set_tile(&self.map.random_empty_point(0), Tile::Food);
    }

    fn spawn_snake(&mut self) {
        let point = self.map.random_empty_point(3);
        self.map.set_tile(&point, Tile::Snake);
        self.snake.body.push_front(point);
    }

    fn spawn_obstacles(&mut self) {
        for _ in 0..=self.map.size / 3 {
            self.map
                .set_tile(&self.map.random_empty_point(0), Tile::Obstacle);
        }
    }
}

fn random_direction() -> Result<Direction, IntEnumError<Direction>> {
    let mut rng = thread_rng();
    Direction::from_int(rng.gen_range(0..=3) as u8)
}
