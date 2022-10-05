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

type Cells = Vec<Vec<Cell>>;
type Snake = VecDeque<(u16, u16)>;

pub struct Game {
    pub cells: Cells,
    pub snake: Snake,
    direction: Direction,
    pub end: bool,
    pub steps: u32,
}

impl Game {
    pub fn new(size: u16) -> Game {
        let mut cells = create_cells(size);
        spawn_food(&mut cells);
        spawn_obstacles(&mut cells);
        let snake = spawn_snake(&mut cells);

        Game {
            cells,
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
        match self.cells[x as usize][y as usize] {
            Cell::Obstacle | Cell::Snake => {
                self.cells[x as usize][y as usize] = Cell::Crash;
                self.end = true;
                return;
            }
            Cell::Food => spawn_food(&mut self.cells),
            Cell::Free => {
                // remove last snake cell to "move" the snake
                let (tail_x, tail_y) = self.snake.pop_back().unwrap();
                self.cells[tail_x as usize][tail_y as usize] = Cell::Free;
            }
            Cell::Crash => unreachable!(),
        }

        // grow snake
        self.cells[x as usize][y as usize] = Cell::Snake;
        self.snake.push_front((x, y));

        self.steps += 1;
    }

    pub fn set_direction(&mut self, direction: Direction) {
        if self.direction.opposite() != direction {
            self.direction = direction;
        }
    }
}

fn next_cell(x: u16, y: u16, direction: Direction) -> (u16, u16) {
    match direction {
        Direction::Up => (x, y - 1),
        Direction::Down => (x, y + 1),
        Direction::Left => (x - 1, y),
        Direction::Right => (x + 1, y),
    }
}

// TODO: document distance parameter
fn random_empty_cell(cells: &Cells, distance: u16) -> (u16, u16) {
    let size = cells.len() as u16;
    loop {
        let x = thread_rng().gen_range(distance + 1..size - distance);
        let y = thread_rng().gen_range(distance + 1..size - distance);
        if cells[x as usize][y as usize] == Cell::Free {
            break (x, y);
        }
    }
}

fn random_direction() -> Direction {
    Direction::from_int(thread_rng().gen_range(0..=3) as u8).unwrap()
}

fn spawn_food(cells: &mut Cells) {
    let (x, y) = random_empty_cell(cells, 1);
    cells[x as usize][y as usize] = Cell::Food;
}

fn spawn_obstacles(cells: &mut Cells) {
    let size = cells.len() as u16;
    for _ in 0..=size / 2 {
        let (x, y) = random_empty_cell(cells, 0);
        cells[x as usize][y as usize] = Cell::Obstacle;
    }
}

fn spawn_snake(cells: &mut Cells) -> Snake {
    let (x, y) = random_empty_cell(cells, 3);
    cells[x as usize][y as usize] = Cell::Snake;
    let mut snake = VecDeque::new();
    snake.push_front((x, y));
    snake
}

fn create_cells(size: u16) -> Cells {
    let mut cells = vec![vec![Cell::Free; size.into()]; size.into()];
    for x in 0..=size - 1 {
        for y in 0..=size - 1 {
            if x == 0 || y == 0 || x == size - 1 || y == size - 1 {
                cells[x as usize][y as usize] = Cell::Obstacle;
            };
        }
    }
    cells
}
