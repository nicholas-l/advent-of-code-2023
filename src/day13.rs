use std::{
    fmt::{Display, Formatter},
    io::BufRead,
    ops::Deref,
};

#[derive(PartialEq, Debug, Clone)]
enum Element {
    Ash,
    Rocks,
}

impl From<char> for Element {
    fn from(value: char) -> Self {
        match value {
            '.' => Element::Ash,
            '#' => Element::Rocks,
            _ => unreachable!(),
        }
    }
}

#[derive(PartialEq, Debug)]
enum MirrorPosition {
    Row(usize),
    Column(usize),
}

impl Display for MirrorPosition {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            MirrorPosition::Row(row) => write!(f, "row {row}"),
            MirrorPosition::Column(col) => write!(f, "column {col}"),
        }
    }
}

struct Map(Vec<Vec<Element>>);

impl Display for Map {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        for row in self.0.iter() {
            for col in row.iter() {
                write!(
                    f,
                    "{}",
                    match col {
                        Element::Ash => '.',
                        Element::Rocks => '#',
                    }
                )?;
            }
            writeln!(f)?;
        }
        Ok(())
    }
}

struct MapGenerator {
    map: Map,
    position: (usize, usize),
}

impl Iterator for MapGenerator {
    type Item = Map;

    fn next(&mut self) -> Option<Self::Item> {
        let mut map = self.map.clone();

        let (x, y) = self.position;

        if y >= map.len() {
            return None;
        }

        map[y][x] = match map[y][x] {
            Element::Ash => Element::Rocks,
            Element::Rocks => Element::Ash,
        };

        self.position = if x + 1 < map[y].len() {
            (x + 1, y)
        } else {
            (0, y + 1)
        };

        Some(Map(map))
    }
}

impl Deref for Map {
    type Target = Vec<Vec<Element>>;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

fn find_mirror(group: &Map, existing: Option<&MirrorPosition>) -> Option<MirrorPosition> {
    // Maybe only need to do half
    for row in 0..group.len() {
        let length = (row + 1).min(group.len() - row - 1);
        let offset = row + 1 - length;

        if length > 0
            && (0..length)
                // FIXME needs to be adjust to row
                .all(|i| {
                    &group[offset + i]
                        == group.get(offset + length * 2 - i - 1).unwrap_or_else(|| {
                            panic!(
                                "Trying to compare when offset is \n{group}\n\n{offset}, {length}, {row}, {i}"
                            )
                        })
                })
        {
            if let Some(existing) = existing {
                if existing == &MirrorPosition::Row(row) {
                    continue;
                }
            }
            return Some(MirrorPosition::Row(row));
        }
    }

    let col_length = group[0].len();

    for col in 0..col_length {
        let length = (col + 1).min(col_length - col - 1);
        let offset = col + 1 - length;
        if length > 0
            && (0..group.len()).all(|y| {
                (0..length).all(|i| group[y][offset + i] == group[y][offset + length * 2 - i - 1])
            })
        {
            if let Some(existing) = existing {
                if existing == &MirrorPosition::Column(col) {
                    continue;
                }
            }
            return Some(MirrorPosition::Column(col));
        }
    }
    None
}

pub fn star_one(mut input: impl BufRead) -> String {
    let mut str = String::new();
    input.read_to_string(&mut str).expect("cannot read string");

    str.split("\n\n")
        .map(|group| {
            group
                .lines()
                .map(|line| line.chars().map(Element::from).collect::<Vec<_>>())
                .collect::<Vec<Vec<_>>>()
        })
        .map(|group| {
            // find the mirror
            let mirror = find_mirror(&Map(group), None).unwrap();

            match mirror {
                MirrorPosition::Row(row) => 100 * (row + 1),
                MirrorPosition::Column(col) => col + 1,
            }
        })
        .sum::<usize>()
        .to_string()
}

pub fn star_two(mut input: impl BufRead) -> String {
    let mut str = String::new();
    input.read_to_string(&mut str).expect("cannot read string");

    let maps = str
        .split("\n\n")
        .map(|group| {
            group
                .lines()
                .map(|line| line.chars().map(Element::from).collect::<Vec<_>>())
                .collect::<Vec<Vec<_>>>()
        })
        .collect::<Vec<_>>();
    maps.into_iter()
        .map(|group| {
            let map = Map(group);
            let mirror = find_mirror(&map, None).unwrap();
            // find the mirror
            let generator = MapGenerator {
                map,
                position: (0, 0),
            };
            let mirror = generator
                .into_iter()
                .filter_map(|map| find_mirror(&map, Some(&mirror)))
                .find(|position| *position != mirror)
                .unwrap();

            match mirror {
                MirrorPosition::Row(row) => 100 * (row + 1),
                MirrorPosition::Column(col) => col + 1,
            }
        })
        .sum::<usize>()
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
                b"#.##..##.
..#.##.#.
##......#
##......#
..#.##.#.
..##..##.
#.#.##.#.

#...##..#
#....#..#
..##..###
#####.##.
#####.##.
..##..###
#....#..#"
            )),
            "405"
        );
    }

    #[test]
    fn test_star_two() {
        assert_eq!(
            star_two(Cursor::new(
                b"#.##..##.
..#.##.#.
##......#
##......#
..#.##.#.
..##..##.
#.#.##.#.

#...##..#
#....#..#
..##..###
#####.##.
#####.##.
..##..###
#....#..#"
            )),
            "400"
        );
    }
}
