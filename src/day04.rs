use std::{
    collections::{HashMap, HashSet, VecDeque},
    io::BufRead,
};

pub fn star_one(input: impl BufRead) -> String {
    input
        .lines()
        .map(|line| {
            let line = line.unwrap();

            let (_card, numbers) = line.split_once(':').unwrap();
            let (winning, have_numbers) = numbers.split_once('|').unwrap();

            let winning = winning
                .split_whitespace()
                .map(|x| x.parse::<u32>().unwrap())
                .collect::<HashSet<_>>();

            let have_numbers = have_numbers
                .split_whitespace()
                .map(|x| x.parse::<u32>().unwrap())
                .collect::<HashSet<_>>();

            let count = winning.intersection(&have_numbers).count();
            if count == 0 {
                0
            } else {
                2usize.pow(count as u32 - 1)
            }
        })
        .sum::<usize>()
        .to_string()
}

pub fn star_two(input: impl BufRead) -> String {
    let cards: Vec<_> = input
        .lines()
        .map(|line| {
            let line = line.unwrap();

            let (card, numbers) = line.split_once(':').unwrap();

            let card = {
                card.replace("Card ", "")
                    .trim()
                    .parse::<usize>()
                    .unwrap_or_else(|_e| panic!("Could not parse {}", card))
            };

            let (winning, have_numbers) = numbers.split_once('|').unwrap();
            let winning = winning
                .split_whitespace()
                .map(|x| x.parse::<u32>().unwrap())
                .collect::<HashSet<_>>();

            let have_numbers = have_numbers
                .split_whitespace()
                .map(|x| x.parse::<u32>().unwrap())
                .collect::<HashSet<_>>();

            (card, winning, have_numbers)
        })
        .map(|(card, winning, have_numbers)| (card, winning.intersection(&have_numbers).count()))
        .collect();

    let mut playing_cards: VecDeque<_> = cards.iter().collect();

    let mut final_playing_cards = HashMap::new();

    while let Some((card, count)) = playing_cards.pop_front() {
        *final_playing_cards.entry(*card).or_insert(0) += 1;

        if count > &0 {
            playing_cards.extend(cards[*card..(*card + count)].iter());
        }
    }

    final_playing_cards.values().sum::<usize>().to_string()
}

#[cfg(test)]
mod tests {
    use super::{star_one, star_two};
    use std::io::Cursor;

    #[test]
    fn test_star_one() {
        assert_eq!(
            star_one(Cursor::new(
                b"Card 1: 41 48 83 86 17 | 83 86  6 31 17  9 48 53
Card 2: 13 32 20 16 61 | 61 30 68 82 17 32 24 19
Card 3:  1 21 53 59 44 | 69 82 63 72 16 21 14  1
Card 4: 41 92 73 84 69 | 59 84 76 51 58  5 54 83
Card 5: 87 83 26 28 32 | 88 30 70 12 93 22 82 36
Card 6: 31 18 13 56 72 | 74 77 10 23 35 67 36 11"
            )),
            "13"
        );
    }

    #[test]
    fn test_star_two() {
        assert_eq!(
            star_two(Cursor::new(
                b"Card 1: 41 48 83 86 17 | 83 86  6 31 17  9 48 53
Card 2: 13 32 20 16 61 | 61 30 68 82 17 32 24 19
Card 3:  1 21 53 59 44 | 69 82 63 72 16 21 14  1
Card 4: 41 92 73 84 69 | 59 84 76 51 58  5 54 83
Card 5: 87 83 26 28 32 | 88 30 70 12 93 22 82 36
Card 6: 31 18 13 56 72 | 74 77 10 23 35 67 36 11"
            )),
            "30"
        );
    }
}
