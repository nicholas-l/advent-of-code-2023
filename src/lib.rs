use std::{
    fs,
    io::{BufRead, BufReader},
    path::{Path, PathBuf},
};

mod day01;
mod day02;
mod day03;
mod day04;
mod day05;
mod day06;
mod day07;
mod day08;
mod day09;
mod day10;

type DayFn = fn(Box<dyn BufRead>) -> String;

pub fn get_day(day: usize) -> (DayFn, DayFn, PathBuf) {
    match day {
        1 => {
            use day01::{star_one, star_two};
            (
                star_one as DayFn,
                star_two as DayFn,
                Path::new("data").join("day01.txt"),
            )
        }

        2 => {
            use day02::{star_one, star_two};
            (
                star_one as DayFn,
                star_two as DayFn,
                Path::new("data").join("day02.txt"),
            )
        }

        3 => {
            use day03::{star_one, star_two};
            (
                star_one as DayFn,
                star_two as DayFn,
                Path::new("data").join("day03.txt"),
            )
        }

        4 => {
            use day04::{star_one, star_two};
            (
                star_one as DayFn,
                star_two as DayFn,
                Path::new("data").join("day04.txt"),
            )
        }

        5 => {
            use day05::{star_one, star_two};
            (
                star_one as DayFn,
                star_two as DayFn,
                Path::new("data").join("day05.txt"),
            )
        }

        6 => {
            use day06::{star_one, star_two};
            (
                star_one as DayFn,
                star_two as DayFn,
                Path::new("data").join("day06.txt"),
            )
        }

        7 => {
            use day07::{star_one, star_two};
            (
                star_one as DayFn,
                star_two as DayFn,
                Path::new("data").join("day07.txt"),
            )
        }

        8 => {
            use day08::{star_one, star_two};
            (
                star_one as DayFn,
                star_two as DayFn,
                Path::new("data").join("day08.txt"),
            )
        }

        9 => {
            use day09::{star_one, star_two};
            (
                star_one as DayFn,
                star_two as DayFn,
                Path::new("data").join("day09.txt"),
            )
        }

        10 => {
            use day10::{star_one, star_two};
            (
                star_one as DayFn,
                star_two as DayFn,
                Path::new("data").join("day10.txt"),
            )
        }

        x => {
            unimplemented!("Have not implemented day {}", x);
        }
    }
}

pub fn get_days() -> impl Iterator<Item = usize> {
    1..=10
}

pub fn get_data(filepath: &Path) -> Box<dyn BufRead> {
    let f = fs::File::open(filepath).unwrap();
    let input = BufReader::new(f);
    Box::new(input)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn day01_complete() {
        let (star_one, star_two, filepath) = get_day(1);
        assert_eq!(star_one(get_data(&filepath)), "55017");

        assert_eq!(star_two(get_data(&filepath)), "53539");
    }

    #[test]
    fn day02_complete() {
        let (star_one, star_two, filepath) = get_day(2);
        assert_eq!(star_one(get_data(&filepath)), "2551");

        assert_eq!(star_two(get_data(&filepath)), "62811");
    }

    #[test]
    fn day03_complete() {
        let (star_one, star_two, filepath) = get_day(3);
        assert_eq!(star_one(get_data(&filepath)), "538046");

        assert_eq!(star_two(get_data(&filepath)), "81709807");
    }

    #[test]
    fn day04_complete() {
        let (star_one, star_two, filepath) = get_day(4);
        assert_eq!(star_one(get_data(&filepath)), "32001");

        assert_eq!(star_two(get_data(&filepath)), "5037841");
    }

    #[test]
    fn day05_complete() {
        let (star_one, _star_two, filepath) = get_day(5);
        assert_eq!(star_one(get_data(&filepath)), "322500873");
        // Very slow implementation
        // assert_eq!(star_two(get_data(&filepath)), "108956227");
    }

    #[test]
    fn day06_complete() {
        let (star_one, star_two, filepath) = get_day(6);
        assert_eq!(star_one(get_data(&filepath)), "503424");

        assert_eq!(star_two(get_data(&filepath)), "32607562");
    }

    #[test]
    fn day07_complete() {
        let (star_one, star_two, filepath) = get_day(7);
        assert_eq!(star_one(get_data(&filepath)), "253205868");

        assert_eq!(star_two(get_data(&filepath)), "253907829");
    }

    #[test]
    fn day08_complete() {
        let (star_one, star_two, filepath) = get_day(8);
        assert_eq!(star_one(get_data(&filepath)), "22411");

        assert_eq!(star_two(get_data(&filepath)), "11188774513823");
    }

    #[test]
    fn day09_complete() {
        let (star_one, star_two, filepath) = get_day(9);
        assert_eq!(star_one(get_data(&filepath)), "1861775706");

        assert_eq!(star_two(get_data(&filepath)), "1082");
    }

    #[test]
    fn day10_complete() {
        let (star_one, star_two, filepath) = get_day(10);
        assert_eq!(star_one(get_data(&filepath)), "7086");

        assert_eq!(star_two(get_data(&filepath)), "317");
    }
}
