use itertools::Itertools;
use std::io::BufRead;

pub fn star_one(input: impl BufRead) -> String {
    let mut galaxy_positions = input
        .lines()
        .enumerate()
        .flat_map(|(y, line)| {
            let line = line.unwrap();
            line.chars()
                .enumerate()
                .filter_map(move |(x, c)| {
                    if c == '#' {
                        Some((y as isize, x as isize))
                    } else {
                        None
                    }
                })
                .collect::<Vec<_>>()
        })
        .collect::<Vec<_>>();

    let mut galaxy_positions = {
        // Expand rows
        // First sort the positions by y
        galaxy_positions.sort_by_key(|&(y, _)| y);

        let mut new_positions = Vec::with_capacity(galaxy_positions.len());

        // Add the first one as we know that it does not need expanding. (Might get a bit confusing 😅)
        new_positions.push(galaxy_positions[0]);

        let mut current_expansion = 0;

        for i in 1..galaxy_positions.len() {
            let galaxy = galaxy_positions[i];
            let last_galaxy = galaxy_positions[i - 1];
            if galaxy.0 - last_galaxy.0 > 1 {
                let expansion = galaxy.0 - last_galaxy.0 - 1;
                current_expansion += expansion;
            }
            // Expand the number of rows empty * 2

            new_positions.push((galaxy.0 + current_expansion, galaxy.1));
        }

        new_positions
    };

    let galaxy_positions = {
        // Expand rows
        // First sort the positions by y (since we `pop` form the end we do it in reverse order)
        galaxy_positions.sort_by_key(|&(_, x)| x);

        let mut new_positions = Vec::with_capacity(galaxy_positions.len());

        // Add the first one as we know that it does not need expanding. (Might get a bit confusing 😅)
        new_positions.push(galaxy_positions[0]);

        let mut current_expansion = 0;

        for i in 1..galaxy_positions.len() {
            let galaxy = galaxy_positions[i];
            let last_galaxy = galaxy_positions[i - 1];
            if galaxy.1 - last_galaxy.1 > 1 {
                // Expand the number of rows empty * 2
                let expansion = galaxy.1 - last_galaxy.1 - 1;
                current_expansion += expansion;
            }

            new_positions.push((galaxy.0, galaxy.1 + current_expansion));
        }

        new_positions
    };

    galaxy_positions
        .into_iter()
        .combinations(2)
        .map(|pair| (pair[0].1 - pair[1].1).abs() + (pair[0].0 - pair[1].0).abs())
        .sum::<isize>()
        .to_string()
}

pub fn star_two(input: impl BufRead) -> String {
    let factor = 1000000;
    let mut galaxy_positions = input
        .lines()
        .enumerate()
        .flat_map(|(y, line)| {
            let line = line.unwrap();
            line.chars()
                .enumerate()
                .filter_map(move |(x, c)| {
                    if c == '#' {
                        Some((y as isize, x as isize))
                    } else {
                        None
                    }
                })
                .collect::<Vec<_>>()
        })
        .collect::<Vec<_>>();

    let mut galaxy_positions = {
        // Expand rows
        // First sort the positions by y
        galaxy_positions.sort_by_key(|&(y, _)| y);

        let mut new_positions = Vec::with_capacity(galaxy_positions.len());

        // Add the first one as we know that it does not need expanding. (Might get a bit confusing 😅)
        new_positions.push(galaxy_positions[0]);

        let mut current_expansion = 0;

        for i in 1..galaxy_positions.len() {
            let galaxy = galaxy_positions[i];
            let last_galaxy = galaxy_positions[i - 1];
            if galaxy.0 - last_galaxy.0 > 1 {
                let expansion = (galaxy.0 - last_galaxy.0 - 1) * (factor - 1);
                current_expansion += expansion;
            }
            // Expand the number of rows empty * 2

            new_positions.push((galaxy.0 + current_expansion, galaxy.1));
        }

        new_positions
    };

    let galaxy_positions = {
        // Expand rows
        // First sort the positions by y (since we `pop` form the end we do it in reverse order)
        galaxy_positions.sort_by_key(|&(_, x)| x);

        let mut new_positions = Vec::with_capacity(galaxy_positions.len());

        // Add the first one as we know that it does not need expanding. (Might get a bit confusing 😅)
        new_positions.push(galaxy_positions[0]);

        let mut current_expansion = 0;

        for i in 1..galaxy_positions.len() {
            let galaxy = galaxy_positions[i];
            let last_galaxy = galaxy_positions[i - 1];
            if galaxy.1 - last_galaxy.1 > 1 {
                // Expand the number of rows empty * 2
                let expansion = (galaxy.1 - last_galaxy.1 - 1) * (factor - 1);
                current_expansion += expansion;
            }

            new_positions.push((galaxy.0, galaxy.1 + current_expansion));
        }

        new_positions
    };

    galaxy_positions
        .into_iter()
        .combinations(2)
        .map(|pair| (pair[0].1 - pair[1].1).abs() + (pair[0].0 - pair[1].0).abs())
        .sum::<isize>()
        .to_string()
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::io::Cursor;

    #[test]
    fn test_star_one() {
        assert_eq!(
            star_one(Cursor::new(
                b"...#......
.......#..
#.........
..........
......#...
.#........
.........#
..........
.......#..
#...#....."
            )),
            "374"
        );

        assert_eq!(
            star_one(Cursor::new(
                b"...#...
......#"
            )),
            "6"
        );
    }

    #[test]
    fn test_star_two() {}
}
