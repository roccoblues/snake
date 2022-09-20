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

#[derive(Debug, Copy, Clone)]
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

#[derive(Debug)]
struct Snake {
    body: VecDeque<Point>,
}

impl Snake {
    fn new() -> Snake {
        Snake {
            body: VecDeque::new(),
        }
    }

    fn head(&self) -> &Point {
        self.body.front().unwrap()
    }

    fn remove_tail(&mut self) -> Point {
        self.body.pop_back().unwrap()
    }

    fn next(&self, direction: Direction) -> Point {
        let head = self.head();
        match direction {
            Direction::Up => Point(head.0, head.1 - 1),
            Direction::Down => Point(head.0, head.1 + 1),
            Direction::Left => Point(head.0 - 1, head.1),
            Direction::Right => Point(head.0 + 1, head.1),
        }
    }

    fn grow_head(&mut self, point: Point) {
        self.body.push_front(point);
    }
}

#[derive(Debug)]
pub struct Game {
    pub map: Vec<Vec<Tile>>,
    snake: Snake,
    size: u16,
}

impl Game {
    pub fn new(size: u16) -> Game {
        let mut game = Game {
            size,
            map: vec![vec![Tile::Free; size as usize]; size as usize],
            snake: Snake::new(),
        };
        game.initialize_map();
        game.spawn_snake();
        game.spawn_food();
        game
    }

    pub fn advance_snake(&mut self, direction: Direction) -> Result<(), Error> {
        let next = self.snake.next(direction);
        let next_tile = self.tile(&next);

        if next_tile == Tile::Obstacle || next_tile == Tile::Snake {
            self.set_tile(next, Tile::Crash);
            return Err(Error::SnakeCrash);
        }

        if next_tile == Tile::Food {
            self.spawn_food()
        } else {
            let tail = self.snake.remove_tail();
            self.set_tile(tail, Tile::Free);
        }

        self.snake.grow_head(next);
        self.set_tile(next, Tile::Snake);

        Ok(())
    }

    fn tile(&self, point: &Point) -> Tile {
        self.map[point.x()][point.y()]
    }

    fn set_tile(&mut self, point: Point, tile: Tile) {
        self.map[point.x()][point.y()] = tile;
    }

    fn spawn_food(&mut self) {
        self.set_random_empty_point(1, self.size - 2, Tile::Food);
    }

    fn spawn_snake(&mut self) {
        let point = self.set_random_empty_point(2, self.size - 3, Tile::Snake);
        self.snake.grow_head(point);
    }

    fn set_random_empty_point(&mut self, min: u16, max: u16, tile: Tile) -> Point {
        let mut rng = thread_rng();
        let point = loop {
            let x = rng.gen_range(min..=max) as u16;
            let y = rng.gen_range(min..=max) as u16;
            let point = Point(x, y);
            if self.tile(&point) == Tile::Free {
                break point;
            }
        };
        self.set_tile(point, tile);
        point
    }

    fn initialize_map(&mut self) {
        for x in 0..=self.size - 1 {
            for y in 0..=self.size - 1 {
                let tile = if x == 0 || y == 0 || x == self.size - 1 || y == self.size - 1 {
                    Tile::Obstacle
                } else {
                    Tile::Free
                };
                self.map[x as usize][y as usize] = tile;
            }
        }
    }
}

pub fn random_direction() -> Result<Direction, IntEnumError<Direction>> {
    let mut rng = thread_rng();
    Direction::from_int(rng.gen_range(0..=3) as u8)
}
