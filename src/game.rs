use int_enum::IntEnum;
use rand::prelude::*;
use std::collections::VecDeque;

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
    North = 0,
    South = 1,
    West = 2,
    East = 3,
}

impl Direction {
    fn opposite(&self) -> Direction {
        match self {
            Direction::North => Direction::South,
            Direction::South => Direction::North,
            Direction::West => Direction::East,
            Direction::East => Direction::West,
        }
    }
}

pub type Point = (usize, usize);

pub struct Snake {
    body: VecDeque<Point>,
}

impl Snake {
    pub fn head(&self) -> Point {
        *self.body.front().unwrap()
    }

    // Returns the current direction of the snake.
    fn direction(&self) -> Direction {
        let (head_x, head_y) = self.head();
        let (neck_x, neck_y) = *self.body.get(1).unwrap();
        if head_x > neck_x {
            Direction::East
        } else if head_x < neck_x {
            Direction::West
        } else if head_y > neck_y {
            Direction::South
        } else {
            Direction::North
        }
    }

    pub fn grow(&mut self, direction: Direction) -> Point {
        // The snake can't reverse direction. So if the new direction is the opposite
        // of the current one we discard it.
        let mut d = self.direction();
        if direction != d.opposite() {
            d = direction;
        }

        // Add the next point in the direction as a new head to the snake.
        let head = self.head();
        let next = next(head, d);
        self.body.push_front(next);
        next
    }

    pub fn len(&self) -> u16 {
        self.body.len() as u16
    }

    pub fn remove_tail(&mut self) -> Point {
        self.body.pop_back().unwrap()
    }
}

pub struct Grid {
    tiles: Vec<Vec<Tile>>,
}

impl Grid {
    pub fn new(width: u16, height: u16) -> Self {
        let mut tiles = vec![vec![Tile::Free; height.into()]; width.into()];
        for x in 0..=width - 1 {
            for y in 0..=height - 1 {
                if x == 0 || y == 0 || x == width - 1 || y == height - 1 {
                    tiles[x as usize][y as usize] = Tile::Obstacle;
                };
            }
        }
        Self { tiles }
    }

    pub fn tile(&self, p: Point) -> Tile {
        let (x, y) = p;
        self.tiles[x][y]
    }

    pub fn set_tile(&mut self, p: Point, tile: Tile) {
        let (x, y) = p;
        self.tiles[x][y] = tile
    }

    pub fn width(&self) -> usize {
        self.tiles.len()
    }

    pub fn height(&self) -> usize {
        self.tiles[0].len()
    }

    pub fn spawn_snake(&mut self) -> Snake {
        let mut body = VecDeque::with_capacity(2);

        // Spawn first snake point.
        let head = self.random_empty_point(4);
        self.set_tile(head, Tile::Snake);
        body.push_front(head);

        // Spawn a second point in a random direction to ensure the snake is moving.
        let next = next(head, random_direction());
        self.set_tile(next, Tile::Snake);
        body.push_front(next);

        Snake { body }
    }

    pub fn spawn_food(&mut self) -> Point {
        let p = self.random_empty_point(1);
        self.set_tile(p, Tile::Food);
        p
    }

    pub fn spawn_obstacles(&mut self, count: u16) {
        for _ in 0..=count {
            let p = self.random_empty_point(0);
            self.set_tile(p, Tile::Obstacle);
        }
    }

    // Returns a random empty point on the grid. The distance parameter specifies
    // the minimum distance from the edge of the grid.
    fn random_empty_point(&self, distance: usize) -> Point {
        loop {
            let x = thread_rng().gen_range(distance + 1..self.width() - distance);
            let y = thread_rng().gen_range(distance + 1..self.height() - distance);
            let p = (x, y);
            if self.tile(p) == Tile::Free {
                break p;
            }
        }
    }
}

pub fn random_direction() -> Direction {
    Direction::from_int(thread_rng().gen_range(0..=3) as u8).unwrap()
}

// Returns the next point in the given direction.
fn next(p: Point, direction: Direction) -> Point {
    let (x, y) = p;
    match direction {
        Direction::North => (x, y - 1),
        Direction::South => (x, y + 1),
        Direction::West => (x - 1, y),
        Direction::East => (x + 1, y),
    }
}
