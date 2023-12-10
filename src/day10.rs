use std::{
    collections::{HashMap, HashSet, VecDeque},
    io::BufRead,
};

/*

   | is a vertical pipe connecting north and south.
   - is a horizontal pipe connecting east and west.
   L is a 90-degree bend connecting north and east.
   J is a 90-degree bend connecting north and west.
   7 is a 90-degree bend connecting south and west.
   F is a 90-degree bend connecting south and east.
   . is ground; there is no pipe in this tile.
   S is the starting position of the animal; there is a pipe on this tile, but your sketch doesn't show what shape the pipe has.

*/

#[derive(PartialEq, Clone, Copy, Debug)]
enum Position {
    Empty,
    Vertical,
    Horizontal,
    NorthEast,
    NorthWest,
    SouthWest,
    SouthEast,
    Start,
}

fn get_count_to_edge(
    map: &HashMap<(isize, isize), Position>,
    path: &HashSet<(isize, isize)>,
    pos: &(isize, isize),
) -> usize {
    let mut crossing_count = 0;

    let mut last_bend = None;

    for i in 0..=pos.0 {
        if path.contains(&(i, pos.1)) {
            let pos_type = map.get(&(i, pos.1)).unwrap();

            let pos_type = if matches!(pos_type, Position::Start) {
                let above = matches!(
                    map.get(&(i - 1, pos.1)),
                    Some(Position::Vertical)
                        | Some(Position::SouthEast)
                        | Some(Position::SouthWest)
                );

                let left = matches!(
                    map.get(&(i, pos.1 - 1)),
                    Some(Position::Horizontal)
                        | Some(Position::NorthEast)
                        | Some(Position::SouthEast)
                );
                let right = matches!(
                    map.get(&(i, pos.1 + 1)),
                    Some(Position::Horizontal)
                        | Some(Position::SouthWest)
                        | Some(Position::NorthWest)
                );

                let down = matches!(
                    map.get(&(i + 1, pos.1)),
                    Some(Position::Vertical)
                        | Some(Position::NorthWest)
                        | Some(Position::NorthEast)
                );

                match (above, left, right, down) {
                    (true, true, true, true) => panic!(),
                    (true, true, true, false) => panic!(),
                    (true, true, false, true) => panic!(),
                    (true, true, false, false) => Position::NorthWest,
                    (true, false, true, true) => panic!(),
                    (true, false, true, false) => Position::NorthEast,
                    (true, false, false, true) => Position::Vertical,
                    (true, false, false, false) => panic!(),
                    (false, true, true, true) => panic!(),
                    (false, true, true, false) => Position::Horizontal,
                    (false, true, false, true) => Position::SouthWest,
                    (false, true, false, false) => panic!(),
                    (false, false, true, true) => Position::SouthEast,
                    (false, false, true, false) => panic!(),
                    (false, false, false, true) => panic!(),
                    (false, false, false, false) => panic!(),
                }
            } else {
                *pos_type
            };

            if pos == &(8, 12) {
                println!("({} {}) {} - {:?}", i, pos.1, crossing_count, pos_type);
            }

            match pos_type {
                Position::Empty => {}
                Position::Vertical => {}
                Position::Horizontal => {
                    crossing_count += 1;
                }
                Position::NorthEast => {
                    if let Some(last_bend) = last_bend.take() {
                        if last_bend == Position::SouthWest {
                            crossing_count += 1;
                        }
                    } else {
                        panic!("we should have seen a bend")
                    }
                }
                Position::NorthWest => {
                    if let Some(last_bend) = last_bend.take() {
                        if last_bend == Position::SouthEast {
                            crossing_count += 1;
                        }
                    } else {
                        panic!("we should have seen a bend")
                    }
                }
                Position::SouthWest => {
                    if last_bend.is_none() {
                        last_bend = Some(Position::SouthWest);
                    } else {
                        panic!("we should not have seen a bend")
                    }
                }
                Position::SouthEast => {
                    if last_bend.is_none() {
                        last_bend = Some(Position::SouthEast);
                    } else {
                        panic!("we should not have seen a bend")
                    }
                }
                Position::Start => {
                    panic!()
                    // Might have to look and see what type it is
                }
            }
        }
    }
    crossing_count
}

pub fn star_one(input: impl BufRead) -> String {
    let map = input
        .lines()
        .enumerate()
        .flat_map(|(y, line)| {
            let line = line.unwrap();
            line.chars()
                .enumerate()
                .map(move |(x, c)| {
                    let pos = match c {
                        '.' => Position::Empty,
                        '|' => Position::Vertical,
                        '-' => Position::Horizontal,
                        'L' => Position::NorthEast,
                        'J' => Position::NorthWest,
                        '7' => Position::SouthWest,
                        'F' => Position::SouthEast,
                        'S' => Position::Start,

                        _ => panic!("Unknown character"),
                    };
                    ((y as isize, x as isize), pos)
                })
                .collect::<Vec<_>>()
        })
        .collect::<HashMap<_, _>>();

    let start = map
        .iter()
        .find_map(|(&pos, value)| {
            if value == &Position::Start {
                Some(pos)
            } else {
                None
            }
        })
        .unwrap();

    let mut stack = VecDeque::new();

    stack.push_back((start, 0, HashSet::new()));

    let mut circ = Vec::new();

    while let Some(((y, x), steps, mut seen)) = stack.pop_back() {
        // println!("{:?} {}", (y, x), steps);
        if seen.contains(&(y, x)) {
            circ.push(steps);
            continue;
        }
        seen.insert((y, x));
        if let Some(pos) = map.get(&(y, x)) {
            match pos {
                Position::Empty => {}
                Position::Vertical => {
                    stack.push_back(((y + 1, x), steps + 1, seen.clone()));
                    stack.push_back(((y - 1, x), steps + 1, seen.clone()));
                }
                Position::Horizontal => {
                    stack.push_back(((y, x + 1), steps + 1, seen.clone()));
                    stack.push_back(((y, x - 1), steps + 1, seen.clone()));
                }
                Position::NorthEast => {
                    stack.push_back(((y - 1, x), steps + 1, seen.clone()));
                    stack.push_back(((y, x + 1), steps + 1, seen.clone()));
                }
                Position::NorthWest => {
                    stack.push_back(((y - 1, x), steps + 1, seen.clone()));
                    stack.push_back(((y, x - 1), steps + 1, seen.clone()));
                }
                Position::SouthWest => {
                    stack.push_back(((y + 1, x), steps + 1, seen.clone()));
                    stack.push_back(((y, x - 1), steps + 1, seen.clone()));
                }
                Position::SouthEast => {
                    stack.push_back(((y + 1, x), steps + 1, seen.clone()));
                    stack.push_back(((y, x + 1), steps + 1, seen.clone()));
                }
                Position::Start => {
                    stack.push_back(((y + 1, x), steps + 1, seen.clone()));
                    stack.push_back(((y - 1, x), steps + 1, seen.clone()));
                    stack.push_back(((y, x + 1), steps + 1, seen.clone()));
                    stack.push_back(((y, x - 1), steps + 1, seen.clone()));
                }
            }
        }
    }

    circ.iter().max().map(|x| x / 2).unwrap().to_string()
}

pub fn star_two(input: impl BufRead) -> String {
    let map = input
        .lines()
        .enumerate()
        .flat_map(|(y, line)| {
            let line = line.unwrap();
            line.chars()
                .enumerate()
                .map(move |(x, c)| {
                    let pos = match c {
                        '.' => Position::Empty,
                        '|' => Position::Vertical,
                        '-' => Position::Horizontal,
                        'L' => Position::NorthEast,
                        'J' => Position::NorthWest,
                        '7' => Position::SouthWest,
                        'F' => Position::SouthEast,
                        'S' => Position::Start,

                        x => panic!("Unknown character: {}", x),
                    };
                    ((y as isize, x as isize), pos)
                })
                .collect::<Vec<_>>()
        })
        .collect::<HashMap<_, _>>();

    let start = map
        .iter()
        .find_map(|(&pos, value)| {
            if value == &Position::Start {
                Some(pos)
            } else {
                None
            }
        })
        .unwrap();

    let mut stack = VecDeque::new();

    stack.push_back((start, 0, HashSet::new()));

    let mut circ = Vec::new();

    while let Some(((y, x), steps, mut seen)) = stack.pop_back() {
        // println!("{:?} {}", (y, x), steps);
        if seen.contains(&(y, x)) {
            circ.push((steps, seen));
            continue;
        }
        seen.insert((y, x));
        if let Some(pos) = map.get(&(y, x)) {
            match pos {
                Position::Empty => {}
                Position::Vertical => {
                    stack.push_back(((y + 1, x), steps + 1, seen.clone()));
                    stack.push_back(((y - 1, x), steps + 1, seen.clone()));
                }
                Position::Horizontal => {
                    stack.push_back(((y, x + 1), steps + 1, seen.clone()));
                    stack.push_back(((y, x - 1), steps + 1, seen.clone()));
                }
                Position::NorthEast => {
                    stack.push_back(((y - 1, x), steps + 1, seen.clone()));
                    stack.push_back(((y, x + 1), steps + 1, seen.clone()));
                }
                Position::NorthWest => {
                    stack.push_back(((y - 1, x), steps + 1, seen.clone()));
                    stack.push_back(((y, x - 1), steps + 1, seen.clone()));
                }
                Position::SouthWest => {
                    stack.push_back(((y + 1, x), steps + 1, seen.clone()));
                    stack.push_back(((y, x - 1), steps + 1, seen.clone()));
                }
                Position::SouthEast => {
                    stack.push_back(((y + 1, x), steps + 1, seen.clone()));
                    stack.push_back(((y, x + 1), steps + 1, seen.clone()));
                }
                Position::Start => {
                    stack.push_back(((y + 1, x), steps + 1, seen.clone()));
                    stack.push_back(((y - 1, x), steps + 1, seen.clone()));
                    stack.push_back(((y, x + 1), steps + 1, seen.clone()));
                    stack.push_back(((y, x - 1), steps + 1, seen.clone()));
                }
            }
        }
    }

    let path = circ.iter().max_by_key(|k| k.0).unwrap();
    let max_y = map.keys().map(|(y, _)| y).max().unwrap();
    let max_x = map.keys().map(|(_, x)| x).max().unwrap();

    for i in 0..=*max_y {
        for j in 0..=*max_x {
            if path.1.contains(&(i, j)) {
                print!("x");
            } else {
                print!(" ");
            }
        }
        println!()
    }

    let mut total = 0;
    for i in 0..=*max_y {
        for j in 0..=*max_x {
            if path.1.contains(&(i, j)) {
                print!("x");
                continue;
            }
            let count = get_count_to_edge(&map, &path.1, &(i, j));
            if count % 2 == 1 {
                print!("{}", count);
                total += 1;
            } else {
                print!("{}", count);
            }
        }
        println!()
    }
    total.to_string()
}

#[cfg(test)]
mod tests {
    use super::{star_one, star_two};
    use std::io::Cursor;

    #[test]
    fn test_star_one() {
        assert_eq!(
            star_one(Cursor::new(
                b".....
.S-7.
.|.|.
.L-J.
....."
            )),
            "4"
        );
        assert_eq!(
            star_one(Cursor::new(
                b"..F7.
.FJ|.
SJ.L7
|F--J
LJ..."
            )),
            "8"
        );
    }

    #[test]
    fn test_star_two() {
        assert_eq!(
            star_two(Cursor::new(
                "...........
.S-------7.
.|F-----7|.
.||.....||.
.||.....||.
.|L-7.F-J|.
.|..|.|..|.
.L--J.L--J.
..........."
            )),
            "4"
        );

        assert_eq!(
            star_two(Cursor::new(
                b"
.F----7F7F7F7F-7....
.|F--7||||||||FJ....
.||.FJ||||||||L7....
FJL7L7LJLJ||LJ.L-7..
L--J.L7...LJS7F-7L7.
....F-J..F7FJ|L7L7L7
....L7.F7||L7|.L7L7|
.....|FJLJ|FJ|F7|.LJ
....FJL-7.||.||||...
....L---J.LJ.LJLJ..."
            )),
            "8"
        );

        assert_eq!(
            star_two(Cursor::new(
                b"FF7FSF7F7F7F7F7F---7
L|LJ||||||||||||F--J
FL-7LJLJ||||||LJL-77
F--JF--7||LJLJ7F7FJ-
L---JF-JLJ.||-FJLJJ7
|F|F-JF---7F7-L7L|7|
|FFJF7L7F-JF7|JL---7
7-L-JL7||F7|L7F-7F7|
L.L7LFJ|||||FJL7||LJ
L7JLJL-JLJLJL--JLJ.L"
            )),
            "10"
        );
    }
}
