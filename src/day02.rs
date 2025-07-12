use std::{collections::HashMap, io::BufRead};

pub fn star_one(input: impl BufRead) -> String {
    let mut max = HashMap::new();
    max.insert("red".to_owned(), 12);
    max.insert("green".to_owned(), 13);
    max.insert("blue".to_owned(), 14);

    input
        .lines()
        .map(|line| {
            let line = line.unwrap();
            let (x, sets) = line.split_once(':').unwrap();
            let game = sets
                .split(';')
                .map(|set| {
                    set.split(',')
                        .map(|cube| {
                            cube.trim()
                                .split_once(' ')
                                .map(|(x, colour)| (colour.to_string(), x.parse::<i32>().unwrap()))
                                .unwrap()
                        })
                        .collect::<HashMap<String, i32>>()
                })
                .collect::<Vec<_>>();
            // dbg!(&game);
            let id = x.replace("Game ", "").parse::<u64>().unwrap();
            (id, game)
        })
        .filter(|(_id, sets)| {
            sets.iter().all(|cubes| {
                cubes.iter().all(|(colour, x)| {
                    max.get(colour)
                        .map(|m| m >= x)
                        .unwrap_or_else(|| panic!("Could not find {colour}"))
                })
            })
        })
        .map(|(id, _)| id)
        .sum::<u64>()
        .to_string()
}

pub fn star_two(input: impl BufRead) -> String {
    input
        .lines()
        .map(|line| {
            let line = line.unwrap();
            let (x, sets) = line.split_once(':').unwrap();
            let game = sets
                .split(';')
                .map(|set| {
                    set.split(',')
                        .map(|cube| {
                            cube.trim()
                                .split_once(' ')
                                .map(|(x, colour)| (colour.to_string(), x.parse::<i32>().unwrap()))
                                .unwrap()
                        })
                        .collect::<HashMap<String, i32>>()
                })
                .collect::<Vec<_>>();
            // dbg!(&game);
            let id = x.replace("Game ", "").parse::<u64>().unwrap();
            (id, game)
        })
        .map(|(_id, sets)| {
            let mut max = HashMap::new();

            sets.into_iter().for_each(|cubes| {
                cubes.into_iter().for_each(|(colour, x)| {
                    let entry = max.entry(colour).or_insert(x);
                    if x > *entry {
                        *entry = x;
                    }
                })
            });
            max
        })
        .map(|max| max[&"red".to_string()] * max[&"green".to_string()] * max[&"blue".to_string()])
        .sum::<i32>()
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
                b"Game 1: 3 blue, 4 red; 1 red, 2 green, 6 blue; 2 green
Game 2: 1 blue, 2 green; 3 green, 4 blue, 1 red; 1 green, 1 blue
Game 3: 8 green, 6 blue, 20 red; 5 blue, 4 red, 13 green; 5 green, 1 red
Game 4: 1 green, 3 red, 6 blue; 3 green, 6 red; 3 green, 15 blue, 14 red
Game 5: 6 red, 1 blue, 3 green; 2 blue, 1 red, 2 green"
            )),
            "8"
        );
    }

    #[test]
    fn test_star_two() {
        assert_eq!(
            star_two(Cursor::new(
                b"Game 1: 3 blue, 4 red; 1 red, 2 green, 6 blue; 2 green
Game 2: 1 blue, 2 green; 3 green, 4 blue, 1 red; 1 green, 1 blue
Game 3: 8 green, 6 blue, 20 red; 5 blue, 4 red, 13 green; 5 green, 1 red
Game 4: 1 green, 3 red, 6 blue; 3 green, 6 red; 3 green, 15 blue, 14 red
Game 5: 6 red, 1 blue, 3 green; 2 blue, 1 red, 2 green"
            )),
            "2286"
        );
    }
}
