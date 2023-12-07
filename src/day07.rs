use itertools::Itertools;
use std::io::BufRead;

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Clone, Copy)]
enum Card {
    Joker,
    Two,
    Three,
    Four,
    Five,
    Six,
    Seven,
    Eight,
    Nine,
    Ten,
    Jack,
    Queen,
    King,
    Ace,
}

static JOKER_REPLACEMENTS: [Card; 13] = [
    Card::Ace,
    Card::King,
    Card::Queen,
    Card::Jack,
    Card::Ten,
    Card::Nine,
    Card::Eight,
    Card::Seven,
    Card::Six,
    Card::Five,
    Card::Four,
    Card::Three,
    Card::Two,
];

#[derive(PartialEq, Eq, Debug)]
enum HandType {
    FiveOfAKind,
    FourOfAKind,
    FullHouse,
    Three,
    TwoPair,
    OnePair,
    None,
}

impl PartialOrd for HandType {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for HandType {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        match (self, other) {
            (HandType::FiveOfAKind, HandType::FiveOfAKind) => std::cmp::Ordering::Equal,
            (HandType::FiveOfAKind, _) => std::cmp::Ordering::Greater,
            // Four Of A Kind
            (HandType::FourOfAKind, HandType::FourOfAKind) => std::cmp::Ordering::Equal,
            (HandType::FourOfAKind, HandType::FiveOfAKind) => std::cmp::Ordering::Less,
            (HandType::FourOfAKind, _) => std::cmp::Ordering::Greater,
            // Full House
            (HandType::FullHouse, HandType::FullHouse) => std::cmp::Ordering::Equal,
            (HandType::FullHouse, HandType::FiveOfAKind) => std::cmp::Ordering::Less,
            (HandType::FullHouse, HandType::FourOfAKind) => std::cmp::Ordering::Less,
            (HandType::FullHouse, _) => std::cmp::Ordering::Greater,
            // Three
            (HandType::Three, HandType::Three) => std::cmp::Ordering::Equal,
            (HandType::Three, HandType::FiveOfAKind) => std::cmp::Ordering::Less,
            (HandType::Three, HandType::FourOfAKind) => std::cmp::Ordering::Less,
            (HandType::Three, HandType::FullHouse) => std::cmp::Ordering::Less,
            (HandType::Three, _) => std::cmp::Ordering::Greater,

            //Two Pair
            (HandType::TwoPair, HandType::TwoPair) => std::cmp::Ordering::Equal,
            (HandType::TwoPair, HandType::FiveOfAKind) => std::cmp::Ordering::Less,
            (HandType::TwoPair, HandType::FourOfAKind) => std::cmp::Ordering::Less,
            (HandType::TwoPair, HandType::FullHouse) => std::cmp::Ordering::Less,
            (HandType::TwoPair, HandType::Three) => std::cmp::Ordering::Less,
            (HandType::TwoPair, _) => std::cmp::Ordering::Greater,

            // One Pair
            (HandType::OnePair, HandType::OnePair) => std::cmp::Ordering::Equal,
            (HandType::OnePair, HandType::FiveOfAKind) => std::cmp::Ordering::Less,
            (HandType::OnePair, HandType::FourOfAKind) => std::cmp::Ordering::Less,
            (HandType::OnePair, HandType::FullHouse) => std::cmp::Ordering::Less,
            (HandType::OnePair, HandType::Three) => std::cmp::Ordering::Less,
            (HandType::OnePair, HandType::TwoPair) => std::cmp::Ordering::Less,
            (HandType::OnePair, HandType::None) => std::cmp::Ordering::Greater,
            // None
            (HandType::None, HandType::None) => std::cmp::Ordering::Equal,
            (HandType::None, _) => std::cmp::Ordering::Less,
        }
    }
}

fn get_hand_type(cards: &[Card]) -> HandType {
    if cards[0] == cards[4] {
        HandType::FiveOfAKind
    } else if cards[0] == cards[3] || cards[1] == cards[4] {
        HandType::FourOfAKind
    } else if (cards[0] == cards[2] && cards[3] == cards[4])
        || (cards[0] == cards[1] && cards[2] == cards[4])
    {
        HandType::FullHouse
    } else if cards[0] == cards[2] || cards[1] == cards[3] || cards[2] == cards[4] {
        HandType::Three
    } else if (cards[0] == cards[1] && (cards[2] == cards[3] || cards[3] == cards[4]))
        || (cards[1] == cards[2] && cards[3] == cards[4])
    {
        HandType::TwoPair
    } else if cards[0] == cards[1]
        || cards[1] == cards[2]
        || cards[2] == cards[3]
        || cards[3] == cards[4]
    {
        HandType::OnePair
    } else {
        HandType::None
    }
}

pub fn star_one(input: impl BufRead) -> String {
    let mut hands = input
        .lines()
        .map(|line| {
            let line = line.unwrap();

            let (cards, bid) = line.split_once(' ').unwrap();

            let bid = bid.parse::<usize>().unwrap();

            let cards_input = cards
                .chars()
                .map(|c| match c {
                    '2' => Card::Two,
                    '3' => Card::Three,
                    '4' => Card::Four,
                    '5' => Card::Five,
                    '6' => Card::Six,
                    '7' => Card::Seven,
                    '8' => Card::Eight,
                    '9' => Card::Nine,
                    'T' => Card::Ten,
                    'J' => Card::Jack,
                    'Q' => Card::Queen,
                    'K' => Card::King,
                    'A' => Card::Ace,
                    _ => panic!("Invalid card"),
                })
                .collect::<Vec<_>>();

            let mut cards = cards_input.clone();
            cards.sort();

            let hand_type = get_hand_type(&cards);

            (hand_type, cards_input, bid)
        })
        .collect::<Vec<_>>();

    hands.sort_by(|hand, other| {
        hand.0.cmp(&other.0).then(
            hand.1
                .iter()
                .zip(other.1.iter())
                .find_map(|(a, b)| {
                    let c = a.cmp(b);

                    if c == std::cmp::Ordering::Equal {
                        None
                    } else {
                        Some(c)
                    }
                })
                .unwrap(),
        )
    });
    hands
        .iter()
        .enumerate()
        .map(|(i, hand)| (i + 1) * hand.2)
        .sum::<usize>()
        .to_string()
}

pub fn star_two(input: impl BufRead) -> String {
    let mut hands = input
        .lines()
        .map(|line| {
            let line = line.unwrap();

            let (cards, bid) = line.split_once(' ').unwrap();

            let bid = bid.parse::<usize>().unwrap();

            let cards_input = cards
                .chars()
                .map(|c| match c {
                    '2' => Card::Two,
                    '3' => Card::Three,
                    '4' => Card::Four,
                    '5' => Card::Five,
                    '6' => Card::Six,
                    '7' => Card::Seven,
                    '8' => Card::Eight,
                    '9' => Card::Nine,
                    'T' => Card::Ten,
                    'J' => Card::Joker,
                    'Q' => Card::Queen,
                    'K' => Card::King,
                    'A' => Card::Ace,
                    _ => panic!("Invalid card"),
                })
                .collect::<Vec<_>>();

            // Find all the jokers
            let joker_positions = cards_input
                .iter()
                .enumerate()
                .filter_map(|(i, &card)| if card == Card::Joker { Some(i) } else { None })
                .collect::<Vec<_>>();

            let mut hand_combinations = JOKER_REPLACEMENTS
                .iter()
                .combinations_with_replacement(joker_positions.len())
                .map(|replacements| {
                    let mut cards = cards_input.clone();
                    replacements
                        .iter()
                        .zip(joker_positions.iter())
                        .for_each(|(&replacement, &position)| cards[position] = *replacement);

                    cards.sort();

                    get_hand_type(&cards)
                })
                .collect::<Vec<_>>();

            hand_combinations.sort();

            (
                hand_combinations.into_iter().last().unwrap(),
                cards_input,
                bid,
            )
        })
        .collect::<Vec<_>>();

    hands.sort_by(|hand, other| {
        hand.0.cmp(&other.0).then(
            hand.1
                .iter()
                .zip(other.1.iter())
                .find_map(|(a, b)| match a.cmp(b) {
                    std::cmp::Ordering::Equal => None,
                    c => Some(c),
                })
                .unwrap(),
        )
    });
    hands
        .iter()
        .enumerate()
        .map(|(i, hand)| (i + 1) * hand.2)
        .sum::<usize>()
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
                b"32T3K 765
T55J5 684
KK677 28
KTJJT 220
QQQJA 483"
            )),
            "6440"
        );
    }

    #[test]
    fn test_star_two() {
        assert_eq!(
            star_two(Cursor::new(
                b"32T3K 765
T55J5 684
KK677 28
KTJJT 220
QQQJA 483"
            )),
            "5905"
        );
    }
}
