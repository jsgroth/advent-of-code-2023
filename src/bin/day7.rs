//! Day 7: Camel Cards
//!
//! <https://adventofcode.com/2023/day/7>

use advent_of_code_2023::impl_standard_main;
use winnow::ascii::{digit1, newline, space1};
use winnow::combinator::{fail, opt, repeat, separated, separated_pair, success};
use winnow::dispatch;
use winnow::prelude::*;
use winnow::token::any;

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
enum HandType {
    HighCard,
    OnePair,
    TwoPair,
    ThreeOfAKind,
    FullHouse,
    FourOfAKind,
    FiveOfAKind,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
struct Hand([u8; 5]);

const JACK_VALUE: u8 = 11;

impl Hand {
    fn hand_type(self) -> HandType {
        let group_lengths = compute_group_lengths(self.0, false);

        match (group_lengths[0], group_lengths[1]) {
            (5, _) => HandType::FiveOfAKind,
            (4, _) => HandType::FourOfAKind,
            (3, 2) => HandType::FullHouse,
            (3, 1) => HandType::ThreeOfAKind,
            (2, 2) => HandType::TwoPair,
            (2, 1) => HandType::OnePair,
            (1, _) => HandType::HighCard,
            _ => panic!("Unexpected group lengths: {group_lengths:?}"),
        }
    }

    fn best_possible_hand_type(self) -> HandType {
        let jack_count = self.0.into_iter().filter(|&card| card == JACK_VALUE).count();

        let group_lengths = compute_group_lengths(self.0, true);

        match (jack_count, group_lengths[0], group_lengths[1]) {
            (0, _, _) => self.hand_type(),
            (1, 4, _) | (2, 3, _) | (3, 2, _) | (4, _, _) | (5, _, _) => HandType::FiveOfAKind,
            (1, 3, _) | (2, 2, _) | (3, 1, _) => HandType::FourOfAKind,
            (1, 2, 2) => HandType::FullHouse,
            (1, 2, 1) | (2, 1, _) => HandType::ThreeOfAKind,
            (1, 1, _) => HandType::OnePair,
            _ => panic!(
                "Unexpected jack count / group lengths combination: {jack_count} / {group_lengths:?}"
            ),
        }
    }
}

fn compute_group_lengths(mut hand: [u8; 5], ignore_jacks: bool) -> [u8; 5] {
    hand.sort();

    let mut out = [0; 5];
    let mut out_idx = 0;

    let mut last_card = None;
    for card in hand {
        if ignore_jacks && card == JACK_VALUE {
            continue;
        }

        if Some(card) != last_card {
            if last_card.is_some() {
                out_idx += 1;
            }
            last_card = Some(card);
        }

        out[out_idx] += 1;
    }

    out.sort_by(|a, b| a.cmp(b).reverse());

    out
}

impl From<&[u8]> for Hand {
    fn from(value: &[u8]) -> Self {
        let mut cards = [0; 5];
        cards.copy_from_slice(value);
        Self(cards)
    }
}

fn parse_u64(input: &mut &str) -> PResult<u64> {
    digit1.parse_to().parse_next(input)
}

fn parse_card(input: &mut &str) -> PResult<u8> {
    dispatch! { any;
        c @ '2'..='9' => success(c.to_digit(10).unwrap() as u8),
        'T' => success(10),
        'J' => success(11),
        'Q' => success(12),
        'K' => success(13),
        'A' => success(14),
        _ => fail,
    }
    .parse_next(input)
}

fn parse_hand(input: &mut &str) -> PResult<Hand> {
    let cards: Vec<_> = repeat(5, parse_card).parse_next(input)?;
    Ok(Hand::from(cards.as_ref()))
}

fn parse_line(input: &mut &str) -> PResult<(Hand, u64)> {
    separated_pair(parse_hand, space1, parse_u64).parse_next(input)
}

fn parse_input(input: &mut &str) -> PResult<Vec<(Hand, u64)>> {
    let hands = separated(1.., parse_line, newline).parse_next(input)?;

    opt(newline).parse_next(input)?;

    Ok(hands)
}

fn solve_part_1(input: &str) -> u64 {
    let mut hands = parse_input.parse(input).expect("Invalid input");

    hands.sort_by(|(a, _), (b, _)| a.hand_type().cmp(&b.hand_type()).then_with(|| a.0.cmp(&b.0)));

    hands.into_iter().enumerate().map(|(i, (_, bid))| (i as u64 + 1) * bid).sum()
}

fn solve_part_2(input: &str) -> u64 {
    let hands = parse_input.parse(input).expect("Invalid input");

    let mut hands: Vec<_> = hands
        .into_iter()
        .map(|(mut hand, bid)| {
            let hand_type = hand.best_possible_hand_type();
            for value in &mut hand.0 {
                if *value == JACK_VALUE {
                    // Make jacks sort below every other card when comparing arrays
                    *value = 1;
                }
            }
            (hand, bid, hand_type)
        })
        .collect();
    hands.sort_by(|(a, _, a_type), (b, _, b_type)| a_type.cmp(b_type).then_with(|| a.0.cmp(&b.0)));

    hands.into_iter().enumerate().map(|(i, (_, bid, _))| (i as u64 + 1) * bid).sum()
}

impl_standard_main!(p1: solve_part_1, p2: solve_part_2);

#[cfg(test)]
mod tests {
    use super::*;

    const SAMPLE_INPUT: &str = include_str!("../sample/day7.txt");

    #[test]
    fn sample_input_part_1() {
        assert_eq!(solve_part_1(SAMPLE_INPUT), 6440);
    }

    #[test]
    fn sample_input_part_2() {
        assert_eq!(solve_part_2(SAMPLE_INPUT), 5905);
    }
}
