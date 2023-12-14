use std::{
    collections::{hash_map::Entry, HashMap, HashSet},
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
        let mut moved = HashSet::new();
        // Roll the rocks
        while let Some(pos) = get_next(&self.0, &moved, direction) {
            // find rollable rock

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

            moved.insert((next_pos.0 as usize, next_pos.1 as usize));
        }
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

fn get_next(
    map: &[Vec<Option<Rock>>],
    moved: &HashSet<(usize, usize)>,
    direction: Direction,
) -> Option<(usize, usize)> {
    match direction {
        Direction::North => {
            for j in 0..map[0].len() {
                if let Some(x) = map
                    .iter()
                    .enumerate()
                    .filter(|(i, _)| !moved.contains(&(*i, j)))
                    .find_map(|(i, row)| {
                        if row[j] == Some(Rock::Round) {
                            return Some((i, j));
                        }
                        None
                    })
                {
                    return Some(x);
                }
            }
        }
        Direction::West => {
            for (i, row) in map.iter().enumerate() {
                for (j, c) in row.iter().enumerate() {
                    if c == &Some(Rock::Round) && !moved.contains(&(i, j)) {
                        return Some((i, j));
                    }
                }
            }
        }
        Direction::South => {
            for j in (0..map[0].len()).rev() {
                for i in (0..map.len()).rev() {
                    if map[i][j] == Some(Rock::Round) && !moved.contains(&(i, j)) {
                        return Some((i, j));
                    }
                }
            }
        }
        Direction::East => {
            for i in (0..map.len()).rev() {
                for j in (0..map[i].len()).rev() {
                    if map[i][j] == Some(Rock::Round) && !moved.contains(&(i, j)) {
                        return Some((i, j));
                    }
                }
            }
        }
    }
    None
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
