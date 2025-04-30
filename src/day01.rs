use std::io::BufRead;

pub fn star_one(input: impl BufRead) -> String {
    input
        .lines()
        .map(|line| {
            let line = line.unwrap();
            let mut digits = line
                .chars()
                .filter(|c| c.is_ascii_digit())
                .map(|c| c.to_digit(10).unwrap());

            let first_digit = digits.next().unwrap();

            // If there is only one digit then the last digit is the same as the first
            let last_digit = digits.next_back().unwrap_or(first_digit);

            first_digit * 10 + last_digit
        })
        .sum::<u32>()
        .to_string()
}

pub fn star_two(input: impl BufRead) -> String {
    input
        .lines()
        .map(|line| {
            let line = {
                let line = line.unwrap();
                // Here we are replacing the words with numbers along with the
                // start and end letter of the word as these may be used in the
                // other digit such as `oneight` which should end up with the
                // number 18. This is a bit of a hack but it is to get around
                // fact that Regex module does not support overlapping matches
                // nor searching from the reverse direction.
                line.replace("one", "o1e")
                    .replace("two", "t2o")
                    .replace("three", "t3e")
                    .replace("four", "f4r")
                    .replace("five", "f5e")
                    .replace("six", "s6x")
                    .replace("seven", "s7n")
                    .replace("eight", "e8t")
                    .replace("nine", "n9e")
            };
            let mut digits = line
                .chars()
                .filter(|c| c.is_ascii_digit())
                .map(|c| c.to_digit(10).unwrap());

            let first_digit = digits.next().unwrap();
            let last_digit = digits.next_back().unwrap_or(first_digit);

            first_digit * 10 + last_digit
        })
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
                b"1abc2
pqr3stu8vwx
a1b2c3d4e5f
treb7uchet"
            )),
            "142"
        );
    }

    #[test]
    fn test_star_two() {
        assert_eq!(
            star_two(Cursor::new(
                b"two1nine
eightwothree
abcone2threexyz
xtwone3four
4nineeightseven2
zoneight234
7pqrstsixteen"
            )),
            "281"
        );
    }
}
