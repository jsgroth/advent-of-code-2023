//! Day 7: Camel Cards
//!
//! <https://adventofcode.com/2023/day/7>

use nom::character::complete::{digit1, newline, one_of, space1};
use nom::combinator::map_res;
use nom::multi::{many_m_n, separated_list1};
use nom::sequence::separated_pair;
use nom::IResult;
use std::cmp;
use std::error::Error;

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
        let mut copy = self.0;
        copy.sort();

        if copy[0] == copy[4] {
            return HandType::FiveOfAKind;
        }

        if copy[0] == copy[3] || copy[1] == copy[4] {
            return HandType::FourOfAKind;
        }

        if (copy[0] == copy[2] && copy[3] == copy[4]) || (copy[0] == copy[1] && copy[2] == copy[4])
        {
            return HandType::FullHouse;
        }

        if copy[0] == copy[2] || copy[1] == copy[3] || copy[2] == copy[4] {
            return HandType::ThreeOfAKind;
        }

        if (copy[0] == copy[1] && copy[2] == copy[3])
            || (copy[0] == copy[1] && copy[3] == copy[4])
            || (copy[1] == copy[2] && copy[3] == copy[4])
        {
            return HandType::TwoPair;
        }

        if copy[0] == copy[1] || copy[1] == copy[2] || copy[2] == copy[3] || copy[3] == copy[4] {
            return HandType::OnePair;
        }

        HandType::HighCard
    }

    fn best_possible_hand_type(self) -> HandType {
        let mut best = self.hand_type();

        for i in 0..5 {
            if self.0[i] != JACK_VALUE {
                continue;
            }

            for value in 0..15 {
                if value == JACK_VALUE {
                    continue;
                }

                let mut copy = self.0;
                copy[i] = value;
                best = cmp::max(best, Self(copy).best_possible_hand_type());
            }
        }

        best
    }
}

impl From<&[u8]> for Hand {
    fn from(value: &[u8]) -> Self {
        let mut cards = [0; 5];
        cards.copy_from_slice(value);
        Self(cards)
    }
}

fn parse_card(input: &str) -> IResult<&str, u8> {
    let (input, c) = one_of("23456789TJQKA")(input)?;
    let value = match c {
        '2'..='9' => c.to_digit(10).unwrap() as u8,
        'T' => 10,
        'J' => 11,
        'Q' => 12,
        'K' => 13,
        'A' => 14,
        _ => panic!("Invalid card char: {c}"),
    };

    Ok((input, value))
}

fn parse_hand(input: &str) -> IResult<&str, Hand> {
    let (input, cards) = many_m_n(5, 5, parse_card)(input)?;

    Ok((input, Hand::from(cards.as_ref())))
}

fn parse_u64(input: &str) -> IResult<&str, u64> {
    map_res(digit1, str::parse)(input)
}

fn parse_line(input: &str) -> IResult<&str, (Hand, u64)> {
    separated_pair(parse_hand, space1, parse_u64)(input)
}

fn parse_input(input: &str) -> IResult<&str, Vec<(Hand, u64)>> {
    separated_list1(newline, parse_line)(input)
}

fn solve_part_1(input: &str) -> u64 {
    let (_, mut hands) = parse_input(input).expect("Invalid input");

    hands.sort_by(|(a, _), (b, _)| a.hand_type().cmp(&b.hand_type()).then_with(|| a.0.cmp(&b.0)));

    hands.into_iter().enumerate().map(|(i, (_, bid))| (i as u64 + 1) * bid).sum()
}

fn solve_part_2(input: &str) -> u64 {
    let (_, hands) = parse_input(input).expect("Invalid input");

    let mut hands: Vec<_> = hands
        .into_iter()
        .map(|(mut hand, bid)| {
            let hand_type = hand.best_possible_hand_type();
            for value in &mut hand.0 {
                if *value == JACK_VALUE {
                    *value = 1;
                }
            }
            (hand, bid, hand_type)
        })
        .collect();
    hands.sort_by(|(a, _, a_type), (b, _, b_type)| a_type.cmp(b_type).then_with(|| a.0.cmp(&b.0)));

    hands.into_iter().enumerate().map(|(i, (_, bid, _))| (i as u64 + 1) * bid).sum()
}

fn main() -> Result<(), Box<dyn Error>> {
    let input = advent_of_code_2023::read_input()?;

    let solution1 = solve_part_1(&input);
    println!("{solution1}");

    let solution2 = solve_part_2(&input);
    println!("{solution2}");

    Ok(())
}

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
