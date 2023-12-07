use std::collections::BTreeMap;

use crate::utils::{Task, TaskError};

pub struct Day7;

#[derive(PartialEq, PartialOrd, Eq, Ord, Clone, Copy, Debug)]
enum Card {
    Two,
    Three,
    Four,
    Five,
    Six,
    Seven,
    Eight,
    Nine,
    T,
    J,
    Q,
    K,
    A,
}

impl From<char> for Card {
    fn from(value: char) -> Self {
        match value {
            'A' => Card::A,
            'K' => Card::K,
            'Q' => Card::Q,
            'J' => Card::J,
            'T' => Card::T,
            '9' => Card::Nine,
            '8' => Card::Eight,
            '7' => Card::Seven,
            '6' => Card::Six,
            '5' => Card::Five,
            '4' => Card::Four,
            '3' => Card::Three,
            '2' => Card::Two,
            _ => panic!("Unknown card"),
        }
    }
}

#[derive(PartialEq, Eq, PartialOrd, Ord, Debug)]
struct Hand {
    rank: usize,
    cards: [Card; 5],
}

impl From<&str> for Hand {
    fn from(value: &str) -> Self {
        assert!(value.len() == 5);

        let mut hand = Hand {
            cards: [Card::A; 5],
            rank: 0,
        };

        for (i, c) in value.chars().enumerate() {
            hand.cards[i] = Card::from(c);
        }

        // let mut card_groups = Vec::new();
        let mut card_groups: BTreeMap<Card, usize> = BTreeMap::new();

        for card in &hand.cards {
            *card_groups.entry(*card).or_insert(0) += 1;
        }

        let rank = match card_groups.values().len() {
            5 => 1,
            4 => 2,
            3 => {
                let mut res = 0;
                let mut pair_found = false;
                for group in card_groups.values() {
                    if *group == 3 {
                        res = 4;
                        break;
                    }
                    if *group == 2 {
                        if pair_found {
                            res = 3;
                            break;
                        }
                        pair_found = true;
                        res = 2;
                    }
                }
                res
            }
            2 => {
                let mut res = 5;
                for group in card_groups.values() {
                    if *group == 4 {
                        res = 6;
                        break;
                    }
                }
                res
            }
            1 => 7,
            _ => panic!("Invalid lenght"),
        };
        hand.rank = rank;
        hand
    }
}

#[derive(PartialEq, PartialOrd, Eq, Ord, Clone, Copy, Debug)]
enum CardJ {
    J,
    Two,
    Three,
    Four,
    Five,
    Six,
    Seven,
    Eight,
    Nine,
    T,
    Q,
    K,
    A,
}

impl From<char> for CardJ {
    fn from(value: char) -> Self {
        match value {
            'A' => CardJ::A,
            'K' => CardJ::K,
            'Q' => CardJ::Q,
            'J' => CardJ::J,
            'T' => CardJ::T,
            '9' => CardJ::Nine,
            '8' => CardJ::Eight,
            '7' => CardJ::Seven,
            '6' => CardJ::Six,
            '5' => CardJ::Five,
            '4' => CardJ::Four,
            '3' => CardJ::Three,
            '2' => CardJ::Two,
            _ => panic!("Unknown card"),
        }
    }
}

#[derive(PartialEq, Eq, PartialOrd, Ord, Debug)]
struct HandJ {
    rank: usize,
    cards: [CardJ; 5],
}

impl From<&str> for HandJ {
    fn from(value: &str) -> Self {
        assert!(value.len() == 5);

        let mut hand = HandJ {
            cards: [CardJ::A; 5],
            rank: 0,
        };

        for (i, c) in value.chars().enumerate() {
            hand.cards[i] = CardJ::from(c);
        }

        let mut card_groups: BTreeMap<CardJ, usize> = BTreeMap::new();

        let mut jokers = 0;
        for card in &hand.cards {
            if *card == CardJ::J {
                jokers += 1;
                continue;
            }
            *card_groups.entry(*card).or_insert(0) += 1;
        }
        if jokers == 5 {
            card_groups.insert(CardJ::J, 5);
        } else if jokers != 0 {
            *card_groups.values_mut().max().unwrap() += jokers;
        }

        let rank = match card_groups.values().len() {
            5 => 1,
            4 => 2,
            3 => {
                let mut res = 0;
                let mut pair_found = false;
                for group in card_groups.values() {
                    if *group == 3 {
                        res = 4;
                        break;
                    }
                    if *group == 2 {
                        if pair_found {
                            res = 3;
                            break;
                        }
                        pair_found = true;
                        res = 2;
                    }
                }
                res
            }
            2 => {
                let mut res = 5;
                for group in card_groups.values() {
                    if *group == 4 {
                        res = 6;
                        break;
                    }
                }
                res
            }
            1 => 7,
            _ => panic!("Invalid lenght"),
        };
        hand.rank = rank;
        hand
    }
}

impl Task for Day7 {
    fn task_part_one(&self, file_content: &str) -> Result<String, TaskError> {
        let mut hands = Vec::new();

        for line in file_content.lines() {
            let (hand, bid) = line.split_once(' ').unwrap();
            hands.push((Hand::from(hand), bid.parse::<usize>().unwrap()));
        }

        hands.sort();

        let mut total = 0;
        for (rank, (_, bid)) in hands.iter().enumerate() {
            total += (rank + 1) * bid;
        }

        Ok(total.to_string())
    }

    fn task_part_two(&self, file_content: &str) -> Result<String, TaskError> {
        let mut hands = Vec::new();

        for line in file_content.lines() {
            let (hand, bid) = line.split_once(' ').unwrap();
            hands.push((HandJ::from(hand), bid.parse::<usize>().unwrap()));
        }

        hands.sort();
        let mut total = 0;
        for (rank, (_, bid)) in hands.iter().enumerate() {
            total += (rank + 1) * bid;
        }

        Ok(total.to_string())
    }
}
