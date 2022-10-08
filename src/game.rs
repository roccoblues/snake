use int_enum::IntEnum;
use rand::prelude::*;
use std::collections::VecDeque;

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
    fn opposite(&self) -> Direction {
        match self {
            Direction::North => Direction::South,
            Direction::South => Direction::North,
            Direction::West => Direction::East,
            Direction::East => Direction::West,
        }
    }
}

pub type Grid = Vec<Vec<Cell>>;
type Snake = VecDeque<(usize, usize)>;

pub struct Game {
    pub grid: Grid,
    pub snake: Snake,
    direction: Direction,
    pub end: bool,
    pub steps: u32,
}

impl Game {
    pub fn new(size: usize) -> Game {
        let mut grid = create_grid(size);
        spawn_food(&mut grid);
        spawn_obstacles(&mut grid);
        let snake = spawn_snake(&mut grid);

        Game {
            grid,
            snake,
            direction: random_direction(),
            end: false,
            steps: 0,
        }
    }

    pub fn step(&mut self) {
        let (head_x, head_y) = *self.snake.front().unwrap();

        // cell in front of the snake
        let (x, y) = next_cell(head_x, head_y, self.direction);
        match self.grid[x as usize][y as usize] {
            Cell::Obstacle | Cell::Snake => {
                self.grid[x as usize][y as usize] = Cell::Crash;
                self.end = true;
                return;
            }
            Cell::Food => spawn_food(&mut self.grid),
            Cell::Free => {
                // remove last snake cell to "move" the snake
                let (tail_x, tail_y) = self.snake.pop_back().unwrap();
                self.grid[tail_x as usize][tail_y as usize] = Cell::Free;
            }
            Cell::Crash => unreachable!(),
        }

        // grow snake
        self.grid[x as usize][y as usize] = Cell::Snake;
        self.snake.push_front((x, y));

        self.steps += 1;
    }

    pub fn set_direction(&mut self, direction: Direction) {
        if self.direction.opposite() != direction {
            self.direction = direction;
        }
    }

    pub fn direction(&self) -> Direction {
        self.direction
    }
}

fn next_cell(x: usize, y: usize, direction: Direction) -> (usize, usize) {
    match direction {
        Direction::North => (x, y - 1),
        Direction::South => (x, y + 1),
        Direction::West => (x - 1, y),
        Direction::East => (x + 1, y),
    }
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

fn random_direction() -> Direction {
    Direction::from_int(thread_rng().gen_range(0..=3) as u8).unwrap()
}

fn spawn_food(grid: &mut Grid) {
    let (x, y) = random_empty_cell(grid, 1);
    grid[x as usize][y as usize] = Cell::Food;
}

fn spawn_obstacles(grid: &mut Grid) {
    let size = grid.len();
    for _ in 0..=size / 2 {
        let (x, y) = random_empty_cell(grid, 0);
        grid[x as usize][y as usize] = Cell::Obstacle;
    }
}

fn spawn_snake(grid: &mut Grid) -> Snake {
    let (x, y) = random_empty_cell(grid, 3);
    grid[x][y] = Cell::Snake;
    let mut snake = VecDeque::new();
    snake.push_front((x, y));
    snake
}

fn create_grid(size: usize) -> Grid {
    let mut grid = vec![vec![Cell::Free; size]; size];
    for x in 0..=size - 1 {
        for y in 0..=size - 1 {
            if x == 0 || y == 0 || x == size - 1 || y == size - 1 {
                grid[x][y] = Cell::Obstacle;
            };
        }
    }
    grid
}
