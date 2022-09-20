use int_enum::{IntEnum, IntEnumError};
use rand::prelude::*;
use std::collections::VecDeque;

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
pub struct Map {
    tiles: Vec<Vec<Tile>>,
    snake: VecDeque<Point>,
}

impl Map {
    pub fn new(size: u16) -> Map {
        let mut tiles = vec![vec![Tile::Free; size as usize]; size as usize];
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

        let mut rng = thread_rng();
        let mut snake = VecDeque::new();
        let x = rng.gen_range(2..=size - 3) as u16;
        let y = rng.gen_range(2..=size - 3) as u16;
        tiles[x as usize][y as usize] = Tile::Snake;
        snake.push_front(Point(x, y));

        Map { tiles, snake }
    }

    pub fn tiles(&self) -> &Vec<Vec<Tile>> {
        &self.tiles
    }

    pub fn advance_snake(&mut self, direction: Direction) {
        let prev = self.snake.front().unwrap();
        let next = match direction {
            Direction::Up => Point(prev.0, prev.1 - 1),
            Direction::Down => Point(prev.0, prev.1 + 1),
            Direction::Left => Point(prev.0 - 1, prev.1),
            Direction::Right => Point(prev.0 + 1, prev.1),
        };

        let tail = self.snake.pop_back().unwrap();
        self.tiles[tail.x()][tail.y()] = Tile::Free;

        if self.tiles[next.x()][next.y()] == Tile::Obstacle {
            self.tiles[next.x()][next.y()] = Tile::Crash;
            // TODO: return error
        } else {
            self.snake.push_front(Point(next.0, next.1));
            self.tiles[next.x()][next.y()] = Tile::Snake;
        }
    }
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

pub fn random_direction() -> Result<Direction, IntEnumError<Direction>> {
    let mut rng = thread_rng();
    Direction::from_int(rng.gen_range(0..=3) as u8)
}
