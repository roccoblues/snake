use crate::game::{generate_successors, next_point};
use crate::types::{Direction, Grid, Point, Tile};
use std::collections::HashSet;

// Calculates a path from the start position to the target on the grid using the A* Search Algorithm.
// The result is a vector of directions. If no path can be found an empty vector is returned.
//
// --> https://www.geeksforgeeks.org/a-search-algorithm/
// g: The movement cost to move from the starting point to this point on the grid,
//    following the path generated to get there.
// h: The estimated movement cost to move from this point on the grid to the final destination.
//    We currently use manhatten distance as an approximation heuristic.
// f: The search algorith picks the next point having the lowest 'f' and proceeds with that.
pub fn find(grid: &Grid, start: Point, target: Point) -> Vec<Direction> {
    let (start_x, start_y) = start;
    let grid_width = grid.len();
    let grid_height = grid[0].len();

    // Create a bunch of 2D arrays to hold the details of a point.
    let mut parents = vec![vec![None; grid_height]; grid_width];
    let mut g_list = vec![vec![0; grid_height]; grid_width];
    let mut f_list = vec![vec![i32::MAX; grid_height]; grid_width];

    // Create a closed list to hold already checked points.
    let mut closed = vec![vec![false; grid_height]; grid_width];

    // Create a open list to hold potential points of the path.
    let mut open = HashSet::new();

    // Put the starting point on the open list.
    open.insert(start);
    f_list[start_x][start_y] = 0;

    // Pop the point with the lowest f value off the open list.
    while let Some(p) = get_lowest_f(&mut open, &f_list) {
        let (x, y) = p;

        // Push it on the closed list.
        closed[x][y] = true;

        // Go through all successors for that point.
        for s in generate_successors(p, grid).iter() {
            let (s_x, s_y) = *s;

            // Skip blocked tiles.
            if blocked_tile(grid, *s) {
                continue;
            }

            // If the successor is already on the closed list, ignore it.
            if closed[s_x][s_y] {
                continue;
            }

            // If successor is the target, stop and generate the path.
            if *s == target {
                parents[s_x][s_y] = Some(p);
                return generate_path(*s, &parents);
            }

            // Compute g,h and f for the successor.
            let g = g_list[x][y] + 1;
            let h = manhatten_distance(*s, target);
            let f = g + h;

            // If the known f value is lower than what we currently have for the position.
            if f < f_list[s_x][s_y] {
                // Update the details of this position with the values of the successor.
                g_list[s_x][s_y] = g;
                f_list[s_x][s_y] = f;
                parents[s_x][s_y] = Some(p);

                // And push it on the open list.
                open.insert(*s);
            }
        }
    }

    // If we reach this point we couldn't find a clear path.
    // We fallback to to longest free straight path.
    best_straight_path(grid, start)
}

// Finds the point with the lowest f value in the list and returns it.
fn get_lowest_f(list: &mut HashSet<Point>, f_list: &[Vec<i32>]) -> Option<Point> {
    let mut lowest_f = i32::MAX;
    let mut res: Option<Point> = None;
    for (x, y) in list.iter() {
        let f = f_list[*x][*y];
        if f < lowest_f {
            lowest_f = f;
            res = Some((*x, *y));
        }
    }
    if let Some(p) = res {
        list.remove(&p);
    }
    res
}

// Generates the path from the starting point to the target as a vector of directions.
// The entries are in reverse order so that a pop() on the vector returns the next direction.
fn generate_path(target: Point, parents: &[Vec<Option<Point>>]) -> Vec<Direction> {
    let mut directions: Vec<Direction> = Vec::new();
    let mut p = target;
    loop {
        let (x, y) = p;
        match parents[x][y] {
            Some(parent) => {
                let direction = get_direction(parent, p);
                directions.push(direction);
                p = parent;
            }
            None => break,
        }
    }
    directions
}

fn best_straight_path(grid: &Grid, start: Point) -> Vec<Direction> {
    let mut direction = None;
    let mut count = 0;
    for p in generate_successors(start, grid) {
        let d = get_direction(start, p);
        let mut n = p;
        let mut c = 0;
        while !blocked_tile(grid, n) {
            c += 1;
            n = next_point(n, d);
        }
        if c > count {
            count = c;
            direction = Some(d);
        }
    }
    match direction {
        Some(d) => vec![d],
        None => Vec::new(),
    }
}

fn get_direction(from: Point, to: Point) -> Direction {
    let (from_x, from_y) = from;
    let (to_x, to_y) = to;
    if to_x > from_x {
        Direction::East
    } else if to_x < from_x {
        Direction::West
    } else if to_y > from_y {
        Direction::South
    } else {
        Direction::North
    }
}

fn manhatten_distance(from: Point, to: Point) -> i32 {
    let (from_x, from_y) = from;
    let (to_x, to_y) = to;
    let dx = (from_x as i32 - to_x as i32).abs();
    let dy = (from_y as i32 - to_y as i32).abs();
    dx + dy
}

fn blocked_tile(grid: &Grid, p: Point) -> bool {
    let (x, y) = p;
    grid[x][y] != Tile::Free && grid[x][y] != Tile::Food
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn solve_path_simple() {
        let mut grid = vec![vec![Tile::Free; 3]; 3];
        grid[2][0] = Tile::Food;
        assert_eq!(
            find(&grid, (0, 0), (2, 0)),
            vec![Direction::East, Direction::East]
        )
    }

    #[test]
    fn solve_path_with_obstacle() {
        let mut grid = vec![vec![Tile::Free; 3]; 3];
        grid[1][0] = Tile::Obstacle;
        grid[1][1] = Tile::Obstacle;
        grid[2][0] = Tile::Food;
        assert_eq!(
            find(&grid, (0, 0), (2, 0)),
            vec![
                Direction::North,
                Direction::North,
                Direction::East,
                Direction::East,
                Direction::South,
                Direction::South,
            ]
        )
    }

    #[test]
    fn solve_path_with_obstacle_reverse() {
        let mut grid = vec![vec![Tile::Free; 3]; 3];
        grid[1][1] = Tile::Obstacle;
        grid[1][2] = Tile::Obstacle;
        grid[0][2] = Tile::Food;
        assert_eq!(
            find(&grid, (2, 2), (0, 2)),
            vec![
                Direction::South,
                Direction::South,
                Direction::West,
                Direction::West,
                Direction::North,
                Direction::North,
            ]
        )
    }

    #[test]
    fn best_straight_path_none() {
        let mut grid = vec![vec![Tile::Free; 3]; 3];
        grid[0][1] = Tile::Obstacle;
        grid[1][0] = Tile::Obstacle;
        assert_eq!(best_straight_path(&grid, (0, 0)), vec![])
    }

    #[test]
    fn best_straight_path_east() {
        let mut grid = vec![vec![Tile::Free; 9]; 9];
        grid[4][3] = Tile::Obstacle;
        grid[6][4] = Tile::Obstacle;
        grid[4][7] = Tile::Obstacle;
        grid[0][4] = Tile::Obstacle;
        assert_eq!(best_straight_path(&grid, (4, 4)), vec![Direction::West])
    }
}
