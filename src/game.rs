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
pub type Snake = VecDeque<Point>;

pub struct Grid {
    pub tiles: Vec<Vec<Tile>>,
}

impl Grid {
    pub fn new(width: u16, height: u16) -> Self {
        assert!(width >= 12, "Minimum grid size is 12!");
        assert!(height >= 9, "Minimum grid size is 9!");
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

// Returns the current direction of the snake.
fn get_direction(snake: &Snake) -> Direction {
    let (head_x, head_y) = snake.front().unwrap();
    let (neck_x, neck_y) = snake.get(1).unwrap();
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

pub fn spawn_snake(grid: &mut Grid) -> Snake {
    let mut snake = VecDeque::with_capacity(2);

    // Spawn first snake point.
    let head = grid.random_empty_point(4);
    grid.set_tile(head, Tile::Snake);
    snake.push_front(head);

    // Spawn a second point in a random direction to ensure the snake is moving.
    let next = next(head, random_direction());
    grid.set_tile(next, Tile::Snake);
    snake.push_front(next);

    snake
}

pub fn spawn_food(grid: &mut Grid) -> Point {
    let p = grid.random_empty_point(1);
    grid.set_tile(p, Tile::Food);
    p
}

pub fn spawn_obstacles(grid: &mut Grid, count: u16) {
    for _ in 0..=count {
        let p = grid.random_empty_point(0);
        grid.set_tile(p, Tile::Obstacle);
    }
}

pub fn grow_snake(snake: &mut Snake, direction: Direction) -> Point {
    // The snake can't reverse direction. So if the new direction is the opposite
    // of the current one we discard it.
    let mut d = get_direction(snake);
    if direction != d.opposite() {
        d = direction;
    }

    // Add the next point in the direction as a new head to the snake.
    let head = *snake.front().unwrap();
    let next = next(head, d);
    snake.push_front(next);
    next
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
