use crate::game::{Cell, Direction, Grid};
use log::{debug, error, info, log_enabled, Level};

#[derive(Default, Clone, Debug)]
struct CellInfo {
    x: usize,
    y: usize,
    parent_x: isize,
    parent_y: isize,
    f: usize,
    g: usize,
}

// https://www.geeksforgeeks.org/a-search-algorithm/
pub fn solve(grid: &Grid, (start_x, start_y): (usize, usize)) -> Vec<Direction> {
    let (target_x, target_y) = find_target(grid);
    debug!(
        "start: ({},{}) target: ({},{})",
        start_x, start_y, target_x, target_y
    );

    // 1.  Initialize the open list
    let mut open_list: Vec<CellInfo> = Vec::with_capacity(grid.len());

    // 2.  Initialize the closed list
    //     put the starting node on the open list (you can leave its f at zero)
    let mut closed_list: Vec<Vec<Option<CellInfo>>> = vec![vec![None; grid.len()]; grid.len()];
    open_list.push(CellInfo {
        x: start_x,
        y: start_y,
        parent_x: -1,
        parent_y: -1,
        ..Default::default()
    });

    // 3.  while the open list is not empty
    while !open_list.is_empty() {
        // a) find the node with the least f on the open list, call it "q"
        let i = lowest_f(&open_list);

        // b) pop q off the open list
        let q = open_list.remove(i);

        // c) generate q's 4 successors
        let successors = generate_successors(&q, grid);

        debug!("successors: {:?}", successors);

        // d) for each successor
        'successor: for (x, y) in successors.into_iter() {
            // i) if successor is the goal, stop search
            if x == target_x && y == target_y {
                closed_list[q.x][q.y] = Some(CellInfo {
                    x: q.x,
                    y: q.y,
                    parent_x: q.parent_x,
                    parent_y: q.parent_y,
                    ..Default::default()
                });
                closed_list[x][y] = Some(CellInfo {
                    x,
                    y,
                    parent_x: q.x as isize,
                    parent_y: q.y as isize,
                    ..Default::default()
                });
                return generate_path(
                    &CellInfo {
                        x,
                        y,
                        parent_x: q.x as isize,
                        parent_y: q.y as isize,
                        ..Default::default()
                    },
                    &closed_list,
                );
            }

            // ii) compute g,h and f for successor
            let g = q.g + 1;
            let h = manhatten_distance(x, y, target_x, target_y);
            let f = g + h as usize;

            // iii) if a node with the same position as successor is in the OPEN list
            //      which has a lower f than successor, skip this successor
            for c in open_list.iter() {
                if c.x == x && c.y == y && c.f < f {
                    continue 'successor;
                }
            }

            // iV) if a node with the same position as successor is in the CLOSED list
            //     which has a lower f than successor, skip this successor
            if let Some(c) = &closed_list[x][y] {
                if c.f < f {
                    continue;
                }
            }

            // otherwise, add the node to the open list
            open_list.push(CellInfo {
                x,
                y,
                f,
                g,
                parent_x: q.x as isize,
                parent_y: q.y as isize,
            })
        }

        // e) push q on the closed list
        let (x, y) = (q.x, q.y);
        closed_list[x][y] = Some(q);
    }

    unreachable!()
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

fn lowest_f(list: &[CellInfo]) -> usize {
    assert!(list.len() > 0);

    let mut f = usize::MAX;
    let mut i = 0;
    for (n, cell) in list.iter().enumerate() {
        if cell.f < f {
            f = cell.f;
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
fn generate_successors(cell: &CellInfo, grid: &Grid) -> Vec<(usize, usize)> {
    let mut result: Vec<(usize, usize)> = Vec::with_capacity(4);

    // north
    if cell.x > 0 {
        let x = cell.x - 1;
        let y = cell.y;
        result.push((x, y))
    }
    // south
    if cell.x + 1 < grid.len() {
        let x = cell.x + 1;
        let y = cell.y;
        result.push((x, y))
    }
    // east
    if cell.y + 1 < grid.len() {
        let x = cell.x;
        let y = cell.y + 1;
        result.push((x, y))
    }
    // west
    if cell.y > 0 {
        let x = cell.x;
        let y = cell.y - 1;
        result.push((x, y))
    }

    result
        .into_iter()
        .filter(|(x, y)| grid[*x][*y] == Cell::Free || grid[*x][*y] == Cell::Food)
        .collect()
}

fn generate_path(start: &CellInfo, list: &[Vec<Option<CellInfo>>]) -> Vec<Direction> {
    let mut directions: Vec<Direction> = Vec::new();
    let mut x = start.x;
    let mut y = start.y;
    let mut parent_x = start.parent_x;
    let mut parent_y = start.parent_y;
    while parent_x >= 0 && parent_y >= 0 {
        match &list[parent_x as usize][parent_y as usize] {
            Some(parent) => {
                debug!("({},{}) -> ({},{})", parent.x, parent.y, x, y);
                let direction = get_direction(parent.x, parent.y, x, y);
                directions.push(direction);
                x = parent.x;
                y = parent.y;
                parent_x = parent.parent_x;
                parent_y = parent.parent_y;
            }
            None => break,
        }
    }
    directions.into_iter().rev().collect()
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
                Direction::South,
                Direction::South,
                Direction::East,
                Direction::East,
                Direction::North,
                Direction::North,
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
                Direction::North,
                Direction::North,
                Direction::West,
                Direction::West,
                Direction::South,
                Direction::South,
            ]
        )
    }

    #[test]
    fn generate_path_works() {
        let start = CellInfo {
            x: 2,
            y: 0,
            parent_x: 1,
            parent_y: 0,
            ..Default::default()
        };

        let mut list: Vec<Vec<Option<CellInfo>>> = vec![vec![None; 3]; 3];
        list[0][0] = Some(CellInfo {
            parent_x: -1,
            parent_y: -1,
            ..Default::default()
        });
        list[1][0] = Some(CellInfo {
            x: 1,
            y: 0,
            ..Default::default()
        });
        list[2][0] = Some(CellInfo {
            parent_x: 1,
            parent_y: 0,
            ..Default::default()
        });

        assert_eq!(
            generate_path(&start, &list),
            vec![Direction::East, Direction::East]
        )
    }
}
