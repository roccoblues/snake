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

type Tiles = Vec<Vec<Tile>>;
type Snake = VecDeque<(u16, u16)>;

pub struct Game {
    pub tiles: Tiles,
    pub snake: Snake,
    direction: Direction,
    pub end: bool,
    pub steps: u32,
}

impl Game {
    pub fn new(size: u16) -> Game {
        let mut tiles = create_tiles(size);
        spawn_food(&mut tiles);
        spawn_obstacles(&mut tiles);
        let snake = spawn_snake(&mut tiles);

        Game {
            tiles,
            snake,
            direction: random_direction(),
            end: false,
            steps: 0,
        }
    }

    pub fn step(&mut self) {
        let (head_x, head_y) = *self.snake.front().unwrap();

        // tile in front of the snake
        let (x, y) = next_point(head_x, head_y, self.direction);
        match self.tiles[x as usize][y as usize] {
            Tile::Obstacle | Tile::Snake => {
                self.tiles[x as usize][y as usize] = Tile::Crash;
                self.end = true;
                return;
            }
            Tile::Food => spawn_food(&mut self.tiles),
            Tile::Free => {
                // remove last snake tile to "move" the snake
                let (tail_x, tail_y) = self.snake.pop_back().unwrap();
                self.tiles[tail_x as usize][tail_y as usize] = Tile::Free;
            }
            Tile::Crash => unreachable!(),
        }

        // grow snake
        self.tiles[x as usize][y as usize] = Tile::Snake;
        self.snake.push_front((x, y));

        self.steps += 1;
    }

    pub fn set_direction(&mut self, direction: Direction) {
        if self.direction.opposite() != direction {
            self.direction = direction;
        }
    }
}

fn next_point(x: u16, y: u16, direction: Direction) -> (u16, u16) {
    match direction {
        Direction::Up => (x, y - 1),
        Direction::Down => (x, y + 1),
        Direction::Left => (x - 1, y),
        Direction::Right => (x + 1, y),
    }
}

// TODO: document distance parameter
fn random_empty_point(tiles: &Tiles, distance: u16) -> (u16, u16) {
    let size = tiles.len() as u16;
    loop {
        let x = thread_rng().gen_range(distance + 1..size - distance);
        let y = thread_rng().gen_range(distance + 1..size - distance);
        if tiles[x as usize][y as usize] == Tile::Free {
            break (x, y);
        }
    }
}

fn random_direction() -> Direction {
    Direction::from_int(thread_rng().gen_range(0..=3) as u8).unwrap()
}

fn spawn_food(tiles: &mut Tiles) {
    let (x, y) = random_empty_point(tiles, 1);
    tiles[x as usize][y as usize] = Tile::Food;
}

fn spawn_obstacles(tiles: &mut Tiles) {
    let size = tiles.len() as u16;
    for _ in 0..=size / 2 {
        let (x, y) = random_empty_point(tiles, 0);
        tiles[x as usize][y as usize] = Tile::Obstacle;
    }
}

fn spawn_snake(tiles: &mut Tiles) -> Snake {
    let (x, y) = random_empty_point(tiles, 3);
    tiles[x as usize][y as usize] = Tile::Snake;
    let mut snake = VecDeque::new();
    snake.push_front((x, y));
    snake
}

fn create_tiles(size: u16) -> Tiles {
    let mut tiles = vec![vec![Tile::Free; size.into()]; size.into()];
    for x in 0..=size - 1 {
        for y in 0..=size - 1 {
            if x == 0 || y == 0 || x == size - 1 || y == size - 1 {
                tiles[x as usize][y as usize] = Tile::Obstacle;
            };
        }
    }
    tiles
}
