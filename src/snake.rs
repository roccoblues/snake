use int_enum::IntEnum;
use rand::prelude::*;
use std::collections::VecDeque;
use std::sync::atomic::{self, AtomicU16};
use std::sync::mpsc;
use std::sync::Arc;

use crate::input::{self, Input};
use crate::output::{self, Screen};
use crate::path;
use crate::types::{Direction, Grid, Point, Snake, Tile};

pub const MIN_INTERVAL: i64 = 30;

pub struct Config {
    pub autopilot: bool,
    pub arcade: bool,
    pub grid_width: u16,
    pub grid_height: u16,
    pub fit_grid: bool,
    pub no_obstacles: bool,
    pub interval: u16,
}

pub fn run(config: &Config) {
    let mut grid_width = config.grid_width;
    let mut grid_height = config.grid_height;
    if config.fit_grid {
        (grid_width, grid_height) = output::max_grid_size();
    }

    let interval = Arc::new(AtomicU16::new(config.interval));

    let mut end = false;
    let mut paused = false;
    let mut steps = 0;
    let obstacle_count = grid_width * grid_height / 25;

    let mut grid = create_grid(grid_width.into(), grid_height.into());
    let mut snake = spawn_snake(&mut grid);
    let mut food = spawn_food(&mut grid);
    if !config.no_obstacles {
        spawn_obstacles(&mut grid, obstacle_count);
    }

    let mut screen = Screen::new(grid_width, grid_height);
    draw_grid(&screen, &grid);
    draw_steps(&screen, steps);
    draw_snake_len(&screen, &snake);

    let (tx, rx) = mpsc::channel();

    // Spawn thread to handle ui input.
    input::handle(tx.clone());

    // Spawn thread to send ticks.
    input::send_ticks(tx, Arc::clone(&interval));

    let mut direction = random_direction();
    let mut path: Vec<Direction> = Vec::new();

    loop {
        match rx.recv().unwrap() {
            Input::Unknown => {}
            Input::Exit => break,
            Input::ChangeDirection(d) => {
                // The snake can't reverse direction. So if the new direction is the opposite
                // of the current one we discard it.
                let current_direction = snake_direction(&snake);
                if d != current_direction.opposite() {
                    direction = d;
                }
            }
            Input::Pause => {
                if end {
                    // restart game
                    interval.store(config.interval, atomic::Ordering::Relaxed);
                    end = false;
                    paused = false;
                    steps = 0;
                    grid = create_grid(grid_width.into(), grid_height.into());
                    snake = spawn_snake(&mut grid);
                    food = spawn_food(&mut grid);
                    if !config.no_obstacles {
                        spawn_obstacles(&mut grid, obstacle_count);
                    }
                    screen = Screen::new(grid_width, grid_height);
                    draw_grid(&screen, &grid);
                    draw_steps(&screen, steps);
                    draw_snake_len(&screen, &snake);
                    direction = random_direction();
                    path = Vec::new();
                    continue;
                }
                // pause / resume
                paused ^= true
            }
            Input::DecreaseSpeed => {
                if !config.arcade {
                    increase_interval(&interval);
                }
            }
            Input::IncreaseSpeed => {
                if !config.arcade {
                    decrease_interval(&interval);
                }
            }
            Input::Step => {
                if end || paused {
                    continue;
                }

                let head = snake.front().unwrap();

                // In autopilot mode calculate the path to the food as a list of directions.
                if config.autopilot {
                    if path.is_empty() {
                        path = path::find(&grid, *head, food);
                    }
                    // Pop the next direction from the path.
                    // If it is empty (no path found), continue in the current
                    // direction and try again after the next step.
                    direction = path.pop().unwrap_or(direction);
                }

                // Return point in front of the snake in the given direction.
                let p = next_point(*head, direction);
                let (x, y) = p;

                // Check tile in the grid.
                match grid[x][y] {
                    // The snake crashed - end the game.
                    Tile::Obstacle | Tile::Snake => {
                        grid[x][y] = Tile::Crash;
                        screen.draw_tile(p, Tile::Crash);
                        end = true;
                    }
                    // The snake ate - spawn new food.
                    Tile::Food => {
                        snake.push_front(p);
                        grid[x][y] = Tile::Snake;
                        screen.draw_tile(p, Tile::Snake);
                        food = spawn_food(&mut grid);
                        screen.draw_tile(food, Tile::Food);
                        draw_steps(&screen, steps);
                        // In arcade mode we decrease the tick interval with every food eaten
                        // to make the game faster.
                        if config.arcade {
                            decrease_interval(&interval);
                        }
                    }
                    // If the tile is free we pop the tail of the snake to make it look like it is moving.
                    Tile::Free => {
                        snake.push_front(p);
                        grid[x][y] = Tile::Snake;
                        screen.draw_tile(p, Tile::Snake);
                        let tail = snake.pop_back().unwrap();
                        let (tail_x, tail_y) = tail;
                        grid[tail_x][tail_y] = Tile::Free;
                        screen.draw_tile(tail, Tile::Free);
                    }
                    Tile::Crash => unreachable!(),
                }

                steps += 1;
                draw_steps(&screen, steps);
            }
        }
    }
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

// Generates all valid successors of a point.
//           N
//           |
//      W--Point--E
//           |
//           S
pub fn generate_successors(p: Point, grid: &Grid) -> Vec<Point> {
    let mut successors: Vec<Point> = Vec::with_capacity(4);
    let (x, y) = p;

    if x > 0 {
        successors.push(next_point(p, Direction::West));
    }
    if x + 1 < grid.len() {
        successors.push(next_point(p, Direction::East));
    }
    if y + 1 < grid[0].len() {
        successors.push(next_point(p, Direction::South));
    }
    if y > 0 {
        successors.push(next_point(p, Direction::North))
    }

    successors
}

fn increase_interval(interval: &Arc<AtomicU16>) {
    let i = interval.load(atomic::Ordering::Relaxed);
    interval.store(i + 5, atomic::Ordering::Relaxed);
}

fn decrease_interval(interval: &Arc<AtomicU16>) {
    let i = interval.load(atomic::Ordering::Relaxed);
    if i - 5 > MIN_INTERVAL as u16 {
        interval.store(i - 5, atomic::Ordering::Relaxed);
    }
}

fn draw_grid(screen: &Screen, grid: &Grid) {
    for x in 0..grid.len() {
        for y in 0..grid[0].len() {
            screen.draw_tile((x, y), grid[x][y])
        }
    }
}

fn draw_steps(screen: &Screen, steps: u16) {
    screen.draw_text_left(format!("Steps: {}", steps));
}

fn draw_snake_len(screen: &Screen, snake: &Snake) {
    screen.draw_text_right(format!("Snake length: {}", snake.len()));
}

fn create_grid(width: usize, height: usize) -> Grid {
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

fn spawn_snake(grid: &mut Grid) -> Snake {
    let (x, y) = random_empty_point(grid, 4);
    grid[x][y] = Tile::Snake;
    let mut snake = VecDeque::with_capacity(10);
    snake.push_front((x, y));
    snake.push_front(next_point((x, y), random_direction()));
    snake
}

fn spawn_food(grid: &mut Grid) -> Point {
    let (x, y) = random_empty_point(grid, 1);
    grid[x][y] = Tile::Food;
    (x, y)
}

fn spawn_obstacles(grid: &mut Grid, count: u16) {
    for _ in 0..=count {
        // avoid creating dead ends
        'outer: loop {
            let p = random_empty_point(grid, 0);
            let (x, y) = p;
            grid[x][y] = Tile::Obstacle;
            for (a, b) in generate_successors(p, grid) {
                if grid[a][b] == Tile::Free && is_in_dead_end(grid, (a, b)) {
                    grid[x][y] = Tile::Free;
                    continue 'outer;
                }
            }
            break 'outer;
        }
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

// Checks if point is in this shape: #p#
//                                    #
fn is_in_dead_end(grid: &Grid, p: Point) -> bool {
    let mut free = 0;
    for (x, y) in generate_successors(p, grid) {
        if grid[x][y] == Tile::Free {
            free += 1;
        }
    }

    free < 2
}

fn random_direction() -> Direction {
    Direction::from_int(thread_rng().gen_range(0..=3) as u8).unwrap()
}

fn snake_direction(snake: &Snake) -> Direction {
    let (x, y) = snake.front().unwrap();
    let (i, j) = snake.get(1).unwrap();
    if x > i {
        Direction::East
    } else if x < i {
        Direction::West
    } else if y > j {
        Direction::South
    } else {
        Direction::North
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn is_dead_end_empty() {
        let grid = vec![vec![Tile::Free; 3]; 3];
        assert!(!is_in_dead_end(&grid, (0, 0)));
        assert!(!is_in_dead_end(&grid, (1, 1)));
    }

    #[test]
    fn is_dead_end_with_obstacle() {
        let mut grid = vec![vec![Tile::Free; 3]; 3];

        // obstacle
        grid[0][0] = Tile::Obstacle;
        grid[2][0] = Tile::Obstacle;

        // true
        assert!(is_in_dead_end(&grid, (1, 0)));

        // false
        assert!(!is_in_dead_end(&grid, (0, 1)));
        assert!(!is_in_dead_end(&grid, (0, 2)));
        assert!(!is_in_dead_end(&grid, (1, 1)));
        assert!(!is_in_dead_end(&grid, (1, 2)));
        assert!(!is_in_dead_end(&grid, (2, 1)));
        assert!(!is_in_dead_end(&grid, (2, 2)));
    }

    #[test]
    fn is_dead_end_with_obstacle_and_border() {
        let mut grid = vec![vec![Tile::Free; 4]; 4];

        // border
        grid[0][0] = Tile::Obstacle;
        grid[1][0] = Tile::Obstacle;
        grid[2][0] = Tile::Obstacle;
        grid[3][0] = Tile::Obstacle;
        grid[0][3] = Tile::Obstacle;
        grid[1][3] = Tile::Obstacle;
        grid[2][3] = Tile::Obstacle;
        grid[3][3] = Tile::Obstacle;
        grid[0][1] = Tile::Obstacle;
        grid[0][2] = Tile::Obstacle;
        grid[3][1] = Tile::Obstacle;
        grid[3][2] = Tile::Obstacle;
        // obstacle
        grid[1][1] = Tile::Obstacle;
        grid[3][1] = Tile::Obstacle;

        // true
        assert!(is_in_dead_end(&grid, (2, 1)));

        // false
        assert!(!is_in_dead_end(&grid, (2, 2)));
    }
}
