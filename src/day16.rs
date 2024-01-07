use std::{
    collections::HashSet,
    io::BufRead,
    ops::{Add, Deref},
    str::FromStr,
};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
enum Direction {
    Up,
    Down,
    Left,
    Right,
}

impl Add<(isize, isize)> for Direction {
    type Output = (isize, isize);

    fn add(self, rhs: (isize, isize)) -> Self::Output {
        match self {
            Direction::Up => (rhs.0, rhs.1 - 1),
            Direction::Down => (rhs.0, rhs.1 + 1),
            Direction::Left => (rhs.0 - 1, rhs.1),
            Direction::Right => (rhs.0 + 1, rhs.1),
        }
    }
}

enum Position {
    Empty,
    LeftMirror,
    RightMirror,
    VericalSplitter,
    HoritzontalSplitter,
}

struct Grid(Vec<Vec<Position>>);

impl FromStr for Grid {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut grid = Vec::new();
        for line in s.lines() {
            let mut row = Vec::new();
            for c in line.chars() {
                row.push(match c {
                    '.' => Position::Empty,
                    '/' => Position::LeftMirror,
                    '\\' => Position::RightMirror,
                    '|' => Position::VericalSplitter,
                    '-' => Position::HoritzontalSplitter,
                    x => return Err(format!("Invalid character {}", x)),
                });
            }
            grid.push(row);
        }
        Ok(Grid(grid))
    }
}

impl Grid {
    fn get(&self, x: isize, y: isize) -> Option<&Position> {
        if y < 0 || x < 0 {
            return None;
        }
        self.0.get(y as usize).and_then(|row| row.get(x as usize))
    }

    fn starting_positions(&self) -> Vec<((isize, isize), Direction)> {
        // All positions on the top row
        let top_positions = self
            .0
            .first()
            .map(|row| {
                row.iter()
                    .enumerate()
                    .map(|(i, _c)| ((i as isize, 0), Direction::Down))
                    .collect::<Vec<_>>()
            })
            .unwrap_or_default()
            .into_iter();

        let bottom_positions = self
            .0
            .last()
            .map(|row| {
                row.iter()
                    .enumerate()
                    .map(|(i, _c)| ((i as isize, self.0.len() as isize - 1), Direction::Up))
                    .collect::<Vec<_>>()
            })
            .unwrap_or_default()
            .into_iter();

        let left_positions = (0..self.0.len()).map(|i| ((0, i as isize), Direction::Right));

        let right_positions =
            (0..self.0.len()).map(|i| ((self.0.len() as isize - 1, i as isize), Direction::Left));

        top_positions
            .chain(bottom_positions)
            .chain(left_positions)
            .chain(right_positions)
            .collect()
    }
}

impl Deref for Grid {
    type Target = Vec<Vec<Position>>;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

pub fn star_one(mut input: impl BufRead) -> String {
    let mut str = String::new();
    input.read_to_string(&mut str).unwrap();

    let map = str.parse::<Grid>().unwrap();

    let mut seen: HashSet<((isize, isize), Direction)> = HashSet::new();
    let mut stack = Vec::new();
    stack.push(((0, 0), Direction::Right));

    while let Some((pos, dir)) = stack.pop() {
        if seen.contains(&(pos, dir)) {
            continue;
        }
        seen.insert((pos, dir));
        match map.get(pos.0, pos.1) {
            Some(Position::Empty) => {
                stack.push((dir + pos, dir));
            }
            Some(Position::LeftMirror) => {
                let new_dir = match dir {
                    Direction::Up => Direction::Right,
                    Direction::Down => Direction::Left,
                    Direction::Left => Direction::Down,
                    Direction::Right => Direction::Up,
                };
                stack.push((new_dir + pos, new_dir));
            }
            Some(Position::RightMirror) => {
                let new_dir = match dir {
                    Direction::Up => Direction::Left,
                    Direction::Down => Direction::Right,
                    Direction::Left => Direction::Up,
                    Direction::Right => Direction::Down,
                };
                stack.push((new_dir + pos, new_dir));
            }
            Some(Position::VericalSplitter) => {
                if dir == Direction::Left || dir == Direction::Right {
                    stack.push((Direction::Up + pos, Direction::Up));
                    stack.push((Direction::Down + pos, Direction::Down));
                } else {
                    stack.push((dir + pos, dir));
                }
            }
            Some(Position::HoritzontalSplitter) => {
                if dir == Direction::Up || dir == Direction::Down {
                    stack.push((Direction::Left + pos, Direction::Left));
                    stack.push((Direction::Right + pos, Direction::Right));
                } else {
                    stack.push((dir + pos, dir));
                }
            }
            None => {}
        }
    }

    let tiles_covered = seen
        .into_iter()
        .map(|(pos, _dir)| pos)
        .collect::<HashSet<_>>();

    // for (i, row) in map.iter().enumerate() {
    //     for (j, _c) in row.iter().enumerate() {
    //         print!(
    //             "{}",
    //             if tiles_covered.contains(&(j as isize, i as isize)) {
    //                 '#'
    //             } else {
    //                 '.'
    //             }
    //         );
    //     }
    //     println!("");
    // }

    tiles_covered
        .into_iter()
        .filter(|pos| map.get(pos.0, pos.1).is_some())
        .count()
        .to_string()
}

pub fn star_two(mut input: impl BufRead) -> String {
    let mut str = String::new();
    input.read_to_string(&mut str).unwrap();

    let map = str.parse::<Grid>().unwrap();

    let starting = map.starting_positions();
    starting
        .into_iter()
        .map(|(pos, dir)| {
            let mut seen: HashSet<((isize, isize), Direction)> = HashSet::new();
            let mut stack = Vec::new();
            stack.push((pos, dir));

            while let Some((pos, dir)) = stack.pop() {
                if seen.contains(&(pos, dir)) {
                    continue;
                }
                seen.insert((pos, dir));
                match map.get(pos.0, pos.1) {
                    Some(Position::Empty) => {
                        stack.push((dir + pos, dir));
                    }
                    Some(Position::LeftMirror) => {
                        let new_dir = match dir {
                            Direction::Up => Direction::Right,
                            Direction::Down => Direction::Left,
                            Direction::Left => Direction::Down,
                            Direction::Right => Direction::Up,
                        };
                        stack.push((new_dir + pos, new_dir));
                    }
                    Some(Position::RightMirror) => {
                        let new_dir = match dir {
                            Direction::Up => Direction::Left,
                            Direction::Down => Direction::Right,
                            Direction::Left => Direction::Up,
                            Direction::Right => Direction::Down,
                        };
                        stack.push((new_dir + pos, new_dir));
                    }
                    Some(Position::VericalSplitter) => {
                        if dir == Direction::Left || dir == Direction::Right {
                            stack.push((Direction::Up + pos, Direction::Up));
                            stack.push((Direction::Down + pos, Direction::Down));
                        } else {
                            stack.push((dir + pos, dir));
                        }
                    }
                    Some(Position::HoritzontalSplitter) => {
                        if dir == Direction::Up || dir == Direction::Down {
                            stack.push((Direction::Left + pos, Direction::Left));
                            stack.push((Direction::Right + pos, Direction::Right));
                        } else {
                            stack.push((dir + pos, dir));
                        }
                    }
                    None => {}
                }
            }

            let tiles_covered = seen
                .into_iter()
                .map(|(pos, _dir)| pos)
                .collect::<HashSet<_>>();

            // for (i, row) in map.iter().enumerate() {
            //     for (j, _c) in row.iter().enumerate() {
            //         print!(
            //             "{}",
            //             if tiles_covered.contains(&(j as isize, i as isize)) {
            //                 '#'
            //             } else {
            //                 '.'
            //             }
            //         );
            //     }
            //     println!("");
            // }

            tiles_covered
                .into_iter()
                .filter(|pos| map.get(pos.0, pos.1).is_some())
                .count()
        })
        .max()
        .unwrap()
        .to_string()
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::io::Cursor;

    #[test]
    fn test_star_one() {
        assert_eq!(
            star_one(Cursor::new(
                b".|...\\....
|.-.\\.....
.....|-...
........|.
..........
.........\\
..../.\\\\..
.-.-/..|..
.|....-|.\\
..//.|...."
            )),
            "46"
        );
    }

    #[test]
    fn test_star_two() {
        assert_eq!(
            star_two(Cursor::new(
                b".|...\\....
|.-.\\.....
.....|-...
........|.
..........
.........\\
..../.\\\\..
.-.-/..|..
.|....-|.\\
..//.|...."
            )),
            "51"
        );
    }
}
