use std::fmt::Debug;

use super::card::Card;

pub(crate) struct Hand {
    pub(crate) cards: [Card; 5],
    pub(crate) bid: u32,
    _id: String,
}

#[derive(PartialEq, Eq, PartialOrd, Ord, Debug, Copy, Clone)]
pub(crate) enum HandType {
    HighCard,
    OnePair,
    TwoPair,
    ThreeOfAKind,
    FullHouse,
    FourOfAKind,
    FiveOfAKind,
}

impl From<&str> for Hand {
    fn from(s: &str) -> Self {
        let parts = s.split_whitespace().collect::<Vec<&str>>();
        assert!(parts.len() == 2);

        let cards = parts[0]
            .chars()
            .map(Card::from)
            .collect::<Vec<Card>>()
            .try_into()
            .expect("Expected 5 cards");
        let bid = parts[1].parse().unwrap();

        Hand {
            cards,
            bid,
            _id: parts[0].to_string(),
        }
    }
}

impl Hand {
    pub(crate) fn get_type(&self) -> HandType {
        println!("{:?}", self);
        let freq_map = Card::freq_map(&self.cards);

        let max_value = *freq_map.values().max().unwrap_or(&0);
        match max_value {
            5 => HandType::FiveOfAKind,
            4 => HandType::FourOfAKind,
            3 => {
                if freq_map.values().any(|&v| v == 2) {
                    return HandType::FullHouse;
                }
                HandType::ThreeOfAKind
            }
            2 => {
                if freq_map.values().filter(|&&v| v == 2).count() == 2 {
                    return HandType::TwoPair;
                }
                HandType::OnePair
            }
            1 => HandType::HighCard,
            0 => panic!("Shouldn't happen"),
            _ => unreachable!(),
        }
    }
}

impl PartialEq for Hand {
    fn eq(&self, other: &Self) -> bool {
        self.cards == other.cards
    }
}

impl Eq for Hand {}

impl PartialOrd for Hand {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for Hand {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        if self.get_type() == other.get_type() {
            self.cards.cmp(&other.cards)
        } else {
            self.get_type().cmp(&other.get_type())
        }
    }
}

impl Debug for Hand {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self._id)
    }
}
