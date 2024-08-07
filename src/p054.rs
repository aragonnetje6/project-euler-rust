use std::{cmp::Ordering, iter::zip};

use crate::file_io;

pub fn p054() -> i32 {
    let mut total = 0;
    for line in file_io::read_file("src/p054_poker.txt").split('\n') {
        let (mut hand1, mut hand2) = decode_hands(line);
        hand1.sort_by_key(|x| x.value);
        hand2.sort_by_key(|x| x.value);
        let (rank1, val1) = rank_hand(&hand1);
        let (rank2, val2) = rank_hand(&hand2);
        match rank1.cmp(&rank2) {
            Ordering::Greater => total += 1,
            Ordering::Equal => match val1.cmp(&val2) {
                Ordering::Greater => total += 1,
                Ordering::Equal => {
                    for (highest1, highest2) in zip(&hand1, &hand2) {
                        match highest1.get_value().cmp(&highest2.get_value()) {
                            Ordering::Equal => {}
                            Ordering::Less => break,
                            Ordering::Greater => {
                                total += 1;
                                break;
                            }
                        }
                    }
                }
                Ordering::Less => {}
            },
            Ordering::Less => {}
        }
    }
    total
}

fn rank_hand(hand: &[Card]) -> (i32, i32) {
    let hand_values: Vec<i32> = hand.iter().map(Card::get_value).collect();
    let max_value = *hand_values.iter().max().unwrap();
    // Royal Flush
    if same_suit(hand) && hand_values.iter().all(|x| [10, 11, 12, 13, 14].contains(x)) {
        return (9, max_value);
    }
    // Straight Flush
    if same_suit(hand) && is_consecutive(&hand_values) {
        return (8, max_value);
    }
    // Four of a Kind
    let (_, no_first) = hand.split_first().unwrap();
    let (_, no_last) = hand.split_last().unwrap();
    if same_value(no_first) || same_value(no_last) {
        return (7, *hand_values.get(2).unwrap());
    }
    // Full House
    let (first_two, last_three) = hand.split_at(2);
    let (first_three, last_two) = hand.split_at(3);
    if (same_value(first_two) && same_value(last_three))
        || (same_value(first_three) && same_value(last_two))
    {
        return (6, *hand_values.get(2).unwrap());
    }
    // Flush
    if same_suit(hand) {
        return (5, max_value);
    }
    // Straight
    if is_consecutive(&hand_values) {
        return (4, max_value);
    }
    // Three of a Kind
    let (_, middle_three) = no_first.split_last().unwrap();
    if same_value(first_three) || same_value(middle_three) || same_value(last_three) {
        return (3, *hand_values.get(2).unwrap());
    }
    // Two Pairs
    let (_, second_two) = first_three.split_first().unwrap();
    let (_, third_two) = last_three.split_last().unwrap();
    let pairs: Vec<&[Card]> = [first_two, second_two, third_two, last_two]
        .iter()
        .filter(|x| same_value(x))
        .copied()
        .collect();
    if pairs.len() == 2 {
        return (
            2,
            pairs
                .iter()
                .map(|x| x.first().unwrap().value)
                .max()
                .unwrap()
                * 20
                + pairs
                    .iter()
                    .map(|x| x.first().unwrap().value)
                    .min()
                    .unwrap(),
        );
    }
    // One Pair
    if pairs.len() == 1 {
        return (1, pairs.first().unwrap().first().unwrap().value);
    }
    (0, max_value)
}

fn is_consecutive(hand_values: &[i32]) -> bool {
    hand_values
        .iter()
        .enumerate()
        .all(|(i, x)| x - hand_values[0] == i as i32)
}

fn same_suit(cards: &[Card]) -> bool {
    cards.iter().all(|x| x.suit == cards.first().unwrap().suit)
}

fn same_value(cards: &[Card]) -> bool {
    cards
        .iter()
        .all(|x| x.value == cards.first().unwrap().value)
}

fn decode_hands(line: &str) -> (Vec<Card>, Vec<Card>) {
    let cards: Vec<Card> = line.split(' ').map(Card::from_code).collect();
    let (h1, h2) = cards.split_at(5);
    (Vec::from(h1), Vec::from(h2))
}

#[derive(Clone, Debug)]
struct Card {
    value: i32,
    suit: char,
}

impl Card {
    fn from_code(code: &str) -> Card {
        let value_char = code.chars().nth(0).unwrap();
        let suit_char = code.chars().nth(1).unwrap();
        Card {
            value: match value_char.to_digit(10) {
                Some(x) => x,
                None => match value_char {
                    'T' => 10,
                    'J' => 11,
                    'Q' => 12,
                    'K' => 13,
                    'A' => 14,
                    _ => panic!("Card value error"),
                },
            } as i32,
            suit: suit_char,
        }
    }

    fn get_suit(&self) -> char {
        self.suit
    }

    fn get_value(&self) -> i32 {
        self.value
    }
}
