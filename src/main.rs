use std::sync::atomic::{self, AtomicU16};
use std::sync::mpsc;
use std::sync::Arc;

use game::{
    create_grid, next_point, random_direction, snake_direction, spawn_food, spawn_obstacles,
    spawn_snake,
};
use input::Input;
use output::Screen;
use types::{Direction, Grid, Snake, Tile};

mod cli;
mod game;
mod input;
mod output;
mod path;
mod types;

fn main() {
    let options = cli::parse_options();

    output::init();

    let mut grid_width = options.grid_width;
    let mut grid_height = options.grid_height;
    if options.fit_grid {
        (grid_width, grid_height) = output::max_grid_size();
    }

    let interval = Arc::new(AtomicU16::new(options.interval));

    let mut end = false;
    let mut paused = false;
    let mut steps = 0;
    let obstacle_count = grid_width * grid_height / 25;

    let mut grid = create_grid(grid_width.into(), grid_height.into());
    let mut snake = spawn_snake(&mut grid);
    let mut food = spawn_food(&mut grid);
    if !options.no_obstacles {
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
                    interval.store(options.interval, atomic::Ordering::Relaxed);
                    end = false;
                    paused = false;
                    steps = 0;
                    grid = create_grid(grid_width.into(), grid_height.into());
                    snake = spawn_snake(&mut grid);
                    food = spawn_food(&mut grid);
                    if !options.no_obstacles {
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
                if !options.arcade {
                    increase_interval(&interval);
                }
            }
            Input::IncreaseSpeed => {
                if !options.arcade {
                    decrease_interval(&interval);
                }
            }
            Input::Step => {
                if end || paused {
                    continue;
                }

                let head = snake.front().unwrap();

                // In autopilot mode calculate the path to the food as a list of directions.
                if options.autopilot {
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
                        if options.arcade {
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

    output::reset();
}

fn increase_interval(interval: &Arc<AtomicU16>) {
    let i = interval.load(atomic::Ordering::Relaxed);
    interval.store(i + 5, atomic::Ordering::Relaxed);
}

fn decrease_interval(interval: &Arc<AtomicU16>) {
    let i = interval.load(atomic::Ordering::Relaxed);
    if i - 5 > cli::MIN_INTERVAL as u16 {
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
