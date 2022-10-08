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
    pub end: bool,
    pub steps: u32,
}

impl Game {
    pub fn new(size: usize) -> Game {
        assert!(size >= 5, "Minimum grid size is 5!");

        let mut grid = create_grid(size);
        let snake = spawn_snake(&mut grid);
        spawn_food(&mut grid);
        spawn_obstacles(&mut grid);

        Game {
            grid,
            snake,
            end: false,
            steps: 0,
        }
    }

    pub fn step(&mut self, direction: Direction) {
        // cell in front of the snake in the given direction
        let (x, y) = snake_next_cell(&self.snake, direction);

        match self.grid[x][y] {
            Cell::Obstacle | Cell::Snake => {
                self.grid[x][y] = Cell::Crash;
                self.end = true;
                return;
            }
            Cell::Food => spawn_food(&mut self.grid),
            Cell::Free => {
                // remove last snake cell to "move" the snake
                let (tail_x, tail_y) = self.snake.pop_back().unwrap();
                self.grid[tail_x][tail_y] = Cell::Free;
            }
            Cell::Crash => unreachable!(),
        }

        // grow snake
        self.grid[x][y] = Cell::Snake;
        self.snake.push_front((x, y));

        self.steps += 1;
    }
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

fn snake_next_cell(snake: &Snake, new_direction: Direction) -> (usize, usize) {
    // only use new direction if it is doesn't change by 180 degrees
    let mut direction = snake_direction(snake);
    if new_direction != direction.opposite() {
        direction = new_direction;
    }

    let (x, y) = *snake.front().unwrap();
    next_cell(x, y, direction)
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

fn spawn_food(grid: &mut Grid) {
    let (x, y) = random_empty_cell(grid, 1);
    grid[x][y] = Cell::Food;
}

fn spawn_obstacles(grid: &mut Grid) {
    let size = grid.len();
    for _ in 0..=size / 2 {
        let (x, y) = random_empty_cell(grid, 0);
        grid[x][y] = Cell::Obstacle;
    }
}

fn spawn_snake(grid: &mut Grid) -> Snake {
    let (x, y) = random_empty_cell(grid, 4);
    grid[x][y] = Cell::Snake;
    grid[x + 1][y] = Cell::Snake;
    let mut snake = VecDeque::new();
    snake.push_front((x, y));
    snake.push_front((x + 1, y));
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

fn snake_direction(snake: &Snake) -> Direction {
    let (head_x, head_y) = snake.front().unwrap();
    let (next_x, next_y) = snake.get(1).unwrap();
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
