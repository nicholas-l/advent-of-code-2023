use std::io::BufRead;

pub fn star_one(input: impl BufRead) -> String {
    input
        .lines()
        .map(|line| {
            let line = line.unwrap();
            line.split_ascii_whitespace()
                .map(|value| value.parse::<isize>().unwrap())
                .collect::<Vec<_>>()
        })
        .map(|values| {
            let mut current = vec![values.clone()];
            while current.last().unwrap().iter().any(|&v| v != 0) {
                let d = current
                    .last()
                    .unwrap()
                    .windows(2)
                    .map(|window| {
                        let (a, b) = (window[0], window[1]);
                        b - a
                    })
                    .collect();
                current.push(d);
            }
            current.iter().map(|v| v.last().unwrap()).sum::<isize>()
        })
        .sum::<isize>()
        .to_string()
}

pub fn star_two(input: impl BufRead) -> String {
    input
        .lines()
        .map(|line| {
            let line = line.unwrap();
            line.split_ascii_whitespace()
                .map(|value| value.parse::<isize>().unwrap())
                .collect::<Vec<_>>()
        })
        .map(|values| {
            let mut current = vec![values.clone()];
            while current.last().unwrap().iter().any(|&v| v != 0) {
                let d = current
                    .last()
                    .unwrap()
                    .windows(2)
                    .map(|window| {
                        let (a, b) = (window[0], window[1]);
                        b - a
                    })
                    .collect();
                current.push(d);
            }
            current
                .iter()
                .zip([1, -1].iter().cycle())
                .map(|(v, m)| m * v.first().unwrap())
                .sum::<isize>()
        })
        .sum::<isize>()
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
                b"0 3 6 9 12 15
1 3 6 10 15 21
10 13 16 21 30 45"
            )),
            "114"
        );
    }

    #[test]
    fn test_star_two() {
        assert_eq!(
            star_two(Cursor::new(
                b"0 3 6 9 12 15
1 3 6 10 15 21
10 13 16 21 30 45"
            )),
            "2"
        );
    }
}
