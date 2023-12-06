use std::{cmp::Reverse, collections::HashMap, io::BufRead, time::Instant};

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

            ranges.push((source_value, destination_value, range));
        }
        ranges.sort_by_key(|(source_value, _, _)| *source_value);
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

                // println!("{:?}", ranges);
                // println!("new_value_type: {}", new_value_type);

                // println!("ranges: {:?}", ranges);

                let new_value = {
                    match ranges.binary_search_by_key(&value, |(source_value, _, _)| *source_value)
                    {
                        Ok(i) => ranges[i].1,
                        Err(i) => {
                            // println!("{}: {:?}", value, i);
                            if i == 0 {
                                value
                            } else {
                                // println!("{}: {:?}", value, ranges[i - 1]);
                                if ranges[i - 1].0 < value
                                    && value < (ranges[i - 1].0 + ranges[i - 1].2)
                                {
                                    // println!("{}: {:?}", value, ranges[i - 1]);
                                    value + ranges[i - 1].1 - ranges[i - 1].0
                                } else {
                                    value
                                }
                            }
                        }
                    }
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

fn collapse_ranges(mut ranges: Vec<(usize, usize)>) -> Vec<(usize, usize)> {
    ranges.sort_by_key(|(start, _)| Reverse(*start));

    let mut final_ranges = Vec::new();
    final_ranges.push(ranges.pop().unwrap());

    // Collapse any overlapping ranges.
    while let Some(range) = ranges.pop() {
        if final_ranges.last().unwrap().0 <= range.0 && range.0 <= final_ranges.last().unwrap().1 {
            let mut d = final_ranges.pop().unwrap();
            d.1 = final_ranges.last().unwrap().1.max(range.1);

            final_ranges.push(d);
        } else {
            final_ranges.push(range)
        }
    }
    final_ranges
}

pub fn star_two(mut input: impl BufRead) -> String {
    let mut str = String::new();
    input.read_to_string(&mut str).expect("cannot read string");

    let start = Instant::now();

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
                (start, start + count)
            })
            .collect::<Vec<_>>();

        needed_seeds.sort_by_key(|(start, _)| Reverse(*start));

        collapse_ranges(needed_seeds)
    };

    let mut rev_conversion_map = HashMap::new();
    let mut rev_type_mapping = HashMap::new();
    let mut enum_mapping = HashMap::new();

    enum_mapping.insert("seed", 0);

    for section in sections {
        let mut rev_ranges = Vec::new();

        let mut lines = section.lines();
        let line = lines.next().unwrap();
        let (source, other) = line.split_once('-').unwrap();

        let (_, other) = other.split_once("to-").unwrap();
        let (destination, _) = other.split_once(' ').unwrap();

        rev_type_mapping.insert(destination, source);

        for line in lines {
            let mut parts = line.split_whitespace();
            let destination_value = parts.next().unwrap().parse::<usize>().unwrap();
            let source_value = parts.next().unwrap().parse::<usize>().unwrap();
            let range = parts.next().unwrap().parse::<usize>().unwrap();

            rev_ranges.push((source_value, destination_value, range));
        }

        rev_ranges.sort_by_key(|(_, destination_value, _)| *destination_value);

        rev_conversion_map.insert((destination, source), rev_ranges);
    }

    println!("Finished: {:?}", start.elapsed());

    (0..)
        .find(|&location| {
            let mut value_type = "location";
            let mut value = location;

            while value_type != "seed" {
                let new_value_type = rev_type_mapping.get(value_type).unwrap();

                let ranges = rev_conversion_map
                    .get(&(value_type, new_value_type))
                    .unwrap();

                let new_value = match ranges
                    .binary_search_by_key(&value, |(_, destination_value, _)| *destination_value)
                {
                    Ok(i) => ranges[i].0,
                    Err(i) => {
                        if i != 0
                            && ranges[i - 1].1 < value
                            && value < (ranges[i - 1].1 + ranges[i - 1].2)
                        {
                            value + ranges[i - 1].0 - ranges[i - 1].1
                        } else {
                            value
                        }
                    }
                };

                value = new_value;
                value_type = new_value_type;
            }

            match needed_seeds.binary_search_by_key(&value, |(start, _length)| *start) {
                Ok(_i) => true,
                Err(i) => i != 0 && needed_seeds[i - 1].0 < value && value < needed_seeds[i - 1].1,
            }
        })
        .unwrap()
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
