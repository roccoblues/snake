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

#[derive(Debug)]
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

type Map = Vec<Vec<Tile>>;

pub struct Game {
    pub map: Map,
    pub steps: u16,
    direction: Direction,
    snake: VecDeque<Point>,
}

impl Game {
    pub fn new(size: u16) -> Game {
        let mut map = new_map(size);
        spawn_food(&mut map);
        spawn_obstacles(&mut map);

        let snake = spawn_snake(&mut map);

        Game {
            map,
            snake,
            direction: random_direction().unwrap(),
            steps: 0,
        }
    }

    pub fn step(&mut self) -> Result<(), Error> {
        // tile in front of the snake
        let next = self.snake.front().unwrap().next(self.direction);

        match self.map[next.x as usize][next.y as usize] {
            Tile::Obstacle | Tile::Snake => {
                set_tile(&mut self.map, &next, Tile::Crash);
                return Err(Error::SnakeCrash);
            }
            Tile::Food => spawn_food(&mut self.map),
            Tile::Free => {
                // remove last snake tile to "move" the snake
                let tail = self.snake.pop_back().unwrap();
                set_tile(&mut self.map, &tail, Tile::Free);
            }
            Tile::Crash => unreachable!(),
        }

        // grow snake
        set_tile(&mut self.map, &next, Tile::Snake);
        self.snake.push_front(next);

        self.steps += 1;

        Ok(())
    }

    pub fn change_direction(&mut self, direction: Direction) -> Result<(), Error> {
        if self.direction.opposite() == direction {
            return Err(Error::InvalidDirection);
        }
        self.direction = direction;
        Ok(())
    }
}

fn random_direction() -> Result<Direction, IntEnumError<Direction>> {
    Direction::from_int(thread_rng().gen_range(0..=3) as u8)
}

fn random_empty_point(map: &Map, distance: u16) -> Point {
    let size = map.len() as u16;
    loop {
        let x = thread_rng().gen_range(distance + 1..size - distance);
        let y = thread_rng().gen_range(distance + 1..size - distance);
        if map[x as usize][y as usize] == Tile::Free {
            break Point { x, y };
        }
    }
}

fn set_tile(map: &mut Map, point: &Point, tile: Tile) {
    map[point.x as usize][point.y as usize] = tile;
}

fn new_map(size: u16) -> Map {
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
    tiles
}

fn spawn_food(map: &mut Map) {
    let point = random_empty_point(&map, 1);
    set_tile(map, &point, Tile::Food);
}

fn spawn_obstacles(map: &mut Map) {
    let size = map.len() as u16;
    for _ in 0..=size / 3 {
        let point = random_empty_point(&map, 0);
        set_tile(map, &point, Tile::Obstacle);
    }
}

fn spawn_snake(map: &mut Map) -> VecDeque<Point> {
    let mut snake = VecDeque::new();
    let point = random_empty_point(&map, 3);
    set_tile(map, &point, Tile::Snake);
    snake.push_front(point);
    snake
}
