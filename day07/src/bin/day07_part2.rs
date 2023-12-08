use std::iter::zip;

use anyhow::Result;
use nom::{
    branch::alt,
    character::complete::{char, multispace1, newline},
    combinator::{all_consuming, map},
    multi::{many1, separated_list1},
    sequence::tuple,
    IResult,
};
use utils::{get_input_file_as_string, get_u64};

#[derive(Copy, Clone, Debug, PartialEq, Eq, PartialOrd, Ord)]
enum Card {
    Jack,
    Two,
    Three,
    Four,
    Five,
    Six,
    Seven,
    Eight,
    Nine,
    Ten,
    Queen,
    King,
    Ace,
}

#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord)]
enum HandType {
    HighCard,
    OnePair,
    TwoPair,
    ThreeOfAKind,
    FullHouse,
    FourOfAKind,
    FiveOfAKind,
}

type HandT = [Card; 5];

#[derive(Clone, Debug, PartialEq, Eq)]
struct Hand {
    cards: HandT,
    h_type: HandType,
    value: u64,
}

impl Ord for Hand {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        if self.h_type == other.h_type {
            self.cards.cmp(&other.cards)
        } else {
            self.h_type.cmp(&other.h_type)
        }
    }
}

impl PartialOrd for Hand {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        if self.h_type == other.h_type {
            self.cards.partial_cmp(&other.cards)
        } else {
            self.h_type.partial_cmp(&other.h_type)
        }
    }
}

impl Hand {
    fn new(cards: HandT, value: u64) -> Self {
        let mut sorted_cards: Vec<Card> = cards.into_iter().filter(|c| c != &Card::Jack).collect();
        sorted_cards.sort();
        sorted_cards.reverse();
        let js = 5 - sorted_cards.len();

        let h_type = if contains_n_of_a_kind(&sorted_cards, 5).is_some() || js == 5 {
            HandType::FiveOfAKind
        } else if contains_n_of_a_kind(&sorted_cards, 4).is_some() {
            if js > 0 {
                HandType::FiveOfAKind
            } else {
                HandType::FourOfAKind
            }
        } else {
            if let Some(thrice) = contains_n_of_a_kind(&sorted_cards, 3) {
                match js {
                    2 => HandType::FiveOfAKind,
                    1 => HandType::FourOfAKind,
                    _ => {
                        if contains_n_of_a_kind(&throw_out(&sorted_cards, thrice), 2).is_some() {
                            HandType::FullHouse
                        } else {
                            HandType::ThreeOfAKind
                        }
                    }
                }
            } else {
                if let Some(double) = contains_n_of_a_kind(&sorted_cards, 2) {
                    let three_cards = throw_out(&sorted_cards, double);
                    match js {
                        3 => HandType::FiveOfAKind,
                        2 => HandType::FourOfAKind,
                        1 => {
                            if contains_n_of_a_kind(&three_cards, 2).is_some() {
                                HandType::FullHouse
                            } else {
                                HandType::ThreeOfAKind
                            }
                        }
                        _ => {
                            if contains_n_of_a_kind(&three_cards, 3).is_some() {
                                HandType::FullHouse
                            } else if contains_n_of_a_kind(&three_cards, 2).is_some() {
                                HandType::TwoPair
                            } else {
                                HandType::OnePair
                            }
                        }
                    }
                } else {
                    match js {
                        4 => HandType::FiveOfAKind,
                        3 => HandType::FourOfAKind,
                        2 => HandType::ThreeOfAKind,
                        1 => HandType::OnePair,
                        _ => HandType::HighCard,
                    }
                }
            }
        };

        Hand {
            cards,
            h_type,
            value,
        }
    }
}

fn throw_out(sorted_cards: &Vec<Card>, throw: Card) -> Vec<Card> {
    sorted_cards
        .iter()
        .filter(|&c| c != &throw)
        .copied()
        .collect()
}

fn contains_n_of_a_kind(sorted_cards: &Vec<Card>, n: usize) -> Option<Card> {
    let mut iter = sorted_cards.windows(n);
    while let Some(exc) = iter.next() {
        match (exc.first(), exc.last(), exc.len()) {
            (Some(first), Some(last), n_found) if n == n_found => {
                if first == last {
                    return Some(*first);
                }
            }
            _ => continue,
        }
    }
    return None;
}

fn main() -> Result<()> {
    let data = get_input_file_as_string()?;

    let get_card_bid = map(
        tuple((many1(parse_card), multispace1, get_u64)),
        |(hand, _, bid)| {
            (
                hand.into_iter()
                    .map(|c| c)
                    .collect::<Vec<Card>>()
                    .try_into()
                    .unwrap(),
                bid,
            )
        },
    );

    let mut get_game = map(
        all_consuming(separated_list1(newline, get_card_bid)),
        |card| {
            // "unzip" Vec<(Vec<&Card>, u64)> into a Vec<&Card> and a Vec<u64>
            card.into_iter().fold(
                (vec![], vec![]),
                |(mut acc_hands, mut acc_values), (hand, value)| {
                    acc_hands.push(hand);
                    acc_values.push(value);
                    (acc_hands, acc_values)
                },
            )
        },
    );

    let (_, (hands, values)) = get_game(data.as_str()).map_err(|err| err.to_owned())?;

    let mut all_hands: Vec<Hand> = zip(hands, values).map(|(h, v)| Hand::new(h, v)).collect();
    all_hands.sort();
    let res: u64 = all_hands
        .iter()
        .enumerate()
        .map(|(i, h)| ((i + 1) as u64) * h.value)
        .sum();
    dbg!(res);
    Ok(())
}

fn parse_card(input: &str) -> IResult<&str, Card> {
    alt((
        map(char('2'), |_| Card::Two),
        map(char('3'), |_| Card::Three),
        map(char('4'), |_| Card::Four),
        map(char('5'), |_| Card::Five),
        map(char('6'), |_| Card::Six),
        map(char('7'), |_| Card::Seven),
        map(char('8'), |_| Card::Eight),
        map(char('9'), |_| Card::Nine),
        map(char('T'), |_| Card::Ten),
        map(char('J'), |_| Card::Jack),
        map(char('Q'), |_| Card::Queen),
        map(char('K'), |_| Card::King),
        map(char('A'), |_| Card::Ace),
    ))(input)
}
