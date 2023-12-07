use std::{cmp::Ordering, collections:: BTreeMap};


fn main() {
    let input = include_str!("./input.txt");
    let output = part1(input);
    dbg!(output);
}

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
enum Card {
    Digit(u32), T, J, Q, K, A
}

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Clone, Copy)]
enum HandType {
    HighCard,
    OnePair,
    TwoPair,
    ThreeKind,
    FullHouse,
    FourKind,
    FiveKind
}

#[derive(Debug, Eq)]
struct Hand {
    bid: usize,
    cards: Vec<Card>,
    hand_type: HandType
}

impl Ord for Hand {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        match self.hand_type.cmp(&other.hand_type) {
            Ordering::Equal => self.cards.iter().zip(other.cards.iter())
                                   .fold(Ordering::Equal, |ord, (sc, oc)| {
                                       if ord != Ordering::Equal {
                                           ord
                                       } else {
                                           sc.cmp(&oc)
                                       }
                                   }),
            not_equal => not_equal
        }
    }
}

impl PartialOrd for Hand {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        match self.hand_type.partial_cmp(&other.hand_type) {
            Some(Ordering::Equal) => self.cards.iter().zip(other.cards.iter())
                                         .fold(Some(Ordering::Equal), |ord, (sc, oc)| {
                                            if ord != Some(Ordering::Equal) {
                                                ord
                                            } else {
                                                sc.partial_cmp(&oc)
                                            }
                                         }),
            not_equal_or_none => not_equal_or_none
        }
    }
}

impl PartialEq for Hand {
    fn eq(&self, other: &Self) -> bool {
        match self.hand_type.eq(&other.hand_type) {
            true => self.cards.iter().zip(other.cards.iter())
                        .fold(true, |b, (sc, oc)| b && sc.eq(oc)),
            false => false
        }
    }
}

fn parse_card(c: char) -> Card {
    use Card::*;
    match c {
        'A' => A,
        'K' => K,
        'Q' => Q,
        'J' => J,
        'T' => T,
        _ if c.is_digit(10) => Digit(c.to_digit(10).expect("should be a digit")),
        _ => panic!("invalid card: {}", c)
    }
}

fn calculate_hand_type(cards: &Vec<Card>) -> HandType {
    use HandType::*;
    let counts = cards.iter().fold(BTreeMap::new(), |mut map, c| {
                    *map.entry(c).or_insert(0) += 1;
                    map
                });
    counts.values().fold(HighCard, |t, count| {
        match (*count, t) {
            (1, FiveKind) => panic!("too many cards"),
            (1, _) => t,
            (2, HighCard) => OnePair,
            (2, OnePair) => TwoPair,
            (2, ThreeKind) => FullHouse,
            (2, _) => panic!("too many cards"),
            (3, HighCard) => ThreeKind,
            (3, OnePair) => FullHouse,
            (3, _) => panic!("too many cards"),
            (4, HighCard) => FourKind,
            (4, _) => panic!("too many cards"),
            (5, HighCard) => FiveKind,
            (5, _) => panic!("too many cards"),
            _ => panic!("invalid card count")
        }
    })
}

fn parse_cards(line: &str) -> (Vec<Card>, HandType) {
    let cards = line.chars().map(parse_card).collect();
    let hand_type = calculate_hand_type(&cards);
    (cards, hand_type)
}

fn parse_hand(line: &str) -> Hand {
    let mut segments = line.split_whitespace();
    let (cards, hand_type) = parse_cards(segments.next().expect("Should always have cards"));
    let bid = segments.next().expect("should always have a bid").parse().expect("should be parseable");

    Hand { 
        bid,
        cards,
        hand_type
    }
}

fn part1(input: &str) -> usize {
    let mut hands: Vec<_> = input.lines().map(parse_hand).collect();
    hands.sort();
    hands.iter().enumerate().map(|(i, h)| h.bid * (i + 1)).sum()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part1() {
        let result = part1("32T3K 765
        T55J5 684
        KK677 28
        KTJJT 220
        QQQJA 483");
        assert_eq!(result, 6440);
    }

}


