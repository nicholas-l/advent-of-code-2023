use std::{
    collections::{BinaryHeap, HashMap, HashSet},
    io::BufRead,
    ops::{Add, Deref},
    str::FromStr,
};

struct Grid(Vec<Vec<isize>>);

impl Grid {
    fn get_heat(&self, pos: &(isize, isize)) -> Option<isize> {
        if pos.0 < 0 || pos.1 < 0 {
            return None;
        }
        self.get(pos.0 as usize)
            .and_then(|row| row.get(pos.1 as usize).copied())
    }
}

impl FromStr for Grid {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut grid = Vec::new();
        for line in s.lines() {
            let mut row = Vec::new();
            for c in line.chars() {
                row.push(c.to_digit(10).unwrap() as isize);
            }
            grid.push(row);
        }
        Ok(Grid(grid))
    }
}

impl Deref for Grid {
    type Target = Vec<Vec<isize>>;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

#[derive(Debug, PartialEq, PartialOrd, Ord, Eq, Clone, Copy, Hash)]
enum Direction {
    Up,
    Down,
    Left,
    Right,
}

impl Direction {
    fn left(&self) -> Self {
        match self {
            Direction::Up => Direction::Left,
            Direction::Down => Direction::Right,
            Direction::Left => Direction::Down,
            Direction::Right => Direction::Up,
        }
    }

    fn right(&self) -> Self {
        match self {
            Direction::Up => Direction::Right,
            Direction::Down => Direction::Left,
            Direction::Left => Direction::Up,
            Direction::Right => Direction::Down,
        }
    }
}

impl Add<(isize, isize)> for Direction {
    type Output = (isize, isize);

    fn add(self, rhs: (isize, isize)) -> Self::Output {
        match self {
            Direction::Up => (rhs.0 - 1, rhs.1),
            Direction::Down => (rhs.0 + 1, rhs.1),
            Direction::Left => (rhs.0, rhs.1 - 1),
            Direction::Right => (rhs.0, rhs.1 + 1),
        }
    }
}

#[derive(Debug, Eq, PartialEq, Clone, PartialOrd, Ord)]
struct State {
    heat: isize,
    pos: (isize, isize),
    dir: Direction,
    count_straight: usize,
}

fn process(grid: &Grid, min_step: usize, max_step: usize) -> isize {
    let mut heap = BinaryHeap::new();
    heap.push(State {
        heat: 0,
        pos: (0, 0),
        dir: Direction::Right,
        count_straight: 0,
    });
    heap.push(State {
        heat: 0,
        pos: (0, 0),
        dir: Direction::Down,
        count_straight: 0,
    });

    let mut cost_map = HashMap::new();
    let mut seen = HashSet::new();

    while let Some(State {
        pos,
        heat,
        dir,
        count_straight,
    }) = heap.pop()
    {
        if pos == (grid.len() as isize - 1, grid[0].len() as isize - 1) {
            return -heat;
        }

        if seen.contains(&(pos, dir, count_straight)) {
            continue;
        }
        seen.insert((pos, dir, count_straight));

        if let Some(next_heat) = grid.get_heat(&(dir + pos)) {
            if count_straight < max_step {
                let pos = dir + pos;
                let heat = heat - next_heat;
                cost_map.insert(pos, -heat);
                heap.push(State {
                    heat,
                    pos,
                    dir,
                    count_straight: count_straight + 1,
                });
            }
        }

        if let Some(next_heat) = grid.get_heat(&(dir.left() + pos)) {
            if count_straight >= min_step {
                let pos = dir.left() + pos;
                let heat = heat - next_heat;

                cost_map.insert(pos, -heat);

                heap.push(State {
                    heat,
                    pos,
                    dir: dir.left(),
                    count_straight: 1,
                });
            }
        }

        if let Some(next_heat) = grid.get_heat(&(dir.right() + pos)) {
            if count_straight >= min_step {
                let pos = dir.right() + pos;
                cost_map.insert(pos, -(heat - next_heat));

                heap.push(State {
                    heat: heat - next_heat,
                    pos,
                    dir: dir.right(),
                    count_straight: 1,
                });
            }
        }
    }

    unreachable!()
}

pub fn star_one(mut input: impl BufRead) -> String {
    let mut str = String::new();
    input.read_to_string(&mut str).unwrap();

    let grid = str.parse::<Grid>().unwrap();

    process(&grid, 1, 3).to_string()
}

pub fn star_two(mut input: impl BufRead) -> String {
    let mut str = String::new();
    input.read_to_string(&mut str).unwrap();

    let grid = str.parse::<Grid>().unwrap();

    process(&grid, 4, 10).to_string()
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::io::Cursor;

    #[test]
    fn test_star_one() {
        assert_eq!(
            star_one(Cursor::new(
                b"2413432311323
3215453535623
3255245654254
3446585845452
4546657867536
1438598798454
4457876987766
3637877979653
4654967986887
4564679986453
1224686865563
2546548887735
4322674655533"
            )),
            "102"
        );
    }

    #[test]
    fn test_star_two() {
        assert_eq!(
            star_two(Cursor::new(
                b"2413432311323
3215453535623
3255245654254
3446585845452
4546657867536
1438598798454
4457876987766
3637877979653
4654967986887
4564679986453
1224686865563
2546548887735
4322674655533"
            )),
            "94"
        );
    }
}
