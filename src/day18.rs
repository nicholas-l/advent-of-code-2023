use std::{collections::HashMap, io::BufRead, str::FromStr};

use ahash::RandomState;

enum Direction {
    Up,
    Down,
    Left,
    Right,
}

impl From<char> for Direction {
    fn from(value: char) -> Self {
        match value {
            'U' => Self::Up,
            'D' => Self::Down,
            'L' => Self::Left,
            'R' => Self::Right,
            _ => panic!(),
        }
    }
}

struct Instuction {
    direction: Direction,
    distance: u32,
}

impl FromStr for Instuction {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut s = s.split(' ');
        let direction = s.next().unwrap().chars().next().unwrap().into();
        let distance = s.next().unwrap().parse().unwrap();

        Ok(Self {
            direction,
            distance,
        })
    }
}

enum Position {
    Trench,
    Lagoon,
}

// fn is_lagoon(
//     grid: &HashMap<(i32, i32), u32>,
//     pos: (i32, i32),
//     min: (isize, isize),
//     max: (isize, isize),
// ) -> bool {
//     let mut neighbours = Vec::new();

//     neighbours.into_iter().all(|x| x.is_some())
// }

pub fn star_one(mut input: impl BufRead) -> String {
    let mut str = String::new();
    input.read_to_string(&mut str).unwrap();

    let mut grid = HashMap::new();
    grid.insert((0, 0), Position::Trench);

    let mut current_pos = (0, 0);

    for ins in str.lines().map(|line| line.parse::<Instuction>().unwrap()) {
        for _ in 0..ins.distance {
            match ins.direction {
                Direction::Up => current_pos.1 += 1,
                Direction::Down => current_pos.1 -= 1,
                Direction::Left => current_pos.0 -= 1,
                Direction::Right => current_pos.0 += 1,
            }

            grid.insert(current_pos, Position::Trench);
        }
    }

    // get max x and y
    let max_x = *grid.keys().map(|(x, _)| x).max().unwrap();
    let min_x = *grid.keys().map(|(x, _)| x).min().unwrap();
    // let max_y = *grid.keys().map(|(_, y)| y).max().unwrap();
    let min_y = *grid.keys().map(|(_, y)| y).min().unwrap();

    let mut stack = Vec::new();

    for x in min_x..=max_x {
        if matches!(grid.get(&(x, min_y)), Some(Position::Trench))
            && !grid.contains_key(&(x, min_y + 1))
        {
            grid.insert((x, min_y + 1), Position::Lagoon);
            stack.push((x, min_y + 2));
        }
    }

    while let Some(pos) = stack.pop() {
        if matches!(
            grid.get(&pos),
            Some(Position::Lagoon) | Some(Position::Trench)
        ) {
            continue;
        }

        // If any of the neighbours are lagoons, then this is a lagoon
        if matches!(grid.get(&(pos.0, pos.1 + 1)), Some(Position::Lagoon))
            || matches!(grid.get(&(pos.0, pos.1 - 1)), Some(Position::Lagoon))
            || matches!(grid.get(&(pos.0 + 1, pos.1)), Some(Position::Lagoon))
            || matches!(grid.get(&(pos.0 - 1, pos.1)), Some(Position::Lagoon))
        {
            grid.insert(pos, Position::Lagoon);
            stack.extend(
                [
                    (pos.0, pos.1 + 1),
                    (pos.0, pos.1 - 1),
                    (pos.0 + 1, pos.1),
                    (pos.0 - 1, pos.1),
                ]
                .into_iter(),
            );
        }
    }

    grid.into_values()
        .filter(|p| matches!(p, Position::Lagoon | Position::Trench))
        .count()
        .to_string()
}

struct Instuction2 {
    direction: Direction,
    distance: usize,
}

impl FromStr for Instuction2 {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut s = s.split(' ');
        let s = s.next_back().unwrap();
        let s = s.trim_matches('(').trim_matches(')').trim_matches('#');
        let value = usize::from_str_radix(&s[..5], 16).unwrap();
        let direction = match &s[5..] {
            "0" => Direction::Right,
            "1" => Direction::Down,
            "2" => Direction::Left,
            "3" => Direction::Up,
            x => panic!("Unknown direction {x} from {s}"),
        };
        let distance = value;

        Ok(Self {
            direction,
            distance,
        })
    }
}

pub fn star_two(mut input: impl BufRead) -> String {
    let mut str = String::new();
    input.read_to_string(&mut str).unwrap();

    let mut grid: HashMap<(isize, isize), Position, RandomState> = HashMap::default();
    grid.insert((0, 0), Position::Trench);

    let mut current_pos = (0, 0);

    let mut points = vec![(0, 0)];
    let mut length: isize = 0;

    for ins in str.lines().map(|line| line.parse::<Instuction2>().unwrap()) {
        match ins.direction {
            Direction::Up => current_pos.1 += ins.distance as isize,
            Direction::Down => current_pos.1 -= ins.distance as isize,
            Direction::Left => current_pos.0 -= ins.distance as isize,
            Direction::Right => current_pos.0 += ins.distance as isize,
        }
        length += ins.distance as isize;
        points.push(current_pos);
    }

    // a_{rea}=\frac{1}{2}\left|\sum_{i=1}^{n-1}\left(x_{1}\left[i\right]y_{1}\left[i+1\right]-y_{1}\left[i\right]x_{1}\left[i+1\right]\right)+x_{1}\left[n\right]y_{1}\left[1\right]-y_{1}\left[n\right]x_{1}\left[1\right]\right|
    // From https://www.desmos.com/calculator/j6v0dbjzmt

    let area = points
        .windows(2)
        .map(|w| w[0].0 * w[1].1 - w[0].1 * w[1].0)
        .sum::<isize>()
        + points.last().unwrap().0 * points.first().unwrap().1
        - points.last().unwrap().1 * points.first().unwrap().0;

    // Have to add length as the width of the perimeter is 1, not sure about the 1 off though.
    let area = area.abs() / 2 + length / 2 + 1;

    area.to_string()
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::io::Cursor;

    #[test]
    fn test_star_one() {
        assert_eq!(
            star_one(Cursor::new(
                b"R 6 (#70c710)
D 5 (#0dc571)
L 2 (#5713f0)
D 2 (#d2c081)
R 2 (#59c680)
D 2 (#411b91)
L 5 (#8ceee2)
U 2 (#caa173)
L 1 (#1b58a2)
U 2 (#caa171)
R 2 (#7807d2)
U 3 (#a77fa3)
L 2 (#015232)
U 2 (#7a21e3)"
            )),
            "62"
        );
    }

    #[test]
    fn test_star_two() {
        assert_eq!(
            star_two(Cursor::new(
                b"R 6 (#70c710)
D 5 (#0dc571)
L 2 (#5713f0)
D 2 (#d2c081)
R 2 (#59c680)
D 2 (#411b91)
L 5 (#8ceee2)
U 2 (#caa173)
L 1 (#1b58a2)
U 2 (#caa171)
R 2 (#7807d2)
U 3 (#a77fa3)
L 2 (#015232)
U 2 (#7a21e3)"
            )),
            "952408144115"
        );
    }
}
