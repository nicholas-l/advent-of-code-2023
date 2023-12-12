use ahash::RandomState;
use std::{collections::HashMap, io::BufRead};

use itertools::{repeat_n, Itertools};
use rayon::prelude::*;

#[derive(Clone, PartialEq, Eq, Debug, Hash)]
enum Status {
    Damaged,
    Functional,
    Unknown,
}

impl From<char> for Status {
    fn from(value: char) -> Self {
        match value {
            '?' => Status::Unknown,
            '.' => Status::Functional,
            '#' => Status::Damaged,
            _ => unreachable!(),
        }
    }
}

pub fn star_one(input: impl BufRead) -> String {
    let data = input
        .lines()
        .map(|line| {
            let line = line.unwrap();

            let (statuses, continuous) = line.split_once(' ').unwrap();

            let statuses = statuses.chars().map(|c| c.into()).collect::<Vec<Status>>();

            let continuous = continuous
                .split(',')
                .map(|s| s.parse::<usize>().unwrap())
                .collect::<Vec<_>>();

            (statuses, continuous)
        })
        .collect::<Vec<_>>();

    data.into_par_iter()
        .map(|(s, c)| {
            let mut cache = HashMap::default();
            process(&s, &c, 0, &mut cache)
        })
        .sum::<usize>()
        .to_string()
}

fn process_functional<'a, 'b>(
    statuses: &'a [Status],
    continuous: &'b [usize],
    current_count: usize,
    cache: &mut Cache<'a, 'b>,
) -> usize {
    if current_count == 0 {
        process(&statuses[1..], continuous, current_count, cache)
    } else if continuous
        .first()
        .map(|&c| c == current_count)
        .unwrap_or(false)
    {
        process(&statuses[1..], &continuous[1..], 0, cache)
    } else {
        0
    }
}
type Cache<'a, 'b> = HashMap<(&'a [Status], &'b [usize], usize), usize, RandomState>;

fn process_damaged<'a, 'b>(
    statuses: &'a [Status],
    continuous: &'b [usize],
    current_count: usize,
    cache: &mut Cache<'a, 'b>,
) -> usize {
    process(&statuses[1..], continuous, current_count + 1, cache)
}

fn process<'a, 'b>(
    statuses: &'a [Status],
    continuous: &'b [usize],
    current_count: usize,
    cache: &mut Cache<'a, 'b>,
) -> usize {
    if let Some(count) = cache.get(&(statuses, continuous, current_count)) {
        return *count;
    }
    let res = match statuses.first() {
        Some(Status::Functional) => process_functional(statuses, continuous, current_count, cache),

        Some(Status::Damaged) => process_damaged(statuses, continuous, current_count, cache),
        Some(Status::Unknown) => {
            // We have an unknown status so try both functional and damaged
            process_damaged(statuses, continuous, current_count, cache)
                + process_functional(statuses, continuous, current_count, cache)
        }
        // We are at the end
        // We have no more continuous to check and there are no more damaged
        None if continuous.is_empty() && current_count == 0 => 1,
        // We have a single continuous to check and it matches the current count
        None if continuous.len() == 1 && current_count == *continuous.first().unwrap() => 1,
        // We have more continuous to check but no more statuses
        None => 0,
    };
    cache.insert((statuses, continuous, current_count), res);
    res
}

pub fn star_two(input: impl BufRead) -> String {
    let data = input
        .lines()
        .map(|line| {
            let line = line.unwrap();

            let (statuses, continuous) = line.split_once(' ').unwrap();

            let statuses = repeat_n(statuses, 5).join("?");
            let continuous = repeat_n(continuous, 5).join(",");

            let statuses = statuses.chars().map(|c| c.into()).collect::<Vec<Status>>();

            let continuous = continuous
                .split(',')
                .map(|s| s.parse::<usize>().unwrap())
                .collect::<Vec<_>>();

            (statuses, continuous)
        })
        .collect::<Vec<_>>();

    data.into_par_iter()
        .map(|(s, c)| {
            let mut cache = HashMap::default();
            process(&s, &c, 0, &mut cache)
        })
        .sum::<usize>()
        .to_string()
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::io::Cursor;

    #[test]
    fn test_process() {
        let data: Vec<Status> = "???.###".chars().map(|c| c.into()).collect::<Vec<_>>();
        let mut cache = HashMap::default();
        assert_eq!(process(&data, &[1, 1, 3], 0, &mut cache), 1);

        let data: Vec<Status> = ".??..??...?##."
            .chars()
            .map(|c| match c {
                '?' => Status::Unknown,
                '.' => Status::Functional,
                '#' => Status::Damaged,
                _ => unreachable!(),
            })
            .collect::<Vec<_>>();
        let mut cache = HashMap::default();

        assert_eq!(process(&data, &[1, 1, 3], 0, &mut cache), 4);
    }

    #[test]
    fn test_star_one() {
        assert_eq!(
            star_one(Cursor::new(
                b"???.### 1,1,3
.??..??...?##. 1,1,3
?#?#?#?#?#?#?#? 1,3,1,6
????.#...#... 4,1,1
????.######..#####. 1,6,5
?###???????? 3,2,1"
            )),
            "21"
        );
    }

    #[test]
    fn test_star_two() {
        assert_eq!(
            star_two(Cursor::new(
                b"???.### 1,1,3
.??..??...?##. 1,1,3
?#?#?#?#?#?#?#? 1,3,1,6
????.#...#... 4,1,1
????.######..#####. 1,6,5
?###???????? 3,2,1"
            )),
            "525152"
        );
    }
}
