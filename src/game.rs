use int_enum::IntEnum;
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
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Error::SnakeCrash => f.write_str("The Snake crashed."),
        }
    }
}

#[derive(Debug)]
pub struct Point {
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

pub struct Snake {
    body: VecDeque<Point>,
    direction: Direction,
}

impl Snake {
    pub fn set_direction(&mut self, direction: Direction) {
        if self.direction.opposite() != direction {
            return self.direction = direction;
        }
    }
}

type Tiles = Vec<Vec<Tile>>;

pub struct Map {
    pub tiles: Tiles,
    rng: ThreadRng,
}

impl Map {
    pub fn new(size: u16) -> Map {
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

        let mut map = Map {
            tiles,
            rng: thread_rng(),
        };

        map.spawn_food();
        map.spawn_obstacles();
        map
    }

    fn tile(&self, point: &Point) -> Tile {
        self.tiles[point.x as usize][point.y as usize]
    }

    fn set_tile(&mut self, point: &Point, tile: Tile) {
        self.tiles[point.x as usize][point.y as usize] = tile;
    }

    fn random_empty_point(&mut self, distance: u16) -> Point {
        let size = self.tiles.len() as u16;
        loop {
            let x = self.rng.gen_range(distance + 1..size - distance);
            let y = self.rng.gen_range(distance + 1..size - distance);
            let point = Point { x, y };
            if self.tile(&point) == Tile::Free {
                break point;
            }
        }
    }

    fn spawn_food(&mut self) {
        let point = self.random_empty_point(1);
        self.set_tile(&point, Tile::Food);
    }

    fn spawn_obstacles(&mut self) {
        let size = self.tiles.len() as u16;
        for _ in 0..=size / 3 {
            let point = self.random_empty_point(0);
            self.set_tile(&point, Tile::Obstacle);
        }
    }

    pub fn spawn_snake(&mut self) -> Snake {
        let point = self.random_empty_point(3);
        self.set_tile(&point, Tile::Snake);
        let mut snake = Snake {
            body: VecDeque::new(),
            direction: random_direction(),
        };
        snake.body.push_front(point);
        snake
    }
}

pub fn step(map: &mut Map, snake: &mut Snake) -> Result<(), Error> {
    // tile in front of the snake
    let next = snake.body.front().unwrap().next(snake.direction);

    match map.tile(&next) {
        Tile::Obstacle | Tile::Snake => {
            map.set_tile(&next, Tile::Crash);
            return Err(Error::SnakeCrash);
        }
        Tile::Food => map.spawn_food(),
        Tile::Free => {
            // remove last snake tile to "move" the snake
            let tail = snake.body.pop_back().unwrap();
            map.set_tile(&tail, Tile::Free);
        }
        Tile::Crash => unreachable!(),
    }

    // grow snake
    map.set_tile(&next, Tile::Snake);
    snake.body.push_front(next);

    Ok(())
}

pub fn random_direction() -> Direction {
    Direction::from_int(thread_rng().gen_range(0..=3) as u8).unwrap()
}
