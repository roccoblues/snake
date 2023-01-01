use crate::types::{Direction, Grid, Point, Snake, Tile};
use int_enum::IntEnum;
use rand::prelude::*;
use std::collections::VecDeque;

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

pub fn random_direction() -> Direction {
    Direction::from_int(thread_rng().gen_range(0..=3) as u8).unwrap()
}

pub fn snake_direction(snake: &Snake) -> Direction {
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
