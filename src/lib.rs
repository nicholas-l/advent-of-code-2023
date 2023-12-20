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
mod day11;
mod day12;
mod day13;
mod day14;
mod day15;
mod day16;
mod day17;
mod day18;
mod day19;

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

        11 => {
            use day11::{star_one, star_two};
            (
                star_one as DayFn,
                star_two as DayFn,
                Path::new("data").join("day11.txt"),
            )
        }

        12 => {
            use day12::{star_one, star_two};
            (
                star_one as DayFn,
                star_two as DayFn,
                Path::new("data").join("day12.txt"),
            )
        }

        13 => {
            use day13::{star_one, star_two};
            (
                star_one as DayFn,
                star_two as DayFn,
                Path::new("data").join("day13.txt"),
            )
        }

        14 => {
            use day14::{star_one, star_two};
            (
                star_one as DayFn,
                star_two as DayFn,
                Path::new("data").join("day14.txt"),
            )
        }

        15 => {
            use day15::{star_one, star_two};
            (
                star_one as DayFn,
                star_two as DayFn,
                Path::new("data").join("day15.txt"),
            )
        }

        16 => {
            use day16::{star_one, star_two};
            (
                star_one as DayFn,
                star_two as DayFn,
                Path::new("data").join("day16.txt"),
            )
        }

        17 => {
            use day17::{star_one, star_two};
            (
                star_one as DayFn,
                star_two as DayFn,
                Path::new("data").join("day17.txt"),
            )
        }

        18 => {
            use day18::{star_one, star_two};
            (
                star_one as DayFn,
                star_two as DayFn,
                Path::new("data").join("day18.txt"),
            )
        }

        19 => {
            use day19::{star_one, star_two};
            (
                star_one as DayFn,
                star_two as DayFn,
                Path::new("data").join("day19.txt"),
            )
        }
        x => {
            unimplemented!("Have not implemented day {}", x);
        }
    }
}

pub fn get_days() -> impl Iterator<Item = usize> {
    1..=17
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
        let (star_one, star_two, filepath) = get_day(5);
        assert_eq!(star_one(get_data(&filepath)), "322500873");

        assert_eq!(star_two(get_data(&filepath)), "108956227");
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

    #[test]
    fn day11_complete() {
        let (star_one, star_two, filepath) = get_day(11);
        assert_eq!(star_one(get_data(&filepath)), "9623138");

        assert_eq!(star_two(get_data(&filepath)), "726820169514");
    }

    #[test]
    fn day12_complete() {
        let (star_one, star_two, filepath) = get_day(12);
        assert_eq!(star_one(get_data(&filepath)), "7173");

        assert_eq!(star_two(get_data(&filepath)), "29826669191291");
    }

    #[test]
    fn day13_complete() {
        let (star_one, star_two, filepath) = get_day(13);
        assert_eq!(star_one(get_data(&filepath)), "31739");

        assert_eq!(star_two(get_data(&filepath)), "31539");
    }

    #[test]
    fn day14_complete() {
        let (star_one, star_two, filepath) = get_day(14);
        assert_eq!(star_one(get_data(&filepath)), "113078");

        assert_eq!(star_two(get_data(&filepath)), "94255");
    }

    #[test]
    fn day15_complete() {
        let (star_one, star_two, filepath) = get_day(15);
        assert_eq!(star_one(get_data(&filepath)), "511215");

        assert_eq!(star_two(get_data(&filepath)), "236057");
    }

    #[test]
    fn day16_complete() {
        let (star_one, star_two, filepath) = get_day(16);
        assert_eq!(star_one(get_data(&filepath)), "7472");

        assert_eq!(star_two(get_data(&filepath)), "7716");
    }

    #[test]
    fn day17_complete() {
        let (star_one, star_two, filepath) = get_day(17);
        assert_eq!(star_one(get_data(&filepath)), "928");

        assert_eq!(star_two(get_data(&filepath)), "1104");
    }

    #[test]
    fn day18_complete() {
        let (star_one, star_two, filepath) = get_day(18);
        assert_eq!(star_one(get_data(&filepath)), "62365");

        assert_eq!(star_two(get_data(&filepath)), "159485361249806");
    }

    #[test]
    fn day19_complete() {
        let (star_one, star_two, filepath) = get_day(19);
        assert_eq!(star_one(get_data(&filepath)), "319062");

        assert_eq!(star_two(get_data(&filepath)), "118638369682135");
    }
}
