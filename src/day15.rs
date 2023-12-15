use std::{collections::HashMap, io::BufRead};

fn hash(str: &str) -> usize {
    let mut value = 0;
    for c in str.chars() {
        value += c as usize;
        value *= 17;
        value %= 256;
    }
    value
}

pub fn star_one(mut input: impl BufRead) -> String {
    let mut str = String::new();
    input.read_to_string(&mut str).expect("cannot read string");
    str.split(',').map(hash).sum::<usize>().to_string()
}

#[derive(Debug)]
enum Operation {
    Equal(isize),
    Dash,
}

pub fn star_two(mut input: impl BufRead) -> String {
    let mut str = String::new();
    input.read_to_string(&mut str).expect("cannot read string");

    let boxes = str
        .split(',')
        .map(|s| {
            if let Some((label, integer)) = s.split_once('=') {
                (Operation::Equal(integer.parse::<isize>().unwrap()), label)
            } else {
                (Operation::Dash, s.trim_matches('-'))
            }
        })
        .fold(HashMap::new(), |mut state, (op, label)| {
            match op {
                Operation::Equal(value) => {
                    let lens_box: &mut Vec<(&str, isize)> = state.entry(hash(label)).or_default();
                    if let Some(v) = lens_box.iter_mut().find(|v| v.0 == label) {
                        v.1 = value;
                    } else {
                        lens_box.push((label, value));
                    }
                }
                Operation::Dash => {
                    if let Some(values) = state.get_mut(&hash(label)) {
                        values.retain_mut(|v| v.0 != label);
                    }
                }
            }
            state
        });
    // println!("{:?}", boxes);
    boxes
        .iter()
        .map(|(key, values)| {
            values
                .iter()
                .enumerate()
                .map(|(i, v)| (i + 1) * v.1 as usize)
                .sum::<usize>()
                * (key + 1)
        })
        .sum::<usize>()
        .to_string()
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::io::Cursor;

    #[test]
    fn test_hash() {
        assert_eq!(hash("HASH"), 52);
    }

    #[test]
    fn test_star_one() {
        assert_eq!(
            star_one(Cursor::new(
                b"rn=1,cm-,qp=3,cm=2,qp-,pc=4,ot=9,ab=5,pc-,pc=6,ot=7"
            )),
            "1320"
        );
    }

    #[test]
    fn test_star_two() {
        assert_eq!(
            star_two(Cursor::new(
                b"rn=1,cm-,qp=3,cm=2,qp-,pc=4,ot=9,ab=5,pc-,pc=6,ot=7"
            )),
            "145"
        );
    }
}
