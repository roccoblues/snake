use crate::game::{Direction, Grid, Point, Tile};
use rand::prelude::*;
use std::collections::HashMap;

#[derive(Default, Debug)]
struct PointInfo {
    parent: Option<Point>,
    // The movement cost to move from the starting point to this point on the grid,
    // following the path generated to get there.
    g: usize,
    // The estimated movement cost to move from this point on the grid to the final destination.
    // We currently use manhatten distance as an approximation heuristic.
    h: usize,
    // The search algorith picks the next point having the lowest 'f' and proceeds with that.
    f: usize,
}

type PointDetails = HashMap<Point, PointInfo>;

// Calculates a path from the start position to the food on the grid using the A* Search Algorithm.
// The result is a vector of directions. If no path can be found an empty vector is returned.
// --> https://www.geeksforgeeks.org/a-search-algorithm/
pub fn solve(grid: &Grid, start: Point) -> Vec<Direction> {
    // Find the food on the grid.
    let target = find_target(grid);

    // Use a hashmap to hold the details of a point.
    let mut point_details = HashMap::new();

    // Create the open list to hold potential points of the path.
    let mut open_list: Vec<Point> = Vec::with_capacity(grid.width() * grid.height());

    // Create a closed list to hold already checked points and initialize it to false
    // which means that no point has been included yet.
    // This closed list is implemented as a boolean 2D array.
    let mut closed_list = vec![vec![false; grid.height()]; grid.width()];

    // Put the starting point on the open list.
    open_list.push(start);
    point_details.insert(
        start,
        PointInfo {
            ..Default::default()
        },
    );

    while !open_list.is_empty() {
        // Pop the point with the lowest f value off the open list.
        let i = lowest_f(&open_list, &point_details);
        let p = open_list.swap_remove(i);
        let (x, y) = p;

        // Push it on the closed list.
        closed_list[x][y] = true;

        // Calculate all valid successors for that point.
        let successors = generate_successors(p, grid);
        for next in successors.into_iter() {
            // If the successor is already on the closed list, ignore it.
            let (next_x, next_y) = next;
            if closed_list[next_x][next_y] {
                continue;
            }

            // If successor is the target, stop and generate the path.
            if next == target {
                point_details.insert(
                    next,
                    PointInfo {
                        parent: Some(p),
                        ..Default::default()
                    },
                );
                return generate_path(next, &point_details);
            }

            // Compute g,h and f for the successor.
            let info = point_details.get(&p).unwrap();
            let g = info.g + 1;
            let h = manhatten_distance(next, target);
            let f = g + h as usize;

            match point_details.get_mut(&next) {
                // If a point with the same position as successor is in the open list.
                Some(next_info) => {
                    // And it has a lower f value than the successor, skip this successor.
                    if next_info.f < f {
                        continue;
                    }
                    // Otherwise, update the details of this point with the values of the successor.
                    next_info.g = g;
                    next_info.h = h;
                    next_info.f = f;
                    next_info.parent = Some(p);
                }
                // If not, add the point to the open list.
                None => {
                    open_list.push(next);
                    point_details.insert(
                        next,
                        PointInfo {
                            g,
                            h,
                            f,
                            parent: Some(p),
                        },
                    );
                }
            }
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

// Finds the food tile in the grid and returns its coordinates.
fn find_target(grid: &Grid) -> Point {
    for x in 0..grid.width() {
        for y in 0..grid.height() {
            let p = (x, y);
            if grid.tile(p) == Tile::Food {
                return p;
            }
        }
    }
    unreachable!("No food found in grid!")
}

// Finds the point with the lowest f value in the list and returns it position in the list.
fn lowest_f(list: &[Point], point_details: &PointDetails) -> usize {
    assert!(!list.is_empty());

    let mut f = usize::MAX;
    let mut i = 0;
    for (n, p) in list.iter().enumerate() {
        match point_details.get(p) {
            Some(p) => {
                if p.f < f {
                    f = p.f;
                    i = n;
                }
            }
            None => unreachable!(),
        }
    }
    i
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
fn generate_path(target: Point, point_details: &PointDetails) -> Vec<Direction> {
    let mut directions: Vec<Direction> = Vec::new();
    let mut p = &target;
    loop {
        match point_details.get(p) {
            Some(info) => match &info.parent {
                Some(parent) => {
                    let direction = get_direction(*parent, *p);
                    directions.push(direction);
                    p = parent;
                }
                None => break,
            },
            None => unreachable!(),
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

fn manhatten_distance(from: Point, to: Point) -> usize {
    let (from_x, from_y) = from;
    let (to_x, to_y) = to;
    let distance = i32::abs(from_x as i32 - to_x as i32) + i32::abs(from_y as i32 - to_y as i32);
    distance.try_into().unwrap()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn solve_path_simple() {
        let mut grid = Grid::new(5, 5);
        grid.set_tile((3, 1), Tile::Food);
        assert_eq!(solve(&grid, (1, 1)), vec![Direction::East, Direction::East])
    }

    #[test]
    fn solve_path_diagonal() {
        let mut grid = Grid::new(5, 5);
        grid.set_tile((3, 3), Tile::Food);
        assert_eq!(
            solve(&grid, (1, 1)),
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
            solve(&grid, (1, 1)),
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
            solve(&grid, (3, 3)),
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
