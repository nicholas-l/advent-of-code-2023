use std::{
    collections::{hash_map::Entry, HashMap, VecDeque},
    io::BufRead,
    str::FromStr,
};

enum Tile {
    Plot,
    Rock,
}

pub fn star_one(input: impl BufRead) -> String {
    process(input, 64).to_string()
}

const DIRS: [(i32, i32); 4] = [(-1, 0), (1, 0), (0, -1), (0, 1)];

struct Grid {
    grid: Vec<Vec<Tile>>,
    start_pos: (isize, isize),
}

impl FromStr for Grid {
    type Err = String;

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

impl GridExt for Grid {
    fn get(&self, pos: &Position) -> Option<&Tile> {
        if pos.0 < 0 || pos.1 < 0 {
            return None;
        }
        self.grid
            .get(pos.0 as usize)
            .and_then(|row| row.get(pos.1 as usize))
    }
}

struct InfiniteGrid {
    grid: Grid,
}

impl FromStr for InfiniteGrid {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let grid = Grid::from_str(s)?;
        Ok(InfiniteGrid { grid })
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
struct Position(isize, isize);

trait GridExt {
    fn get(&self, pos: &Position) -> Option<&Tile>;
}

impl GridExt for InfiniteGrid {
    fn get(&self, pos: &Position) -> Option<&Tile> {
        let row = pos.0.rem_euclid(self.grid.grid.len() as isize) as usize;
        let col = pos.1.rem_euclid(self.grid.grid[0].len() as isize) as usize;
        self.grid.grid.get(row).unwrap().get(col)
    }
}

// Helped by https://github.com/villuna/aoc23/wiki/A-Geometric-solution-to-advent-of-code-2023,-day-21
fn process(mut input: impl BufRead, max_steps: isize) -> String {
    let mut queue = VecDeque::new();
    let mut distances = HashMap::new();

    let mut str = String::new();
    input.read_to_string(&mut str).unwrap();

    let grid = str.parse::<Grid>().unwrap();

    let row_length = grid.grid.len() as isize;

    queue.push_back((grid.start_pos, 0));

    while let Some((pos, steps)) = queue.pop_front() {
        if let Entry::Vacant(e) = distances.entry(pos) {
            e.insert(steps);

            for (drow, dcol) in DIRS {
                let new_pos = (pos.0 + drow as isize, pos.1 + dcol as isize);
                if let Some(Tile::Plot) =
                    grid.get(&Position(pos.0 + drow as isize, pos.1 + dcol as isize))
                {
                    queue.push_back((new_pos, steps + 1));
                }
            }
        }
    }

    let even_corners = distances
        .values()
        .filter(|v| **v % 2 == 0 && **v > 65)
        .count();
    let odd_corners = distances
        .values()
        .filter(|v| **v % 2 == 1 && **v > 65)
        .count();

    // Assume square

    if max_steps - (row_length / 2) - 1 > 0 {
        let n = (max_steps - (row_length / 2)) / row_length;
        // println!("n: {}, rows: {}", n, row_length);
        let n = n as usize;
        let even = n * n;
        let odd = (n + 1) * (n + 1);

        let p2 = odd * distances.values().filter(|v| **v % 2 == 1).count()
            + even * distances.values().filter(|v| **v % 2 == 0).count()
            - ((n + 1) * odd_corners)
            + (n * even_corners);

        p2.to_string()
    } else {
        distances
            .values()
            .filter(|v| **v <= max_steps && **v % 2 == max_steps % 2)
            .count()
            .to_string()
    }
}

pub fn star_two(input: impl BufRead) -> String {
    process(input, 26501365)
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::io::Cursor;

    #[test]
    fn test_star_one() {
        assert_eq!(
            process(
                Cursor::new(
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
                ),
                6
            ),
            "16"
        );
    }

    #[test]
    fn test_star_two() {
        let data = b"...........
.....###.#.
.###.##..#.
..#.#...#..
....#.#....
.##..S####.
.##..#...#.
.......##..
.##.#.####.
.##..##.##.
...........";
        assert_eq!(process(Cursor::new(data), 6), "16");
        // assert_eq!(process3(Cursor::new(data), 10), "50");
        // assert_eq!(process3(Cursor::new(data), 50), "1594");
        // assert_eq!(process3(Cursor::new(data), 100), "6536");
        // assert_eq!(process3(Cursor::new(data), 500), "167004");
        // assert_eq!(process3::<InfiniteGrid>(Cursor::new(data), 1000), "668697");
        // assert_eq!(
        //     process3::<InfiniteGrid>(Cursor::new(data), 5000),
        //     "16733044"
        // );
    }
}
