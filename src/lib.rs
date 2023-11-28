use std::{
    fs,
    io::{BufRead, BufReader},
    path::{Path, PathBuf},
};

type DayFn = fn(Box<dyn BufRead>) -> String;

pub fn get_day(day: usize) -> (DayFn, DayFn, PathBuf) {
    match day {
        x => {
            unimplemented!("Have not implemented day {}", x);
        }
    }
}

pub fn get_days() -> impl Iterator<Item = usize> {
    1..=0
}

pub fn get_data(filepath: &Path) -> Box<dyn BufRead> {
    let f = fs::File::open(filepath).unwrap();
    let input = BufReader::new(f);
    Box::new(input)
}

#[cfg(test)]
mod tests {
    // use super::*;
}
