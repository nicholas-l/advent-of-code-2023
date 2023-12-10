use std::{
    collections::{HashMap, HashSet},
    io::BufRead,
};

use num_integer::Integer;

#[derive(Debug)]
enum Instruction {
    Left,
    Right,
}

pub fn star_one(mut input: impl BufRead) -> String {
    let mut str = String::new();
    input.read_to_string(&mut str).expect("cannot read string");

    let mut sections = str.split("\n\n");

    let mut instructions = sections
        .next()
        .unwrap()
        .chars()
        .map(|c| match c {
            'L' => Instruction::Left,
            'R' => Instruction::Right,
            _ => panic!("Unknown instruction"),
        })
        .cycle();

    let map = sections
        .next()
        .unwrap()
        .lines()
        .map(|line| {
            let (from, other) = line.split_once(" = ").unwrap();

            let (left, right) = other
                .trim_matches(|c| c == '(' || c == ')')
                .split_once(", ")
                .map(|(left, right)| (left, right))
                .unwrap();
            (from, (left, right))
        })
        .collect::<HashMap<_, _>>();

    let mut current = "AAA";

    let mut steps = 0;

    while current != "ZZZ" {
        let (left, right) = map.get(current).unwrap();

        let instruction = instructions.next().unwrap();

        current = match instruction {
            Instruction::Left => *left,
            Instruction::Right => *right,
        };
        steps += 1;
    }
    steps.to_string()
}

pub fn star_two(mut input: impl BufRead) -> String {
    let mut str = String::new();
    input.read_to_string(&mut str).expect("cannot read string");

    let mut sections = str.split("\n\n");

    let instructions = sections
        .next()
        .unwrap()
        .chars()
        .map(|c| match c {
            'L' => Instruction::Left,
            'R' => Instruction::Right,
            _ => panic!("Unknown instruction"),
        })
        .cycle();

    let map = sections
        .next()
        .unwrap()
        .lines()
        .map(|line| {
            let (from, other) = line.split_once(" = ").unwrap();

            let (left, right) = other
                .trim_matches(|c| c == '(' || c == ')')
                .split_once(", ")
                .map(|(left, right)| (left, right))
                .unwrap();
            (from, (left, right))
        })
        .collect::<HashMap<_, _>>();

    let mut current = map
        .keys()
        .filter(|k: &&&str| k.ends_with('A'))
        .copied()
        .collect::<HashSet<_>>();

    current.shrink_to_fit();

    // Assume that the end node is the only node that the path ends with i.e.
    // does not go from one *Z to another different *Z
    let steps = current
        .iter()
        .map(|&k| {
            let mut current = k;
            let mut instructions = instructions.clone();
            let mut steps = 0;
            while !current.ends_with('Z') {
                let (left, right) = map.get(current).unwrap();

                let instruction = instructions.next().unwrap();

                current = match instruction {
                    Instruction::Left => *left,
                    Instruction::Right => *right,
                };
                steps += 1;
            }
            steps as usize
        })
        .collect::<Vec<_>>();

    steps
        .into_iter()
        .reduce(|a, b| a.lcm(&b))
        .unwrap()
        .to_string()

    //// This was my original solution, but it was too slow
    //
    // let end = {
    //     let mut end = map
    //         .keys()
    //         .filter(|k: &&&str| k.ends_with('Z'))
    //         .copied()
    //         .collect::<HashSet<_>>();

    //     end.shrink_to_fit();

    //     end
    // };
    // let mut steps = 0;
    // while !current.is_subset(&end) {
    //     let instruction = instructions.next().unwrap();
    //     current = current
    //         .into_iter()
    //         .map(|k| {
    //             let (left, right) = map.get(k).unwrap();
    //             match instruction {
    //                 Instruction::Left => *left,
    //                 Instruction::Right => *right,
    //             }
    //         })
    //         .collect::<HashSet<_>>();

    //     steps += 1;
    // }

    // steps.to_string()
}

#[cfg(test)]
mod tests {
    use super::{star_one, star_two};
    use std::io::Cursor;

    #[test]
    fn test_star_one() {
        assert_eq!(
            star_one(Cursor::new(
                b"RL

AAA = (BBB, CCC)
BBB = (DDD, EEE)
CCC = (ZZZ, GGG)
DDD = (DDD, DDD)
EEE = (EEE, EEE)
GGG = (GGG, GGG)
ZZZ = (ZZZ, ZZZ)"
            )),
            "2"
        );
        assert_eq!(
            star_one(Cursor::new(
                "LLR

AAA = (BBB, BBB)
BBB = (AAA, ZZZ)
ZZZ = (ZZZ, ZZZ)"
            )),
            "6"
        );
    }

    #[test]
    fn test_star_two() {
        assert_eq!(
            star_two(Cursor::new(
                b"LR

11A = (11B, XXX)
11B = (XXX, 11Z)
11Z = (11B, XXX)
22A = (22B, XXX)
22B = (22C, 22C)
22C = (22Z, 22Z)
22Z = (22B, 22B)
XXX = (XXX, XXX)

"
            )),
            "6"
        );
    }
}
