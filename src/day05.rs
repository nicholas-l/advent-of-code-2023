use std::{cmp::Reverse, collections::HashMap, io::BufRead};

#[derive(Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
struct Range {
    start: usize,
    end: usize,
}

impl Range {
    fn new(start: usize, end: usize) -> Self {
        Range { start, end }
    }

    fn is_empty(&self) -> bool {
        self.start == self.end
    }

    /// Returns the new range after mapping onto a range map along with the range that was mapped
    fn map_onto_rangemap(&self, range: &RangeMap) -> Option<(Range, Range)> {
        if let Some(new_destination) = range.get(self.start) {
            let new_length = (self.end).min(range.source + range.length) - self.start;
            assert!(new_length > 0);

            Some((
                Range::new(self.start, self.start + new_length),
                Range::new(new_destination, new_destination + new_length),
            ))
        } else if let Some(new_end_destination) = range.get(self.end - 1) {
            let new_length = self.end - range.source;
            assert!(new_length > 0, "Failed mapping {self:?} in {range:?}");
            let new_destination = new_end_destination - new_length;

            Some((
                Range::new(self.end - new_length, self.end + 1),
                Range::new(new_destination, new_end_destination),
            ))
        } else if self.start < range.source && (self.end) > (range.source + range.length) {
            let new_start = range.inv_get(range.destination).unwrap();
            Some((
                Range::new(new_start, new_start + range.length),
                Range::new(range.destination, range.destination + range.length),
            ))
        } else {
            None
        }
    }

    fn intersects(&self, other: &Range) -> bool {
        self.start <= other.start && other.start < self.end
    }

    fn minus(&self, other: &Range) -> Vec<Range> {
        let mut ranges = Vec::new();
        let self_range = Range::new(self.start, self.end);
        let other_range = Range::new(other.start, other.end);

        if self_range.intersects(&other_range) {
            if self.start < other.start {
                ranges.push(Range::new(self.start, other.start));
            }

            if self.end > other.end {
                ranges.push(Range::new(other.end, self.end));
            }
        } else {
            ranges.push(self.clone());
        }

        ranges
    }
}

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Clone)]
struct RangeMap {
    source: usize,
    destination: usize,
    length: usize,
}

impl RangeMap {
    fn new(source: usize, destination: usize, length: usize) -> Self {
        RangeMap {
            source,
            destination,
            length,
        }
    }

    fn try_merge(&self, other: &RangeMap) -> Option<RangeMap> {
        if other.source == self.source + self.length
            && self.destination + self.length == other.destination
        {
            // These two ranges are adjacent so safe to merge.
            let new_length = self.length + other.length;
            return Some(RangeMap::new(self.source, self.destination, new_length));
        } else if self.source <= other.source
            && other.source < self.source + self.length
            && self
                .get(other.source)
                .map(|v| v == other.destination)
                .unwrap_or(false)
        {
            // These two ranges map to the same range so safe to merge.
            let new_length =
                (self.source + self.length).max(other.source + other.length) - self.source;
            return Some(RangeMap::new(self.source, self.destination, new_length));
        }
        None
    }

    fn get(&self, value: usize) -> Option<usize> {
        if self.source <= value && value < (self.source + self.length) {
            // println!("{}: {:?}", value, ranges[i - 1]);
            Some(value + self.destination - self.source)
        } else {
            None
        }
    }

    fn inv_get(&self, value: usize) -> Option<usize> {
        if self.destination <= value && value < (self.destination + self.length) {
            // println!("{}: {:?}", value, ranges[i - 1]);
            Some(value + self.source - self.destination)
        } else {
            None
        }
    }
}

#[cfg(test)]
mod test_range_map {
    use super::*;

    #[test]
    fn test_minus() {
        let a = Range { start: 0, end: 10 };

        let b = Range { start: 5, end: 10 };

        assert_eq!(a.minus(&b), vec![Range { start: 0, end: 5 },]);

        // // B is fully within A
        // let a = RangeMap {
        //     source: 0,
        //     destination: 100,
        //     length: 20,
        // };

        // let b = RangeMap {
        //     source: 5,
        //     destination: 5,
        //     length: 5,
        // };

        // assert_eq!(
        //     a.minus(&b),
        //     vec![
        //         RangeMap {
        //             source: 0,
        //             destination: 100,
        //             length: 5
        //         },
        //         RangeMap {
        //             source: 10,
        //             destination: 105,
        //             length: 10
        //         }
        //     ]
        // );

        // let a = RangeMap {
        //     source: 0,
        //     destination: 100,
        //     length: 20,
        // };

        // // No overlap
        // let b = RangeMap {
        //     source: 50,
        //     destination: 5,
        //     length: 5,
        // };

        // assert_eq!(
        //     a.minus(&b),
        //     vec![RangeMap {
        //         source: 0,
        //         destination: 100,
        //         length: 20
        //     },]
        // );
    }

    #[test]
    fn test_map_onto_range_map() {
        // let a = Range { start: 46, end: 57 };
        // let b = RangeMap {
        //     source: 56,
        //     destination: 60,
        //     length: 37,
        // };

        // assert_eq!(
        //     a.map_onto_rangemap(&b),
        //     Some((Range { start: 46, end: 56 }, Range { start: 60, end: 61 }))
        // );
    }

    // #[test]
    // fn test_map_onto_range_map() {
    //     // 1
    //     let a = RangeMap {
    //         source: 10,
    //         destination: 10,
    //         length: 5,
    //     };
    //     let b = RangeMap {
    //         source: 0,
    //         destination: 200,
    //         length: 20,
    //     };
    //     assert_eq!(
    //         a.map_onto_rangemap(&b),
    //         Some(RangeMap {
    //             source: 10,
    //             destination: 210,
    //             length: 5
    //         })
    //     );

    //     // 2
    //     let a = RangeMap {
    //         source: 0,
    //         destination: 10,
    //         length: 15,
    //     };
    //     let b = RangeMap {
    //         source: 20,
    //         destination: 210,
    //         length: 10,
    //     };
    //     assert_eq!(
    //         a.map_onto_rangemap(&b),
    //         Some(RangeMap {
    //             source: 20,
    //             destination: 210,
    //             length: 5
    //         })
    //     );

    //     // 3
    //     let a = RangeMap {
    //         source: 10,
    //         destination: 10,
    //         length: 50,
    //     };
    //     let b = RangeMap {
    //         source: 0,
    //         destination: 200,
    //         length: 20,
    //     };
    //     assert_eq!(
    //         a.map_onto_rangemap(&b),
    //         Some(RangeMap {
    //             source: 10,
    //             destination: 210,
    //             length: 10
    //         })
    //     );

    //     // 4
    //     let a = RangeMap {
    //         source: 0,
    //         destination: 10,
    //         length: 50,
    //     };
    //     let b = RangeMap {
    //         source: 20,
    //         destination: 210,
    //         length: 10,
    //     };
    //     assert_eq!(
    //         a.map_onto_rangemap(&b),
    //         Some(RangeMap {
    //             source: 20,
    //             destination: 210,
    //             length: 10
    //         })
    //     );
    // }

    #[test]
    fn test_try_merge() {
        let a = RangeMap {
            source: 0,
            destination: 1520731987,
            length: 239660433,
        };
        let b = RangeMap {
            source: 239660433,
            destination: 1760392420,
            length: 73127385,
        };

        assert_eq!(
            a.try_merge(&b),
            Some(RangeMap {
                source: 0,
                destination: 1520731987,
                length: 239660433 + 73127385
            })
        );
    }
}

pub fn star_one(mut input: impl BufRead) -> String {
    let mut str = String::new();
    input.read_to_string(&mut str).expect("cannot read string");

    let mut sections = str.split("\n\n");

    let needed_seeds = {
        let line = sections.next().unwrap();

        line.split_once(": ")
            .unwrap()
            .1
            .split_whitespace()
            .map(|s| s.parse::<usize>().unwrap())
            .collect::<Vec<_>>()
    };

    let mut conversion_map = HashMap::new();

    let mut type_mapping = HashMap::new();

    for section in sections {
        let mut ranges = Vec::new();

        let mut lines = section.lines();
        let line = lines.next().unwrap();
        let line = line.replace("-to-", "-");
        let line = line.replace(" map:", "");

        let (source, destination) = line.split_once('-').unwrap();
        let source = source.to_string();
        let destination = destination.to_string();

        type_mapping.insert(source.clone(), destination.clone());

        for line in lines {
            let mut parts = line.split_whitespace();
            let destination_value = parts.next().unwrap().parse::<usize>().unwrap();
            let source_value = parts.next().unwrap().parse::<usize>().unwrap();
            let range = parts.next().unwrap().parse::<usize>().unwrap();

            ranges.push(RangeMap::new(source_value, destination_value, range));
        }
        ranges.sort_by_key(|range| range.source);
        conversion_map.insert((source, destination), ranges);
    }

    needed_seeds
        .iter()
        .map(|&seed| {
            let mut value_type = "seed".to_string();
            let mut value = seed;

            while value_type != "location" {
                let new_value_type = type_mapping.get(&value_type).unwrap().clone();

                let ranges = conversion_map
                    .get(&(value_type.to_string(), new_value_type.clone()))
                    .unwrap();

                let new_value = match ranges.binary_search_by_key(&value, |range| range.source) {
                    Ok(i) => ranges[i].destination,
                    Err(0) => value,
                    Err(i) => ranges[i - 1].get(value).unwrap_or(value),
                };

                value = new_value;
                value_type = new_value_type;
            }

            value
        })
        .min()
        .unwrap()
        .to_string()
}

fn collapse_ranges(mut ranges: Vec<Range>) -> Vec<Range> {
    ranges.sort_by_key(|range| Reverse(range.start));

    let mut final_ranges = Vec::new();
    final_ranges.push(ranges.pop().unwrap());

    // Collapse any overlapping ranges.
    while let Some(range) = ranges.pop() {
        if final_ranges.last().unwrap().start <= range.start
            && range.start <= final_ranges.last().unwrap().end
        {
            let mut d = final_ranges.pop().unwrap();
            d.end = final_ranges.last().unwrap().end.max(range.end);

            final_ranges.push(d);
        } else {
            final_ranges.push(range)
        }
    }

    // println!("{:?}", final_ranges);
    final_ranges
}

fn collapse_range_maps(mut ranges: Vec<RangeMap>) -> Vec<RangeMap> {
    ranges.sort_by_key(|range| Reverse(range.source));

    let mut final_ranges = Vec::new();
    final_ranges.push(ranges.pop().unwrap());

    // Collapse any overlapping ranges.
    while let Some(range) = ranges.pop() {
        let previous_range = final_ranges.pop().unwrap();
        if let Some(range) = previous_range.try_merge(&range) {
            final_ranges.push(range);
        } else {
            final_ranges.push(previous_range);
            final_ranges.push(range);
        }
    }

    final_ranges
}

pub fn star_two(mut input: impl BufRead) -> String {
    let mut str = String::new();
    input.read_to_string(&mut str).expect("cannot read string");

    let mut sections = str.split("\n\n");

    let needed_seeds = {
        let line = sections.next().unwrap();

        let ranges = line
            .split_once(": ")
            .unwrap()
            .1
            .split_whitespace()
            .map(|s| s.parse::<usize>().unwrap())
            .collect::<Vec<_>>();

        let mut needed_seeds = ranges[..]
            .chunks(2)
            .map(|chunk| {
                let mut range = chunk.iter().cloned();
                let start = range.next().unwrap();
                let count = range.next().unwrap();
                Range::new(start, start + count)
            })
            .collect::<Vec<_>>();

        needed_seeds.sort_by_key(|range| Reverse(range.start));

        collapse_ranges(needed_seeds)
    };

    let mut rev_conversion_map = HashMap::new();
    let mut conversion_map = HashMap::new();

    let mut rev_type_mapping = HashMap::new();
    let mut type_mapping = HashMap::new();

    let mut enum_mapping = HashMap::new();

    enum_mapping.insert("seed", 0);

    for section in sections {
        let mut rev_ranges = Vec::new();
        let mut ranges = Vec::new();

        let mut lines = section.lines();
        let line = lines.next().unwrap();
        let (source, other) = line.split_once('-').unwrap();

        let (_, other) = other.split_once("to-").unwrap();
        let (destination, _) = other.split_once(' ').unwrap();

        rev_type_mapping.insert(destination, source);
        type_mapping.insert(source, destination);

        for line in lines {
            let mut parts = line.split_whitespace();
            let destination_value = parts.next().unwrap().parse::<usize>().unwrap();
            let source_value = parts.next().unwrap().parse::<usize>().unwrap();
            let range = parts.next().unwrap().parse::<usize>().unwrap();

            // Swap the source and destination values.
            rev_ranges.push(RangeMap::new(destination_value, source_value, range));
            ranges.push(RangeMap::new(source_value, destination_value, range));
        }

        let mut rev_ranges = collapse_range_maps(rev_ranges);

        // println!("{:?}", rev_ranges);

        // panic!();

        rev_ranges.sort_by_key(|range| range.source);
        ranges.sort_by_key(|range| range.source);

        rev_conversion_map.insert((destination, source), rev_ranges);
        conversion_map.insert((source, destination), ranges);
    }

    let mut current_type = "seed";
    let mut current_ranges = needed_seeds;

    while current_type != "location" {
        let next_type = type_mapping.get(current_type).unwrap();
        let range_maps = conversion_map.get(&(current_type, next_type)).unwrap();

        let mut stack = current_ranges;
        let mut mapped_ranges = vec![];

        while let Some(current_range) = stack.pop() {
            let mut found = false;
            for mapping_range in range_maps {
                if let Some((previous_mapped, mapped)) =
                    current_range.map_onto_rangemap(mapping_range)
                {
                    found = true;
                    let minus = current_range.minus(&previous_mapped);

                    assert!(!mapped.is_empty());
                    assert!(minus.iter().all(|range| !range.is_empty()));

                    mapped_ranges.push(mapped);

                    stack.extend(minus);
                    break;
                }
            }
            if !found {
                mapped_ranges.push(current_range);
            }
        }
        current_type = next_type;
        current_ranges = mapped_ranges;
    }

    current_ranges
        .iter()
        .min_by_key(|range| range.start)
        .unwrap()
        .start
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
                b"seeds: 79 14 55 13

seed-to-soil map:
50 98 2
52 50 48

soil-to-fertilizer map:
0 15 37
37 52 2
39 0 15

fertilizer-to-water map:
49 53 8
0 11 42
42 0 7
57 7 4

water-to-light map:
88 18 7
18 25 70

light-to-temperature map:
45 77 23
81 45 19
68 64 13

temperature-to-humidity map:
0 69 1
1 0 69

humidity-to-location map:
60 56 37
56 93 4"
            )),
            "35"
        );
    }

    #[test]
    fn test_star_two() {
        assert_eq!(
            star_two(Cursor::new(
                b"seeds: 79 14 55 13

seed-to-soil map:
50 98 2
52 50 48

soil-to-fertilizer map:
0 15 37
37 52 2
39 0 15

fertilizer-to-water map:
49 53 8
0 11 42
42 0 7
57 7 4

water-to-light map:
88 18 7
18 25 70

light-to-temperature map:
45 77 23
81 45 19
68 64 13

temperature-to-humidity map:
0 69 1
1 0 69

humidity-to-location map:
60 56 37
56 93 4"
            )),
            "46"
        );
    }
}
