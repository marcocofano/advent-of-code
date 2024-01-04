use std::cmp::Ordering;
use itertools::Itertools;

use advent_of_code::parsers::lines;

advent_of_code::solution!(7);

const DECK: &str = "23456789TJQKA";
const DECK_WITH_J: &str = "J23456789TQKA";
pub fn compute_type(cards: Vec<u32>, deck: &str) -> HandType {
    let mut frequencies = vec![0; deck.len()]; // [0; deck.len()] does not work because it doesn't
    // know the size at compile time.. What is vec! doing then :), magic???? More to learn

    for &num in cards.iter() {
        frequencies[num as usize] += 1;
    }
    frequencies.sort();
    frequencies.reverse();

    let mut result = match frequencies[0..5] {
        [5,0,0,0,0] => HandType::FiveOfAKind,
        [4,1,0,0,0] => HandType::FourOfAKind,
        [3,2,0,0,0] => HandType::FullHouse,
        [3,1,1,0,0] => HandType::ThreeOfAKind,
        [2,2,1,0,0] => HandType::TwoPair,
        [2,1,1,1,0] => HandType::OnePair,
        _ => HandType::HighCard,
    };
    if deck.starts_with('J') {
        result = match cards.iter().filter(|&c| c == &0).count() { // count the number of Jokers
            // and match on those, only a few cases to handle
            0 => result,
            1 => match result {
                HandType::HighCard => HandType::OnePair,
                HandType::OnePair => HandType::ThreeOfAKind,
                HandType::TwoPair => HandType::FullHouse,
                HandType::ThreeOfAKind => HandType::FourOfAKind,
                HandType::FourOfAKind => HandType::FiveOfAKind,
                _ => result 
            },
            2 => match result {
                HandType::OnePair => HandType::ThreeOfAKind,
                HandType::TwoPair => HandType::FourOfAKind,
                HandType::FullHouse => HandType::FiveOfAKind,
                _ => result 
            }
            3 => match result {
                HandType::ThreeOfAKind => HandType::FourOfAKind,
                HandType::FullHouse => HandType::FiveOfAKind,
                _ => result 
            },
            _ => match result {
                HandType::FourOfAKind => HandType::FiveOfAKind,
                _ => result 
            },
        }
    }
    return result
}

pub fn card_mapper(card: char, deck: &str) -> u32 {
    deck.find(|c| c == card).unwrap() as u32
}
#[derive(Debug, PartialOrd, Ord, PartialEq, Eq)]
pub enum HandType {
    HighCard,
    OnePair,
    TwoPair,
    ThreeOfAKind,
    FullHouse,
    FourOfAKind,
    FiveOfAKind,
}

#[derive(Debug)]
pub struct Hand {
    cards: Vec<u32>,
    bet: u32,
    hand_type: HandType,
}

impl Hand {
    fn cmp(&self, other: &Hand) -> Ordering {
        if std::mem::discriminant(&self.hand_type) == std::mem::discriminant(&other.hand_type) {
            self.cards.cmp(&other.cards)
        } else {
            self.hand_type.cmp(&other.hand_type)
        }
    }
}

fn parse_input(line: &str, deck: &str) -> Hand {
    let (card, bet) = line
        .split_once(" ")
        .unwrap();
    let hand = card
        .chars()
        .map(|c| card_mapper(c, deck))
        .collect_vec();
    return Hand {
        cards: hand.clone(),
        bet: bet.parse().unwrap(),
        hand_type: compute_type(hand, deck),
    }
}

pub fn part_one(input: &str) -> Option<u32> {
    Some(
        lines(input)
            .map(|ln| parse_input(ln, &DECK))
            .sorted_by(|a, b| a.cmp(b))
            .enumerate()
            .map(|(k, h)| (k as u32 + 1) * h.bet)
            .sum(),
    )
}

pub fn part_two(input: &str) -> Option<u32> {
    Some(
        lines(input)
            .map(|ln| parse_input(ln, &DECK_WITH_J))
            .sorted_by(|a, b| a.cmp(b))
            .enumerate()
            // .inspect(|(i, hand)| println!("{:?}, {:?}, {:?}, {:?}", i, hand.cards, hand.bet, hand.hand_type))
            .map(|(k, h)| (k as u32 + 1) * h.bet)
            .sum(),
    )
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(6440));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(5905));
    }
}
