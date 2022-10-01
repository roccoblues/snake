use int_enum::IntEnum;
use rand::prelude::*;
use std::collections::VecDeque;
use std::fmt;

#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
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

#[derive(Debug, Eq, PartialEq)]
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

pub struct Snake {
    body: VecDeque<(u16, u16)>,
    direction: Direction,
}

impl Snake {
    pub fn set_direction(&mut self, direction: Direction) {
        if self.direction.opposite() != direction {
            self.direction = direction;
        }
    }
}

type Tiles = Vec<Vec<Tile>>;

pub struct Map {
    pub tiles: Tiles,
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

        let mut map = Map { tiles };

        map.spawn_food();
        map.spawn_obstacles();
        map
    }

    fn random_empty_point(&mut self, distance: u16) -> (u16, u16) {
        let size = self.tiles.len() as u16;
        loop {
            let x = self.rng.gen_range(distance + 1..size - distance);
            let y = self.rng.gen_range(distance + 1..size - distance);
            if self.tiles[x as usize][y as usize] == Tile::Free {
                break (x, y);
            }
        }
    }

    fn spawn_food(&mut self) {
        let (x, y) = self.random_empty_point(1);
        self.tiles[x as usize][y as usize] = Tile::Food;
    }

    fn spawn_obstacles(&mut self) {
        let size = self.tiles.len() as u16;
        for _ in 0..=size / 3 {
            let (x, y) = self.random_empty_point(0);
            self.tiles[x as usize][y as usize] = Tile::Obstacle;
        }
    }

    pub fn spawn_snake(&mut self) -> Snake {
        let (x, y) = self.random_empty_point(3);
        self.tiles[x as usize][y as usize] = Tile::Snake;
        let mut snake = Snake {
            body: VecDeque::new(),
            direction: random_direction(),
        };
        snake.body.push_front((x, y));
        snake
    }
}

pub fn step(map: &mut Map, snake: &mut Snake) -> Result<(), Error> {
    let (head_x, head_y) = *snake.body.front().unwrap();

    // tile in front of the snake
    let (x, y) = next_point(head_x, head_y, snake.direction);

    match map.tiles[x as usize][y as usize] {
        Tile::Obstacle | Tile::Snake => {
            map.tiles[x as usize][y as usize] = Tile::Crash;
            return Err(Error::SnakeCrash);
        }
        Tile::Food => map.spawn_food(),
        Tile::Free => {
            // remove last snake tile to "move" the snake
            let (tail_x, tail_y) = snake.body.pop_back().unwrap();
            map.tiles[tail_x as usize][tail_y as usize] = Tile::Free;
        }
        Tile::Crash => unreachable!(),
    }

    // grow snake
    map.tiles[x as usize][y as usize] = Tile::Snake;
    snake.body.push_front((x, y));

    Ok(())
}

fn next_point(x: u16, y: u16, direction: Direction) -> (u16, u16) {
    match direction {
        Direction::Up => (x, y - 1),
        Direction::Down => (x, y + 1),
        Direction::Left => (x - 1, y),
        Direction::Right => (x + 1, y),
    }
}

fn random_direction() -> Direction {
    Direction::from_int(thread_rng().gen_range(0..=3) as u8).unwrap()
}
