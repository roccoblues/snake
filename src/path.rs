use crate::game::{Cell, Direction, Grid};
use log::debug;
use rand::prelude::*;
use std::collections::HashMap;

#[derive(Debug, Clone, Copy, PartialEq, Hash, Eq)]
struct Point {
    x: usize,
    y: usize,
}

#[derive(Default, Debug)]
struct CellInfo {
    parent: Option<Point>,
    g: usize, // the movement cost to move from the starting point to a given square on the grid, following the path generated to get there.
    h: usize, // the estimated movement cost to move from that given square on the grid to the final destination
    f: usize, // g + h
}

type CellDetails = HashMap<Point, CellInfo>;

// Calculates path as a vector of directions using the A* Search Algorithm.
// https://www.geeksforgeeks.org/a-search-algorithm/
pub fn solve(grid: &Grid, (start_x, start_y): (usize, usize)) -> Vec<Direction> {
    let start = Point {
        x: start_x,
        y: start_y,
    };

    let target = find_target(grid);

    // Use a hashmap to hold the details of a cell.
    let mut cell_details = HashMap::new();

    // Create an open list.
    let mut open_list: Vec<Point> = Vec::with_capacity(grid.len() * grid.len());

    // Create a closed list and initialise it to false which means that no cell
    // has been included yet. This closed list is implemented as a boolean 2D array.
    let mut closed_list = vec![vec![false; grid.len()]; grid.len()];

    // put the starting node on the open list
    open_list.push(start);

    // and record it details
    cell_details.insert(
        start,
        CellInfo {
            ..Default::default()
        },
    );

    while !open_list.is_empty() {
        debug!("open_list.len(): {}", open_list.len());

        // pop the node with the lowest f on the open list
        let i = lowest_f(&open_list, &cell_details);
        let p = open_list.swap_remove(i);

        // push it on the closed list
        closed_list[p.x][p.y] = true;

        let successors = generate_successors(&p, grid);
        for next in successors.into_iter() {
            // if the successor is already on the closed list we ignore it
            if closed_list[next.x][next.y] {
                continue;
            }

            // if successor is the target, stop search
            if next.x == target.x && next.y == target.y {
                cell_details.insert(
                    next,
                    CellInfo {
                        parent: Some(p),
                        ..Default::default()
                    },
                );
                return generate_path(&next, &cell_details);
            }

            // compute g,h and f for successor
            let info = cell_details.get(&p).unwrap();
            let g = info.g + 1;
            let h = manhatten_distance(next, target);
            let f = g + h as usize;

            // if a node with the same position as successor is in the open list
            match cell_details.get_mut(&next) {
                Some(next_info) => {
                    // which has a lower f than successor, skip this successor
                    if next_info.f < f {
                        continue;
                    }
                    // update the details of this cell
                    next_info.f = f;
                    next_info.g = g;
                    next_info.h = h;
                    next_info.parent = Some(p);
                }
                None => {
                    // otherwise, add the node to the open list
                    open_list.push(next);
                    // record the details of this cell
                    cell_details.insert(
                        next,
                        CellInfo {
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

    // We didn't find a clear path.
    // If we have clear successors we pick a random one.
    let successors = generate_successors(&start, grid);
    if successors.len() > 0 {
        let next = successors[thread_rng().gen_range(0..=successors.len() - 1) as usize];
        return vec![get_direction(&start, &next)];
    }

    // Brace for impact!
    vec![]
}

fn find_target(grid: &Grid) -> Point {
    for (x, row) in grid.iter().enumerate() {
        for (y, cell) in row.iter().enumerate() {
            if *cell == Cell::Food {
                return Point { x, y };
            }
        }
    }
    unreachable!("No food found in grid!")
}

fn lowest_f(list: &[Point], cell_details: &CellDetails) -> usize {
    assert!(list.len() > 0);

    let mut f = usize::MAX;
    let mut i = 0;
    for (n, p) in list.iter().enumerate() {
        match cell_details.get(&p) {
            Some(info) => {
                if info.f < f {
                    f = info.f;
                    i = n;
                }
            }
            None => unreachable!(),
        }
    }
    i
}

// Generate all the 4 successor of this cell
//           N
//           |
//      W--Cell--E
//           |
//           S
// Cell-->Popped Cell (x,y)
//
// N --> North  (x-1, y  )
// S --> South  (x+1, y  )
// E --> East   (x,   y+1)
// W --> West   (x,   y-1)
fn generate_successors(p: &Point, grid: &Grid) -> Vec<Point> {
    let mut result: Vec<Point> = Vec::with_capacity(4);

    // north
    if p.x > 0 {
        result.push(Point { x: p.x - 1, ..*p });
    }
    // south
    if p.x + 1 < grid.len() {
        result.push(Point { x: p.x + 1, ..*p });
    }
    // east
    if p.y + 1 < grid.len() {
        result.push(Point { y: p.y + 1, ..*p });
    }
    // west
    if p.y > 0 {
        result.push(Point { y: p.y - 1, ..*p })
    }

    result
        .into_iter()
        .filter(|p| grid[p.x][p.y] == Cell::Free || grid[p.x][p.y] == Cell::Food)
        .collect()
}

fn generate_path(start: &Point, cell_details: &CellDetails) -> Vec<Direction> {
    let mut directions: Vec<Direction> = Vec::new();
    let mut p = start;
    loop {
        match cell_details.get(p) {
            Some(info) => match &info.parent {
                Some(parent) => {
                    let direction = get_direction(parent, p);
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

fn get_direction(from: &Point, to: &Point) -> Direction {
    if to.x > from.x {
        Direction::East
    } else if to.x < from.x {
        Direction::West
    } else if to.y > from.y {
        Direction::South
    } else {
        Direction::North
    }
}

fn manhatten_distance(start: Point, target: Point) -> usize {
    let distance =
        i32::abs(start.x as i32 - target.x as i32) + i32::abs(start.y as i32 - target.y as i32);
    distance.try_into().unwrap()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn solve_path_simple() {
        let mut grid: Vec<Vec<Cell>> = vec![vec![Cell::Free; 3]; 3];
        grid[2][0] = Cell::Food;
        assert_eq!(solve(&grid, (0, 0)), vec![Direction::East, Direction::East])
    }

    #[test]
    fn solve_path_diagonal() {
        let mut grid: Vec<Vec<Cell>> = vec![vec![Cell::Free; 3]; 3];
        grid[2][2] = Cell::Food;
        assert_eq!(
            solve(&grid, (0, 0)),
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
        let mut grid: Vec<Vec<Cell>> = vec![vec![Cell::Free; 3]; 3];
        grid[1][0] = Cell::Obstacle;
        grid[1][1] = Cell::Obstacle;
        grid[2][0] = Cell::Food;
        assert_eq!(
            solve(&grid, (0, 0)),
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
        let mut grid: Vec<Vec<Cell>> = vec![vec![Cell::Free; 3]; 3];
        grid[1][1] = Cell::Obstacle;
        grid[1][2] = Cell::Obstacle;
        grid[0][2] = Cell::Food;
        assert_eq!(
            solve(&grid, (2, 2)),
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
