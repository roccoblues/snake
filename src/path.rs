use crate::game::{Cell, Direction, Grid};
use log::{debug, error, info, log_enabled, Level};
use rand::prelude::*;

#[derive(Default, Clone, Debug)]
struct CellInfo {
    parent_x: isize,
    parent_y: isize,
    g: usize,
    f: usize,
}

// Calculates path as a vector of directions using the A* Search Algorithm.
// https://www.geeksforgeeks.org/a-search-algorithm/
pub fn solve(grid: &Grid, (start_x, start_y): (usize, usize)) -> Vec<Direction> {
    let (target_x, target_y) = find_target(grid);

    // Declare a 2D array of structure to hold the details of a cell.
    let mut cell_details: Vec<Vec<CellInfo>> = vec![
        vec![
            CellInfo {
                f: usize::MAX,
                ..Default::default()
            };
            grid.len()
        ];
        grid.len()
    ];
    for row in cell_details.iter_mut() {
        for cell in row.iter_mut() {
            cell.parent_x = -1;
            cell.parent_y = -1;
        }
    }

    // Create an open list.
    let mut open_list: Vec<(usize, usize)> = Vec::with_capacity(grid.len() * grid.len());

    // Create a closed list and initialise it to false which means that no cell
    // has been included yet. This closed list is implemented as a boolean 2D array.
    let mut closed_list = vec![vec![false; grid.len()]; grid.len()];

    // put the starting node on the open list
    open_list.push((start_x, start_y));

    while !open_list.is_empty() {
        debug!("open_list.len(): {}", open_list.len());

        // pop the node with the lowest f on the open list
        let i = lowest_f(&open_list, &cell_details);
        let (x, y) = open_list.swap_remove(i);

        // push it on the closed list
        closed_list[x][y] = true;

        let successors = generate_successors(x, y, grid);
        for (next_x, next_y) in successors.into_iter() {
            // if successor is the target, stop search
            if next_x == target_x && next_y == target_y {
                cell_details[next_x][next_y].parent_x = x as isize;
                cell_details[next_x][next_y].parent_y = y as isize;
                return generate_path(next_x, next_y, &cell_details);
            }

            // if the successor is already on the closed list we ignore it
            if closed_list[next_x][next_y] {
                continue;
            }

            // compute g,h and f for successor
            let g = cell_details[x][y].g + 1;
            let h = manhatten_distance(next_x, next_y, target_x, target_y);
            let f = g + h as usize;

            // if a node with the same position as successor is in the open list
            // which has a lower f than successor, skip this successor
            if cell_details[next_x][next_y].f < f {
                continue;
            }

            // otherwise, add the node to the open list
            open_list.push((next_x, next_y));
            // Update the details of this cell
            cell_details[next_x][next_y].f = f;
            cell_details[next_x][next_y].parent_x = x as isize;
            cell_details[next_x][next_y].parent_y = y as isize;
        }
    }

    // We didn't find a clear path.
    // If we have clear successors we pick a random one.
    let successors = generate_successors(start_x, start_y, grid);
    if successors.len() > 0 {
        let (next_x, next_y) =
            successors[thread_rng().gen_range(0..=successors.len() - 1) as usize];
        return vec![get_direction(start_x, start_y, next_x, next_y)];
    }

    // Brace for impact!
    vec![]
}

fn find_target(grid: &Grid) -> (usize, usize) {
    for (x, row) in grid.iter().enumerate() {
        for (y, cell) in row.iter().enumerate() {
            if *cell == Cell::Food {
                return (x, y);
            }
        }
    }
    unreachable!("No food found in grid!")
}

fn lowest_f(list: &[(usize, usize)], cell_details: &[Vec<CellInfo>]) -> usize {
    assert!(list.len() > 0);

    let mut f = usize::MAX;
    let mut i = 0;
    for (n, (x, y)) in list.iter().enumerate() {
        if cell_details[*x][*y].f < f {
            f = cell_details[*x][*y].f;
            i = n;
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
fn generate_successors(x: usize, y: usize, grid: &Grid) -> Vec<(usize, usize)> {
    let mut result: Vec<(usize, usize)> = Vec::with_capacity(4);

    // north
    if x > 0 {
        result.push((x - 1, y))
    }
    // south
    if x + 1 < grid.len() {
        result.push((x + 1, y))
    }
    // east
    if y + 1 < grid.len() {
        result.push((x, y + 1))
    }
    // west
    if y > 0 {
        result.push((x, y - 1))
    }

    result
        .into_iter()
        .filter(|(x, y)| grid[*x][*y] == Cell::Free || grid[*x][*y] == Cell::Food)
        .collect()
}

fn generate_path(start_x: usize, start_y: usize, cell_details: &[Vec<CellInfo>]) -> Vec<Direction> {
    let mut directions: Vec<Direction> = Vec::new();
    let mut x = start_x;
    let mut y = start_y;
    loop {
        let parent_x = cell_details[x][y].parent_x;
        let parent_y = cell_details[x][y].parent_y;
        if parent_x < 0 || parent_y < 0 {
            break;
        }
        let direction = get_direction(parent_x as usize, parent_y as usize, x, y);
        directions.push(direction);
        x = parent_x as usize;
        y = parent_y as usize;
    }
    directions
}

fn get_direction(from_x: usize, from_y: usize, to_x: usize, to_y: usize) -> Direction {
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

fn manhatten_distance(x: usize, y: usize, target_x: usize, target_y: usize) -> usize {
    let distance = i32::abs(x as i32 - target_x as i32) + i32::abs(y as i32 - target_y as i32);
    distance.try_into().unwrap()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn manhatten_distance_equal() {
        assert_eq!(manhatten_distance(1, 1, 1, 1), 0);
    }

    #[test]
    fn manhatten_distance_positive() {
        assert_eq!(manhatten_distance(1, 1, 3, 4), 5);
    }

    #[test]
    fn manhatten_distance_negative() {
        assert_eq!(manhatten_distance(5, 5, 1, 3), 6);
    }

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
                Direction::South,
                Direction::South,
                Direction::East,
                Direction::East,
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
