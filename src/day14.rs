use std::{
    collections::{hash_map::Entry, HashMap},
    io::BufRead,
    ops::Add,
    str::FromStr,
};

#[derive(PartialEq, Hash, Debug, Clone, Copy, Eq)]
enum Rock {
    Square,
    Round,
}

#[derive(PartialEq, Eq, Hash, Debug, Clone, Copy)]
enum Direction {
    North,
    West,
    South,
    East,
}

impl Add<(isize, isize)> for Direction {
    type Output = (isize, isize);

    fn add(self, rhs: (isize, isize)) -> Self::Output {
        match self {
            Direction::North => (rhs.0 - 1, rhs.1),
            Direction::West => (rhs.0, rhs.1 - 1),
            Direction::South => (rhs.0 + 1, rhs.1),
            Direction::East => (rhs.0, rhs.1 + 1),
        }
    }
}

struct Map(Vec<Vec<Option<Rock>>>);

impl Map {
    fn weight(&self) -> usize {
        let total_rows = self.0.len();
        self.0
            .iter()
            .enumerate()
            .flat_map(|(i, row)| {
                row.iter()
                    .filter(|rock| **rock == Some(Rock::Round))
                    .map(move |_| total_rows - i)
            })
            .sum::<usize>()
    }

    fn tilt(&mut self, direction: Direction) {
        // Roll the rocks
        // find rollable rock
        while let Some(pos) = self.get_next_rollable(direction) {
            let mut next_pos: (isize, isize) = (pos.0 as isize, pos.1 as isize);

            // While the next position is empty `Some(None)` i.e. there is a place but it is not
            // filled with anything
            while self
                .0
                .get((direction + next_pos).0 as usize)
                .and_then(|row| row.get((direction + next_pos).1 as usize))
                == Some(&None)
            {
                next_pos = direction + next_pos;
            }

            let r = self.0[pos.0][pos.1].take();
            self.0[next_pos.0 as usize][next_pos.1 as usize] = r;
        }
    }

    fn get_next_rollable(&self, direction: Direction) -> Option<(usize, usize)> {
        match direction {
            Direction::North => {
                for j in 0..self.0[0].len() {
                    if let Some(x) = self
                        .0
                        .iter()
                        .enumerate()
                        .skip(1)
                        .filter(|(_i, row)| row[j] == Some(Rock::Round))
                        .filter(|(i, _)| self.0[i - 1][j].is_none())
                        .map(|(i, _row)| (i, j))
                        .next()
                    {
                        return Some(x);
                    }
                }
            }
            Direction::West => {
                for (i, row) in self.0.iter().enumerate() {
                    if let Some(x) = row
                        .iter()
                        .enumerate()
                        .skip(1)
                        .filter(|(_i, c)| c == &&Some(Rock::Round))
                        .filter(|(j, _)| self.0[i][j - 1].is_none())
                        .map(|(j, _)| (i, j))
                        .next()
                    {
                        return Some(x);
                    }
                }
            }
            Direction::South => {
                for j in (0..self.0[0].len()).rev() {
                    if let Some(pos) = self
                        .0
                        .iter()
                        .enumerate()
                        .rev()
                        .skip(1)
                        .filter(|(_i, row)| row[j] == Some(Rock::Round))
                        .filter(|(i, _)| self.0[i + 1][j].is_none())
                        .map(|(i, _)| (i, j))
                        .next()
                    {
                        return Some(pos);
                    }
                }
            }
            Direction::East => {
                for (i, row) in self.0.iter().enumerate().rev() {
                    if let Some(pos) = row
                        .iter()
                        .enumerate()
                        .rev()
                        .skip(1)
                        .filter(|(_j, c)| c == &&Some(Rock::Round))
                        .filter(|(j, _)| self.0[i][j + 1].is_none())
                        .map(|(j, _)| (i, j))
                        .next()
                    {
                        return Some(pos);
                    }
                }
            }
        }
        None
    }
}

impl FromStr for Map {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(Self(
            s.lines()
                .map(|line| {
                    line.chars()
                        .map(|c| match c {
                            '.' => None,
                            '#' => Some(Rock::Square),
                            'O' => Some(Rock::Round),
                            _ => panic!("invalid input"),
                        })
                        .collect::<Vec<_>>()
                })
                .collect::<Vec<_>>(),
        ))
    }
}

pub fn star_one(mut input: impl BufRead) -> String {
    let mut str = String::new();
    input.read_to_string(&mut str).expect("cannot read string");

    let mut map: Map = str.parse().unwrap();

    map.tilt(Direction::North);

    map.weight().to_string()
}

pub fn star_two(mut input: impl BufRead) -> String {
    let mut str = String::new();
    input.read_to_string(&mut str).expect("cannot read string");

    let mut map: Map = str.parse().unwrap();

    let number_of_cycles = 1_000_000_000;
    let mut cache = HashMap::new();

    let mut cycle = 0;

    while cycle < number_of_cycles {
        cycle += match cache.entry(map.0.clone()) {
            Entry::Occupied(last_i) => {
                let cycle_length = cycle - last_i.get();
                let cycles_left = number_of_cycles - cycle;
                (cycles_left / cycle_length) * cycle_length
            }
            Entry::Vacant(e) => {
                e.insert(cycle);
                0
            }
        };

        map.tilt(Direction::North);
        map.tilt(Direction::West);
        map.tilt(Direction::South);
        map.tilt(Direction::East);

        cycle += 1;
    }

    map.weight().to_string()
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::io::Cursor;

    #[test]
    fn test_star_one() {
        assert_eq!(
            star_one(Cursor::new(
                b"O....#....
O.OO#....#
.....##...
OO.#O....O
.O.....O#.
O.#..O.#.#
..O..#O..O
.......O..
#....###..
#OO..#...."
            )),
            "136"
        );
    }

    #[test]
    fn test_star_two() {
        assert_eq!(
            star_two(Cursor::new(
                b"O....#....
O.OO#....#
.....##...
OO.#O....O
.O.....O#.
O.#..O.#.#
..O..#O..O
.......O..
#....###..
#OO..#...."
            )),
            "64"
        );
    }
}
