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

#[derive(Debug, PartialEq)]
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

struct Point {
    x: u16,
    y: u16,
}

impl Point {
    fn next(&self, direction: Direction) -> Point {
        let (x, y) = match direction {
            Direction::Up => (self.x, self.y - 1),
            Direction::Down => (self.x, self.y + 1),
            Direction::Left => (self.x - 1, self.y),
            Direction::Right => (self.x + 1, self.y),
        };
        Point { x, y }
    }
}

struct Snake {
    body: VecDeque<Point>,
    direction: Direction,
}

impl Snake {
    fn new(point: Point) -> Snake {
        let mut body = VecDeque::new();
        body.push_front(point);
        Snake {
            body,
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
        self.body.push_front(self.head().next(self.direction));
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
        self.tiles[point.x as usize][point.y as usize]
    }

    fn set_tile(&mut self, point: &Point, tile: Tile) {
        self.tiles[point.x as usize][point.y as usize] = tile;
    }

    fn spawn_food(&mut self) {
        self.set_tile(&self.random_empty_point(1), Tile::Food);
    }

    fn spawn_obstacles(&mut self) {
        for _ in 0..=self.size / 3 {
            self.set_tile(&self.random_empty_point(0), Tile::Obstacle);
        }
    }

    fn spawn_snake(&mut self) -> Point {
        let point = self.random_empty_point(3);
        self.set_tile(&point, Tile::Snake);
        point
    }

    fn random_empty_point(&self, distance: u16) -> Point {
        loop {
            let x = thread_rng().gen_range(distance + 1..=self.size - distance - 1) as u16;
            let y = thread_rng().gen_range(distance + 1..=self.size - distance - 1) as u16;
            let point = Point { x, y };
            if self.tile(&point) == Tile::Free {
                break point;
            }
        }
    }
}

pub struct Game {
    map: Map,
    snake: Snake,
    steps: u16,
}

impl Game {
    pub fn new(size: u16) -> Game {
        let mut map = Map::new(size);
        map.set_tile(&map.random_empty_point(1), Tile::Food);

        // map.spawn_obstacles();
        let snake = Snake::new(map.spawn_snake());
        Game {
            map,
            snake,
            steps: 0,
        }
    }

    pub fn step(&mut self) -> Result<(), Error> {
        self.snake.advance();

        match self.map.tile(self.snake.head()) {
            Tile::Obstacle | Tile::Snake => {
                self.map.set_tile(self.snake.head(), Tile::Crash);
                return Err(Error::SnakeCrash);
            }
            Tile::Food => self.map.spawn_food(),
            Tile::Free => {
                let tail = self.snake.remove_tail();
                self.map.set_tile(&tail, Tile::Free);
            }
            Tile::Crash => unreachable!(),
        }

        self.map.set_tile(self.snake.head(), Tile::Snake);
        self.steps += 1;

        Ok(())
    }

    pub fn tiles(&self) -> &Vec<Vec<Tile>> {
        &self.map.tiles
    }

    pub fn steps(&self) -> u16 {
        self.steps
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
    Direction::from_int(thread_rng().gen_range(0..=3) as u8)
}
