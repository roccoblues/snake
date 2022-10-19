use crate::game::{Direction, Grid, Point, Tile};
use rand::prelude::*;

// Calculates a path from the start position to the food on the grid using the A* Search Algorithm.
// The result is a vector of directions. If no path can be found an empty vector is returned.
//
// --> https://www.geeksforgeeks.org/a-search-algorithm/
// g: The movement cost to move from the starting point to this point on the grid,
//    following the path generated to get there.
// h: The estimated movement cost to move from this point on the grid to the final destination.
//    We currently use manhatten distance as an approximation heuristic.
// f: The search algorith picks the next point having the lowest 'f' and proceeds with that.
pub fn solve(grid: &Grid, start: Point, target: Point) -> Vec<Direction> {
    // Create a bunch of 2D arrays to hold the details of a point.
    let mut parent_list = vec![vec![None; grid.height()]; grid.width()];
    let mut g_list = vec![vec![0; grid.height()]; grid.width()];
    let mut h_list = vec![vec![0; grid.height()]; grid.width()];
    let mut f_list = vec![vec![i32::MAX; grid.height()]; grid.width()];

    // Create a closed list to hold already checked points and initialize it to false
    // which means that no point has been included yet.
    let mut closed_list = vec![vec![false; grid.height()]; grid.width()];

    // Create a open list to hold potential points of the path.
    let mut open_list: Vec<Point> = Vec::with_capacity(grid.width() * grid.height());

    // Put the starting point on the open list.
    open_list.push(start);

    // Pop the point with the lowest f value off the open list.
    while let Some(p) = get_lowest_f(&mut open_list, &f_list) {
        let (x, y) = p;

        // Push it on the closed list.
        closed_list[x][y] = true;

        // Calculate all valid successors for that point.
        let successors = generate_successors(p, grid);
        for s in successors.into_iter() {
            // If the successor is already on the closed list, ignore it.
            let (s_x, s_y) = s;
            if closed_list[s_x][s_y] {
                continue;
            }

            // If successor is the target, stop and generate the path.
            if s == target {
                parent_list[s_x][s_y] = Some(p);
                return generate_path(s, &parent_list);
            }

            // Compute g,h and f for the successor.
            let g = g_list[x][y] + 1;
            let h = manhatten_distance(s, target);
            let f = g + h;

            // If we have seen the same position with a lower f value, skip this successor.
            if f_list[s_x][s_y] < f {
                continue;
            }

            // Otherwise, update the details of this point with the values of the successor.
            g_list[s_x][s_y] = g;
            h_list[s_x][s_y] = h;
            f_list[s_x][s_y] = f;
            parent_list[s_x][s_y] = Some(p);

            // And push it on the open list.
            open_list.push(s);
        }
    }
    // If we reach this point we couldn't find a clear path.

    // If we have valid successors we simply pick a random one.
    let successors = generate_successors(start, grid);
    if !successors.is_empty() {
        let next = successors[thread_rng().gen_range(0..successors.len()) as usize];
        return vec![get_direction(start, next)];
    }

    // No valid successors left, brace for impact!
    vec![]
}

// Finds the point with the lowest f value in the list and returns it.
fn get_lowest_f(list: &mut Vec<Point>, f_list: &[Vec<i32>]) -> Option<Point> {
    if list.is_empty() {
        return None;
    }

    let mut lowest_f = i32::MAX;
    let mut i = 0;
    for (n, (x, y)) in list.iter().enumerate() {
        let f = f_list[*x][*y];
        if f < lowest_f {
            lowest_f = f;
            i = n;
        }
    }
    Some(list.swap_remove(i))
}

// Generates all valid successors of a point.
//           N
//           |
//      W--Point--E
//           |
//           S
// Successors are excluded if they are outside the grid or if the tiles are not Free or Food.
fn generate_successors(p: Point, grid: &Grid) -> Vec<Point> {
    let mut result: Vec<Point> = Vec::with_capacity(4);
    let (x, y) = p;

    // north
    if x > 0 {
        result.push((x - 1, y));
    }
    // south
    if x + 1 < grid.width() {
        result.push((x + 1, y));
    }
    // east
    if y + 1 < grid.height() {
        result.push((x, y + 1));
    }
    // west
    if y > 0 {
        result.push((x, y - 1))
    }

    result
        .into_iter()
        .filter(|p| grid.tile(*p) == Tile::Free || grid.tile(*p) == Tile::Food)
        .collect()
}

// Generates the path from the starting point to the target as a vector of directions.
// The entries are in reverse order so that a pop() on the vector returns the next direction.
fn generate_path(target: Point, parent_list: &[Vec<Option<Point>>]) -> Vec<Direction> {
    let mut directions: Vec<Direction> = Vec::new();
    let mut p = target;
    loop {
        let (x, y) = p;
        match parent_list[x][y] {
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn solve_path_simple() {
        let mut grid = Grid::new(5, 5);
        grid.set_tile((3, 1), Tile::Food);
        assert_eq!(
            solve(&grid, (1, 1), (3, 1)),
            vec![Direction::East, Direction::East]
        )
    }

    #[test]
    fn solve_path_diagonal() {
        let mut grid = Grid::new(5, 5);
        grid.set_tile((3, 3), Tile::Food);
        assert_eq!(
            solve(&grid, (1, 1), (3, 3)),
            vec![
                Direction::East,
                Direction::East,
                Direction::South,
                Direction::South,
            ]
        )
    }

    #[test]
    fn solve_path_with_obstacle() {
        let mut grid = Grid::new(5, 5);
        grid.set_tile((2, 1), Tile::Obstacle);
        grid.set_tile((2, 2), Tile::Obstacle);
        grid.set_tile((3, 1), Tile::Food);
        assert_eq!(
            solve(&grid, (1, 1), (3, 1)),
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
        let mut grid = Grid::new(5, 5);
        grid.set_tile((2, 2), Tile::Obstacle);
        grid.set_tile((2, 3), Tile::Obstacle);
        grid.set_tile((1, 3), Tile::Food);
        assert_eq!(
            solve(&grid, (3, 3), (1, 3)),
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
}
