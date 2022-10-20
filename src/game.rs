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
    pub fn opposite(&self) -> Direction {
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
pub type Grid = Vec<Vec<Tile>>;

pub fn create_grid(width: usize, height: usize) -> Grid {
    let mut grid = vec![vec![Tile::Free; height]; width];
    for (x, row) in grid.iter_mut().enumerate() {
        for (y, tile) in row.iter_mut().enumerate() {
            if x == 0 || y == 0 || x == width - 1 || y == height - 1 {
                *tile = Tile::Obstacle;
            };
        }
    }
    grid
}

pub fn spawn_snake(grid: &mut Grid) -> Snake {
    let (x, y) = random_empty_point(grid, 4);
    grid[x][y] = Tile::Snake;
    let mut snake = VecDeque::with_capacity(10);
    snake.push_front((x, y));
    snake.push_front(next_point((x, y), random_direction()));
    snake
}

pub fn spawn_food(grid: &mut Grid) -> Point {
    let (x, y) = random_empty_point(grid, 1);
    grid[x][y] = Tile::Food;
    (x, y)
}

pub fn spawn_obstacles(grid: &mut Grid, count: u16) {
    for _ in 0..=count {
        let (x, y) = random_empty_point(grid, 0);
        grid[x][y] = Tile::Obstacle;
    }
}

// Returns a random empty point on the grid. The distance parameter specifies
// the minimum distance from the edge of the grid.
fn random_empty_point(grid: &Grid, distance: usize) -> Point {
    let min_x = distance;
    let max_x = grid.len() - distance - 1;
    let min_y = distance;
    let max_y = grid[0].len() - distance - 1;

    let mut points = Vec::with_capacity(grid.len() * grid[0].len());
    for (x, row) in grid.iter().enumerate() {
        for (y, tile) in row.iter().enumerate() {
            if x > min_x && x < max_x && y > min_y && y < max_y && *tile == Tile::Free {
                points.push((x, y))
            }
        }
    }

    *points.get(thread_rng().gen_range(0..points.len())).unwrap()
}

pub fn random_direction() -> Direction {
    Direction::from_int(thread_rng().gen_range(0..=3) as u8).unwrap()
}

// Returns the next point in the given direction.
pub fn next_point(p: Point, direction: Direction) -> Point {
    let (x, y) = p;
    match direction {
        Direction::North => (x, y - 1),
        Direction::South => (x, y + 1),
        Direction::West => (x - 1, y),
        Direction::East => (x + 1, y),
    }
}
