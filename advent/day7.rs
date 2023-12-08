use std::collections::HashMap;

use crate::utils::{Task, TaskError};

pub struct Day7;

#[derive(PartialEq, PartialOrd, Eq, Ord, Hash, Clone, Copy)]
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

#[derive(PartialEq, PartialOrd, Eq, Ord, Hash)]
enum HandRank {
    HighCard,
    OnePair,
    TwoPairs,
    ThreeOfAKind,
    FullHouse,
    FourOfAKind,
    FiveOfAKind,
}

#[derive(PartialEq, PartialOrd, Eq, Ord, Hash)]
struct Hand {
    rank: HandRank,
    cards: [Card; 5],
}

impl From<&str> for Hand {
    fn from(value: &str) -> Self {
        assert!(value.len() == 5);

        let mut hand = Hand {
            cards: [Card::A; 5],
            rank: HandRank::HighCard,
        };

        for (i, c) in value.chars().enumerate() {
            hand.cards[i] = Card::from(c);
        }
        hand
    }
}

impl Hand {
    fn init(&mut self, jokers_enabled: bool) {
        let mut card_groups: HashMap<Card, usize> = HashMap::new();

        if jokers_enabled {
            self.cards
                .iter_mut()
                .filter(|c| **c == Card::J)
                .for_each(|c| *c = Card::Joker);
            let mut jokers = 0;
            for card in &self.cards {
                if *card == Card::Joker {
                    jokers += 1;
                    continue;
                }
                *card_groups.entry(*card).or_insert(0) += 1;
            }
            if jokers == 5 {
                card_groups.insert(Card::Joker, 5);
            } else {
                *card_groups.values_mut().max().unwrap() += jokers;
            }
        } else {
            for card in &self.cards {
                *card_groups.entry(*card).or_insert(0) += 1;
            }
        }
        let rank = match card_groups.values().len() {
            5 => HandRank::HighCard,
            4 => HandRank::OnePair,
            3 => {
                let mut res = HandRank::HighCard;
                let mut pair_found = false;
                for group in card_groups.values() {
                    if *group == 3 {
                        res = HandRank::ThreeOfAKind;
                        break;
                    }
                    if *group == 2 {
                        if pair_found {
                            res = HandRank::TwoPairs;
                            break;
                        }
                        pair_found = true;
                        res = HandRank::OnePair;
                    }
                }
                res
            }
            2 => {
                let mut res = HandRank::FullHouse;
                for group in card_groups.values() {
                    if *group == 4 {
                        res = HandRank::FourOfAKind;
                        break;
                    }
                }
                res
            }
            1 => HandRank::FiveOfAKind,
            _ => panic!("Invalid lenght"),
        };
        self.rank = rank;
    }
}

fn calculate_winnings(file_content: &str, jokers_enabled: bool) -> usize {
    let mut hands = Vec::new();

    for line in file_content.lines() {
        let (hand, bid) = line.split_once(' ').unwrap();
        let mut hand = Hand::from(hand);
        hand.init(jokers_enabled);
        hands.push((hand, bid.parse::<usize>().unwrap()));
    }

    hands.sort();

    let mut total = 0;
    for (rank, (_, bid)) in hands.iter().enumerate() {
        total += (rank + 1) * bid;
    }
    total
}

impl Task for Day7 {
    fn task_part_one(&self, file_content: &str) -> Result<String, TaskError> {
        Ok(calculate_winnings(file_content, false).to_string())
    }

    fn task_part_two(&self, file_content: &str) -> Result<String, TaskError> {
        Ok(calculate_winnings(file_content, true).to_string())
    }

    fn get_day(&self) -> u32 {
        7
    }
}
