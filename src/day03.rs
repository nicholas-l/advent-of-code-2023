use std::{
    collections::{HashMap, HashSet},
    io::BufRead,
};

static DIRS: [(i32, i32); 8] = [
    (-1, -1),
    (-1, 0),
    (-1, 1),
    (0, -1),
    (0, 1),
    (1, -1),
    (1, 0),
    (1, 1),
];

fn get_number(
    map: &[Vec<char>],
    y: usize,
    mut x: usize,
    seen: &HashSet<(usize, usize)>,
) -> Option<u32> {
    // Go left on the same line until we find a non-digit character
    while x > 0
        && map
            .get(y)
            .and_then(|line| line.get(x - 1).map(|c| c.is_ascii_digit()))
            .unwrap_or(false)
    {
        x -= 1;
    }

    let mut number = 0;

    while let Some(d) = map
        .get(y)
        .and_then(|line| line.get(x).and_then(|c| c.to_digit(10)))
    {
        if seen.contains(&(y, x)) {
            return None;
        }
        number *= 10;
        number += d;
        x += 1;
    }

    Some(number)
}

pub fn star_one(input: impl BufRead) -> String {
    let map = input
        .lines()
        .map(|line| {
            let line = line.unwrap();
            line.chars().collect::<Vec<char>>()
        })
        .collect::<Vec<_>>();

    let mut sum = 0;

    // get all symbols
    let symbol_locations = map.iter().enumerate().flat_map(|(y, line)| {
        line.iter().enumerate().filter_map(move |(x, c)| {
            if !c.is_ascii_digit() && *c != '.' {
                Some((y, x, *c))
            } else {
                None
            }
        })
    });

    // Keep a track of whether we have seen the number before so we
    // don't add it multiple times.
    let mut seen = HashSet::new();

    for (y, x, _c) in symbol_locations {
        DIRS.iter()
            .filter_map(|(dy, dx)| {
                let newy = y as i32 + *dy;
                let newx = x as i32 + *dx;
                if newy >= 0 || newx >= 0 {
                    Some((newy as usize, newx as usize))
                } else {
                    None
                }
            })
            .filter_map(|(newy, newx)| {
                map.get(newy)
                    .and_then(|line: &Vec<char>| line.get(newx))
                    .map(|c| (newy, newx, *c))
            })
            .filter(|(_newy, _newx, c)| c.is_ascii_digit())
            .for_each(|(newy, newx, _c)| {
                // Now parse the number that we have found the digit of.
                if let Some(number) = get_number(&map, newy, newx, &seen) {
                    sum += number;
                }
                seen.insert((newy, newx));
            })
    }
    sum.to_string()
}

pub fn star_two(input: impl BufRead) -> String {
    let map = input
        .lines()
        .map(|line| {
            let line = line.unwrap();
            line.chars().collect::<Vec<char>>()
        })
        .collect::<Vec<_>>();

    // get all symbols
    let gear_locations = map.iter().enumerate().flat_map(|(y, line)| {
        line.iter()
            .enumerate()
            .filter_map(move |(x, c)| if *c == '*' { Some((y, x, *c)) } else { None })
    });

    let mut gears = HashMap::new();

    for (y, x, _c) in gear_locations {
        let mut seen = HashSet::new();

        DIRS.iter()
            .filter_map(|(dy, dx)| {
                let newy = y as i32 + *dy;
                let newx = x as i32 + *dx;
                if newy >= 0 || newx >= 0 {
                    Some((newy as usize, newx as usize))
                } else {
                    None
                }
            })
            .filter_map(|(newy, newx)| {
                map.get(newy)
                    .and_then(|line: &Vec<char>| line.get(newx))
                    .map(|c| (newy, newx, *c))
            })
            .filter(|(_newy, _newx, c)| c.is_ascii_digit())
            .for_each(|(newy, newx, _c)| {
                if let Some(number) = get_number(&map, newy, newx, &seen) {
                    let entry = gears.entry((y, x)).or_insert_with(Vec::new);
                    entry.push(number);
                }
                seen.insert((newy, newx));
            })
    }

    gears
        .into_iter()
        .filter(|(_k, numbers)| numbers.len() == 2)
        .map(|(_k, numbers)| numbers[0] * numbers[1])
        .sum::<u32>()
        .to_string()
}

#[cfg(test)]
mod tests {
    use super::{star_one, star_two};
    use std::io::Cursor;

    #[test]
    fn test_star_one() {
        assert_eq!(
            star_one(Cursor::new(
                b"467..114..
...*......
..35..633.
......#...
617*......
.....+.58.
..592.....
......755.
...$.*....
.664.598.."
            )),
            "4361"
        );
    }

    #[test]
    fn test_star_two() {
        assert_eq!(
            star_two(Cursor::new(
                b"467..114..
...*......
..35..633.
......#...
617*......
.....+.58.
..592.....
......755.
...$.*....
.664.598.."
            )),
            "467835"
        );
    }
}
