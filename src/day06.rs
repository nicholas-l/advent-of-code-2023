use std::io::BufRead;

pub fn star_one(input: impl BufRead) -> String {
    let mut lines = input.lines();

    let times = lines
        .next()
        .unwrap()
        .unwrap()
        .split_once(": ")
        .unwrap()
        .1
        .split_whitespace()
        .map(|s| s.parse::<isize>().unwrap())
        .collect::<Vec<_>>();

    let distances = lines
        .next()
        .unwrap()
        .unwrap()
        .split_once(": ")
        .unwrap()
        .1
        .split_whitespace()
        .map(|s| s.parse::<isize>().unwrap())
        .collect::<Vec<_>>();

    times
        .iter()
        .zip(distances.iter())
        .map(|(&t, &d)| {
            let x = (-t as f64 + ((t.pow(2) + 4 * d) as f64).sqrt()) / 2f64;

            // Find all races that it is possible to win
            (x.ceil() as isize..d)
                .map(|v| v * (t - v))
                .filter(|new_d| new_d > &d)
                .count()
        })
        .inspect(|&x| println!("{}", x))
        .product::<usize>()
        .to_string()
}

pub fn star_two(input: impl BufRead) -> String {
    let mut lines = input.lines();

    let times = {
        let time = lines
            .next()
            .unwrap()
            .unwrap()
            .split_once(": ")
            .unwrap()
            .1
            .replace(' ', "");

        vec![time.parse::<isize>().unwrap()]
    };

    let distances = {
        let distance = lines
            .next()
            .unwrap()
            .unwrap()
            .split_once(": ")
            .unwrap()
            .1
            .replace(' ', "");
        vec![distance.parse::<isize>().unwrap()]
    };

    times
        .iter()
        .zip(distances.iter())
        .map(|(&t, &d)| {
            // Quadratic equation for solving for x  (-x^2 +Bx + C = 0 where B is time and C is distance)
            let x1 = (-t as f64 + ((t.pow(2) + 4 * d) as f64).sqrt()) / 2f64;

            let x2 = (-t as f64 - ((t.pow(2) + 4 * d) as f64).sqrt()) / 2f64;

            // Find all races that it is possible to win
            (x1.ceil() as isize..x2.abs().floor() as isize)
                .map(|v| v * (t - v))
                .filter(|new_d| new_d > &d)
                .count()
        })
        .product::<usize>()
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
                b"Time:      7  15   30
Distance:  9  40  200"
            )),
            "288"
        );
    }

    #[test]
    fn test_star_two() {
        assert_eq!(
            star_two(Cursor::new(
                b"Time:      7  15   30
        Distance:  9  40  200"
            )),
            "71503"
        );
    }
}
