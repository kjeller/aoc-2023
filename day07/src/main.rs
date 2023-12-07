use std::env;
use std::fs::File;
use std::io::BufRead;
use std::io::BufReader;

use strum::IntoEnumIterator;
use strum_macros::Display;
use strum_macros::EnumIter;

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, EnumIter, Display, Clone, Copy)]
enum CardRank {
    _2,
    _3,
    _4,
    _5,
    _6,
    _7,
    _8,
    _9,
    T,
    J,
    Q,
    K,
    A,
}

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Display, Clone, Copy)]
enum ExtCardRank {
    Joker,
    Base(CardRank),
}

impl CardRank {
    fn from_char(c: &char) -> CardRank {
        match c {
            '2' => Self::_2,
            '3' => Self::_3,
            '4' => Self::_4,
            '5' => Self::_5,
            '6' => Self::_6,
            '7' => Self::_7,
            '8' => Self::_8,
            '9' => Self::_9,
            'T' => Self::T,
            'J' => Self::J,
            'Q' => Self::Q,
            'K' => Self::K,
            'A' => Self::A,
            _ => Self::A,
        }
    }

    fn from_char_to_ext(c: &char) -> ExtCardRank {
        ExtCardRank::Base(Self::from_char(c))
    }
}

impl ExtCardRank {
    fn from_char(c: &char) -> ExtCardRank {
        match c {
            'J' => Self::Joker,
            _ => Self::Base(CardRank::from_char(c)),
        }
    }
}

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord)]
enum HandType {
    HighCard,
    OnePair,
    TwoPair,
    ThreeOfAKind,
    FullHouse,
    FourOfAKind,
    FiveOfAKind,
}

impl HandType {
    fn from_card_ranks(cards: &Vec<ExtCardRank>) -> HandType {
        let mut dup = cards.clone();
        dup.sort();
        dup.dedup();
        let unique_c_count = dup.iter().count();
        let dup_count: usize = cards
            .iter()
            .map(|c| cards.iter().filter(|c2| c2 == &c).count() - 1)
            .sum();

        //println!("unique c {}, dup cnt {}", unique_c_count, dup_count);

        match (unique_c_count, dup_count) {
            (2, 12) => Self::FourOfAKind,
            (1, _) => Self::FiveOfAKind,
            (2, _) => Self::FullHouse,
            (3, 4) => Self::TwoPair,
            (3, _) => Self::ThreeOfAKind,
            (4, _) => Self::OnePair,
            (_, _) => Self::HighCard,
        }
    }
}

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord)]
struct Hand {
    hand_type: HandType,
    hand: Vec<ExtCardRank>,
    bid: u64,
}

fn parse_to_string() -> Vec<String> {
    let file = File::open("input.txt").expect("not found");
    let reader = BufReader::new(file);
    reader.lines().map(Result::unwrap).collect::<Vec<String>>()
}

fn parse(card_fun: fn(c: &char) -> ExtCardRank) -> Vec<Hand> {
    parse_to_string()
        .iter()
        .map(|f| {
            let chars: Vec<char> = f.chars().collect();
            let hand = chars[0..5].iter().map(|c| card_fun(&c)).collect();
            Hand {
                hand_type: HandType::from_card_ranks(&hand),
                hand: hand,
                bid: f
                    .split_once(char::is_whitespace)
                    .unwrap()
                    .1
                    .parse::<u64>()
                    .ok()
                    .unwrap(),
            }
        })
        .collect()
}

fn part_1() -> u64 {
    let mut hand = parse(CardRank::from_char_to_ext);
    hand.sort();
    hand.iter()
        .enumerate()
        .map(|(i, h)| h.bid * (i + 1) as u64)
        .sum()
}

fn part_2() -> u64 {
    let mut hand = parse(ExtCardRank::from_char);
    let mut jokercards: Vec<Vec<ExtCardRank>> = vec![];
    hand.iter_mut().for_each(|h| {
        CardRank::iter().for_each(|c1| {
            let jokerhand = h
                .hand
                .iter()
                .map(|c2| {
                    if ExtCardRank::Base(c1) == *c2 {
                        ExtCardRank::Base(c1.to_owned())
                    } else {
                        c2.to_owned()
                    }
                })
                .collect::<Vec<ExtCardRank>>();
            jokercards.push(jokerhand);
        });
        jokercards.sort();
        //println!(">> replacing {:?}", h.hand);
        h.hand = jokercards.first().unwrap().clone();
        println!("handtype1 {:?}", h.hand_type);
        h.hand_type = HandType::from_card_ranks(&h.hand);

        println!("handtype2 {:?}", h.hand_type);
        println!(">> with {:?}", h.hand);
        jokercards = vec![];
    });

    hand.sort();
    hand.iter()
        .enumerate()
        .map(|(i, h)| h.bid * (i + 1) as u64)
        .sum()
}

fn main() {
    let part = match env::var("part") {
        Ok(val) => val,
        Err(_e) => "part1".to_string(),
    };

    if part == "part1" {
        println!("{}", part_1());
    } else if part == "part2" {
        println!("{}", part_2());
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parse_four_of_a_kind_ext() {
        let cards = vec![
            ExtCardRank::Base(CardRank::Q),
            ExtCardRank::Joker,
            ExtCardRank::Joker,
            ExtCardRank::Base(CardRank::Q),
            ExtCardRank::Base(CardRank::_2),
        ];
        let expected = HandType::FourOfAKind;
        assert_eq!(expected, HandType::from_card_ranks(&cards));
    }

    #[test]
    fn parse_highcard() {
        let cards = vec![
            ExtCardRank::Base(CardRank::A),
            ExtCardRank::Base(CardRank::T),
            ExtCardRank::Base(CardRank::J),
            ExtCardRank::Base(CardRank::Q),
            ExtCardRank::Base(CardRank::K),
        ];
        let expected = HandType::HighCard;
        assert_eq!(expected, HandType::from_card_ranks(&cards));
    }

    #[test]
    fn parse_five_of_a_kind() {
        let cards = vec![
            ExtCardRank::Base(CardRank::A),
            ExtCardRank::Base(CardRank::A),
            ExtCardRank::Base(CardRank::A),
            ExtCardRank::Base(CardRank::A),
            ExtCardRank::Base(CardRank::A),
        ];
        let expected = HandType::FiveOfAKind;
        assert_eq!(expected, HandType::from_card_ranks(&cards));
    }

    #[test]
    fn parse_one_pair() {
        let cards = vec![
            ExtCardRank::Base(CardRank::A),
            ExtCardRank::Base(CardRank::_2),
            ExtCardRank::Base(CardRank::_3),
            ExtCardRank::Base(CardRank::A),
            ExtCardRank::Base(CardRank::_4),
        ];
        let expected = HandType::OnePair;
        assert_eq!(expected, HandType::from_card_ranks(&cards));
    }

    #[test]
    fn parse_two_pair() {
        let cards = vec![
            ExtCardRank::Base(CardRank::_2),
            ExtCardRank::Base(CardRank::_3),
            ExtCardRank::Base(CardRank::_4),
            ExtCardRank::Base(CardRank::_3),
            ExtCardRank::Base(CardRank::_2),
        ];
        let expected = HandType::TwoPair;
        assert_eq!(expected, HandType::from_card_ranks(&cards));
    }

    #[test]
    fn parse_three_of_a_kind() {
        let cards = vec![
            ExtCardRank::Base(CardRank::T),
            ExtCardRank::Base(CardRank::T),
            ExtCardRank::Base(CardRank::T),
            ExtCardRank::Base(CardRank::_9),
            ExtCardRank::Base(CardRank::_8),
        ];
        let expected = HandType::ThreeOfAKind;
        assert_eq!(expected, HandType::from_card_ranks(&cards));
    }

    #[test]
    fn parse_four_of_a_kind() {
        let cards = vec![
            ExtCardRank::Base(CardRank::T),
            ExtCardRank::Base(CardRank::T),
            ExtCardRank::Base(CardRank::T),
            ExtCardRank::Base(CardRank::T),
            ExtCardRank::Base(CardRank::_8),
        ];
        let expected = HandType::FourOfAKind;
        assert_eq!(expected, HandType::from_card_ranks(&cards));
    }

    #[test]
    fn parse_full_house() {
        let cards = vec![
            ExtCardRank::Base(CardRank::_2),
            ExtCardRank::Base(CardRank::_3),
            ExtCardRank::Base(CardRank::_3),
            ExtCardRank::Base(CardRank::_3),
            ExtCardRank::Base(CardRank::_2),
        ];
        let expected = HandType::FullHouse;
        assert_eq!(expected, HandType::from_card_ranks(&cards));
    }
}
