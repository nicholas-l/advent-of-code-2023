use std::{collections::HashSet, io::BufRead, str::FromStr};

use ahash::RandomState;
use itertools::Itertools;
use rayon::prelude::*;

enum Tile {
    Plot,
    Rock,
}

pub fn star_one(mut input: impl BufRead) -> String {
    let mut str = String::new();
    input.read_to_string(&mut str).unwrap();

    let mut current_pos = None;

    let grid = str
        .lines()
        .enumerate()
        .map(|(row, line)| {
            line.chars()
                .enumerate()
                .map(|(col, c)| match c {
                    '.' => Tile::Plot,
                    '#' => Tile::Rock,
                    'S' => {
                        current_pos = Some((row, col));
                        Tile::Plot
                    }
                    _ => panic!("Invalid input"),
                })
                .collect::<Vec<_>>()
        })
        .collect::<Vec<_>>();

    let mut places = HashSet::new();
    places.insert(current_pos.unwrap());

    for _step in 0..64 {
        places = places
            .into_iter()
            .flat_map(|pos| {
                [(-1, 0), (1, 0), (0, -1), (0, 1)]
                    .iter()
                    .map(move |(drow, dcol)| (pos.0 as i32 + drow, pos.1 as i32 + dcol))
                    .filter(|(row, col)| *row >= 0 && *col >= 0)
                    .map(|(row, col)| (row as usize, col as usize))
            })
            .filter(|pos| match grid.get(pos.0).and_then(|row| row.get(pos.1)) {
                Some(Tile::Plot) => true,
                Some(Tile::Rock) => false,
                None => false,
            })
            .collect()
    }

    for (i, row) in grid.iter().enumerate() {
        for (j, c) in row.iter().enumerate() {
            match c {
                Tile::Plot if places.contains(&(i, j)) => print!("O"),
                Tile::Plot => print!("."),
                Tile::Rock => {
                    assert!(!places.contains(&(i, j)), "{:?}", places);
                    print!("#")
                }
            }
        }
        println!();
    }

    places.len().to_string()
}

const DIRS: [(i32, i32); 4] = [(-1, 0), (1, 0), (0, -1), (0, 1)];

// Grid (0, 0) is the center grid.
struct Grid {
    grid: Vec<Vec<Tile>>,
    start_pos: (isize, isize),
}

impl FromStr for Grid {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut current_pos = None;
        Ok(Grid {
            grid: s
                .lines()
                .enumerate()
                .map(|(row, line)| {
                    line.chars()
                        .enumerate()
                        .map(|(col, c)| match c {
                            '.' => Tile::Plot,
                            '#' => Tile::Rock,
                            'S' => {
                                current_pos = Some((row as isize, col as isize));
                                Tile::Plot
                            }
                            _ => panic!("Invalid input"),
                        })
                        .collect::<Vec<_>>()
                })
                .collect::<Vec<_>>(),
            start_pos: current_pos.unwrap(),
        })
    }
}

impl Grid {
    fn cell_to_global(&self, cell: (isize, isize), grid_pos: (isize, isize)) -> (isize, isize) {
        (
            grid_pos.0 + cell.0 * self.grid.len() as isize,
            grid_pos.1 + cell.1 * self.grid[0].len() as isize,
        )
    }

    fn get_cell_bounds(&self, cell: (isize, isize)) -> (isize, isize, isize, isize) {
        let (min_y, min_x) = self.cell_to_global(cell, (0, 0));
        let (max_y, max_x) = self.cell_to_global(
            cell,
            (self.grid.len() as isize, self.grid[0].len() as isize),
        );

        (min_y, min_x, max_y, max_x)
    }
}

fn is_all_bounds_within_distance(
    bounds: (isize, isize, isize, isize),
    max_distance: isize,
) -> bool {
    todo!()
}

fn process_one_grid(grid: &Grid, start_pos: (isize, isize), max_steps: isize) -> usize {
    let length = grid.grid.len() as isize;
    let row_length = grid.grid[0].len() as isize;

    let mut places = HashSet::new();
    places.insert(start_pos);

    for step in 0..max_steps {
        places = places
            .into_iter()
            .cartesian_product(DIRS)
            .map(|(pos, (drow, dcol))| (pos.0 + drow as isize, pos.1 + dcol as isize))
            .filter(|pos| {
                let grid_row = ((length + pos.0.rem_euclid(length)) % length) as usize;
                let grid_col = ((row_length + pos.1.rem_euclid(row_length)) % row_length) as usize;

                matches!(grid.grid[grid_row][grid_col], Tile::Plot)
            })
            .collect();
    }

    places.len()
}

pub fn star_two(mut input: impl BufRead) -> String {
    let mut str = String::new();
    input.read_to_string(&mut str).unwrap();

    let grid = str.parse::<Grid>().unwrap();

    let length = grid.grid.len() as isize;
    let row_length = grid.grid[0].len() as isize;

    let max_steps: isize = 26501365;

    let covered_counts = {
        let other_start_pos: (isize, isize) = (grid.start_pos.0 + 1, grid.start_pos.1);
        (
            process_one_grid(&grid, grid.start_pos, max_steps),
            process_one_grid(&grid, other_start_pos, max_steps),
        )
    };

    let number_of_grids = ((max_steps / length * 2), (max_steps / row_length * 2));

    let mut count = 0;
    // let rem = max_steps % 2;
    for cell_y in 0..number_of_grids.0 {
        for cell_x in 0..number_of_grids.1 {
            let bounds = grid.get_cell_bounds((cell_y, cell_x));
            let manhattan_distance = bounds.0.abs() + bounds.1.abs();
            if is_all_bounds_within_distance(bounds, max_steps) && manhattan_distance % 2 == 0 {
                count += covered_counts.0;
            } else if is_all_bounds_within_distance(bounds, max_steps)
                && manhattan_distance % 2 == 0
            {
                count += covered_counts.1;
            } else {
                // Create the hashmap
            }
        }
    }

    return count.to_string();

    let d = ((-26501365_isize)..=26501365_isize)
        .into_par_iter()
        .flat_map_iter(move |y| (-26501365_isize..=26501365_isize).map(move |x| (y, x)))
        .filter(|(drow, dcol)| drow.abs() + dcol.abs() <= 26501365)
        .filter(|(drow, dcol)| (drow.abs() + dcol.abs()) % 2 == 1)
        .filter(|(drow, dcol)| {
            let pos = (grid.start_pos.0 + drow, grid.start_pos.1 + dcol);
            let grid_row = ((length + pos.0.rem_euclid(length)) % length) as usize;
            let grid_col = ((row_length + pos.1.rem_euclid(row_length)) % row_length) as usize;

            matches!(grid.grid[grid_row][grid_col], Tile::Plot)
        })
        // .inspect(|i| println!("{:?}", i))
        .count();

    return d.to_string();

    // for step in 0..26501365 {
    //     places = places
    //         .into_iter()
    //         .cartesian_product(DIRS)
    //         .map(|(pos, (drow, dcol))| (pos.0 + drow as isize, pos.1 + dcol as isize))
    //         .filter(|pos| {
    //             let grid_row = ((length + pos.0.rem_euclid(length)) % length) as usize;
    //             matches!(
    //                 grid.get(grid_row).and_then(|row| {
    //                     let grid_col =
    //                         ((row_length + pos.1.rem_euclid(row_length)) % row_length) as usize;
    //                     row.get(grid_col)
    //                 }),
    //                 Some(Tile::Plot)
    //             )
    //         })
    //         .collect();
    // places = places
    //     .into_par_iter()
    //     .flat_map(|pos| {
    //         [(-1, 0), (1, 0), (0, -1), (0, 1)]
    //             .into_iter()
    //             .map(|(drow, dcol)| (pos.0 + drow, pos.1 + dcol))
    //             .collect::<Vec<_>>()
    //     })
    //     .filter(|pos| {
    //         let length = grid.len() as isize;
    //         let grid_row = ((length + pos.0.rem_euclid(length)) % length) as usize;
    //         matches!(
    //             grid.get(grid_row).and_then(|row| {
    //                 let length = row.len() as isize;
    //                 let grid_col = ((length + pos.1.rem_euclid(length)) % length) as usize;
    //                 row.get(grid_col)
    //             }),
    //             Some(Tile::Plot)
    //         )
    //     })
    //     .collect();

    //     if step > 64 && step % 5 == 0 {
    //         println!("{},{}", step, places.len());
    //     }
    // }

    // for (i, row) in grid.iter().enumerate() {
    //     for (j, c) in row.iter().enumerate() {
    //         match c {
    //             Tile::Plot if places.contains(&(i as isize, j as isize)) => print!("O"),
    //             Tile::Plot => print!("."),
    //             Tile::Rock => {
    //                 assert!(!places.contains(&(i as isize, j as isize)), "{:?}", places);
    //                 print!("#")
    //             }
    //         }
    //     }
    //     println!();
    // }
    todo!()
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::io::Cursor;

    #[test]
    fn test_star_one() {
        assert_eq!(
            star_one(Cursor::new(
                b"...........
.....###.#.
.###.##..#.
..#.#...#..
....#.#....
.##..S####.
.##..#...#.
.......##..
.##.#.####.
.##..##.##.
..........."
            )),
            "16"
        );
    }

    #[test]
    fn test_star_two() {
        assert_eq!(star_two(Cursor::new(b"")), "167409079868000");
    }
}
