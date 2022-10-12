use int_enum::IntEnum;
use rand::prelude::*;
use std::collections::VecDeque;
use std::fmt;

#[derive(Debug)]
pub struct SnakeCrash;

impl fmt::Display for SnakeCrash {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Snake crashed!")
    }
}

#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub enum Cell {
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

pub type Snake = VecDeque<(usize, usize)>;

fn advance_snake(snake: &mut Snake, new_direction: Direction) {
    // only use new direction if it is doesn't change by 180 degrees
    let mut direction = get_direction(snake);
    if new_direction != direction.opposite() {
        direction = new_direction;
    }

    let (x, y) = *snake.front().unwrap();
    let (next_x, next_y) = next_cell(x, y, direction);
    snake.push_front((next_x, next_y));
}

fn get_direction(snake: &Snake) -> Direction {
    let (head_x, head_y) = *snake.front().unwrap();
    let (next_x, next_y) = *snake.get(1).unwrap();
    if head_x > next_x {
        Direction::East
    } else if head_x < next_x {
        Direction::West
    } else if head_y > next_y {
        Direction::South
    } else {
        Direction::North
    }
}

fn remove_tail(snake: &mut Snake) -> (usize, usize) {
    snake.pop_back().unwrap()
}

pub type Grid = Vec<Vec<Cell>>;

pub fn create_grid(size: usize) -> Grid {
    assert!(size >= 10, "Minimum grid size is 10!");
    let mut cells = vec![vec![Cell::Free; size]; size];
    for x in 0..=size - 1 {
        for y in 0..=size - 1 {
            if x == 0 || y == 0 || x == size - 1 || y == size - 1 {
                cells[x][y] = Cell::Obstacle;
            };
        }
    }
    cells
}

// TODO: document distance parameter
fn random_empty_cell(grid: &Grid, distance: usize) -> (usize, usize) {
    let size = grid.len();
    loop {
        let x = thread_rng().gen_range(distance + 1..size - distance);
        let y = thread_rng().gen_range(distance + 1..size - distance);
        if grid[x][y] == Cell::Free {
            break (x, y);
        }
    }
}

pub fn spawn_snake(grid: &mut Grid) -> Snake {
    let (x, y) = random_empty_cell(grid, 4);
    grid[x][y] = Cell::Snake;
    grid[x + 1][y] = Cell::Snake;
    let mut snake = VecDeque::with_capacity(2);
    snake.push_front((x, y));
    snake.push_front((x + 1, y));
    snake
}

pub fn spawn_food(grid: &mut Grid) {
    let (x, y) = random_empty_cell(grid, 1);
    grid[x][y] = Cell::Food;
}

pub fn spawn_obstacles(grid: &mut Grid, count: usize) {
    for _ in 0..=count {
        let (x, y) = random_empty_cell(grid, 0);
        grid[x][y] = Cell::Obstacle;
    }
}

pub fn step(grid: &mut Grid, snake: &mut Snake, direction: Direction) -> Result<(), SnakeCrash> {
    advance_snake(snake, direction);

    let (x, y) = *snake.front().unwrap();
    match grid[x][y] {
        Cell::Obstacle | Cell::Snake => {
            grid[x][y] = Cell::Crash;
            return Err(SnakeCrash);
        }
        Cell::Food => {
            grid[x][y] = Cell::Snake;
            spawn_food(grid);
        }
        Cell::Free => {
            grid[x][y] = Cell::Snake;
            let (tail_x, tail_y) = remove_tail(snake);
            grid[tail_x][tail_y] = Cell::Free;
        }
        Cell::Crash => unreachable!(),
    }
    Ok(())
}

pub fn random_direction() -> Direction {
    Direction::from_int(thread_rng().gen_range(0..=3) as u8).unwrap()
}

fn next_cell(x: usize, y: usize, direction: Direction) -> (usize, usize) {
    match direction {
        Direction::North => (x, y - 1),
        Direction::South => (x, y + 1),
        Direction::West => (x - 1, y),
        Direction::East => (x + 1, y),
    }
}
