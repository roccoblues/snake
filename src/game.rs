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

pub struct Grid {
    pub cells: Vec<Vec<Cell>>,
}

impl Grid {
    fn new(size: usize) -> Grid {
        let mut cells = vec![vec![Cell::Free; size]; size];
        for x in 0..=size - 1 {
            for y in 0..=size - 1 {
                if x == 0 || y == 0 || x == size - 1 || y == size - 1 {
                    cells[x][y] = Cell::Obstacle;
                };
            }
        }
        Grid { cells }
    }

    // TODO: document distance parameter
    fn random_empty_cell(&self, distance: usize) -> (usize, usize) {
        let size = self.cells.len();
        loop {
            let x = thread_rng().gen_range(distance + 1..size - distance);
            let y = thread_rng().gen_range(distance + 1..size - distance);
            if self.cells[x][y] == Cell::Free {
                break (x, y);
            }
        }
    }

    fn spawn_food(&mut self) {
        let (x, y) = self.random_empty_cell(1);
        self.cells[x][y] = Cell::Food;
    }

    fn spawn_obstacles(&mut self) {
        let size = self.cells.len();
        for _ in 0..=size / 2 {
            let (x, y) = self.random_empty_cell(0);
            self.cells[x][y] = Cell::Obstacle;
        }
    }

    fn spawn_snake(&mut self) -> Snake {
        let (x, y) = self.random_empty_cell(4);
        self.cells[x][y] = Cell::Snake;
        self.cells[x + 1][y] = Cell::Snake;
        Snake::new((x, y), (x + 1, y))
    }
}

pub struct Snake {
    body: VecDeque<(usize, usize)>,
}

impl Snake {
    fn new(tail: (usize, usize), head: (usize, usize)) -> Snake {
        let mut body = VecDeque::with_capacity(2);
        body.push_front(tail);
        body.push_front(head);
        Snake { body }
    }

    fn advance(&mut self, new_direction: Direction) {
        // only use new direction if it is doesn't change by 180 degrees
        let mut direction = self.direction();
        if new_direction != direction.opposite() {
            direction = new_direction;
        }

        let (x, y) = self.head();
        let (next_x, next_y) = next_cell(x, y, direction);
        self.body.push_front((next_x, next_y));
    }

    fn direction(&self) -> Direction {
        let (head_x, head_y) = self.head();
        let (next_x, next_y) = *self.body.get(1).unwrap();
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

    fn head(&self) -> (usize, usize) {
        *self.body.front().unwrap()
    }

    pub fn len(&self) -> usize {
        self.body.len()
    }

    fn remove_tail(&mut self) -> (usize, usize) {
        self.body.pop_back().unwrap()
    }
}

pub struct Game {
    pub grid: Grid,
    pub snake: Snake,
    pub end: bool,
    pub steps: u32,
}

impl Game {
    pub fn new(size: usize) -> Game {
        assert!(size >= 5, "Minimum grid size is 5!");

        let mut grid = Grid::new(size);
        let snake = grid.spawn_snake();
        grid.spawn_food();
        grid.spawn_obstacles();

        Game {
            grid,
            snake,
            end: false,
            steps: 0,
        }
    }

    pub fn step(&mut self, direction: Direction) {
        self.snake.advance(direction);

        let (x, y) = self.snake.head();
        match self.grid.cells[x][y] {
            Cell::Obstacle | Cell::Snake => {
                self.grid.cells[x][y] = Cell::Crash;
                self.end = true;
            }
            Cell::Food => {
                self.grid.cells[x][y] = Cell::Snake;
                self.grid.spawn_food();
            }
            Cell::Free => {
                self.grid.cells[x][y] = Cell::Snake;
                let (tail_x, tail_y) = self.snake.remove_tail();
                self.grid.cells[tail_x][tail_y] = Cell::Free;
            }
            Cell::Crash => unreachable!(),
        }

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
