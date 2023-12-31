use std::collections::HashMap;

#[derive(PartialEq, Eq, PartialOrd, Ord, Hash, Debug)]
pub(crate) enum Card {
    Jester,
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

impl From<&str> for Card {
    fn from(s: &str) -> Self {
        match s {
            "2" => Card::Two,
            "3" => Card::Three,
            "4" => Card::Four,
            "5" => Card::Five,
            "6" => Card::Six,
            "7" => Card::Seven,
            "8" => Card::Eight,
            "9" => Card::Nine,
            "T" => Card::Ten,
            "J" => Card::Jester,
            "Q" => Card::Queen,
            "K" => Card::King,
            "A" => Card::Ace,
            _ => panic!("Invalid card"),
        }
    }
}

impl From<char> for Card {
    fn from(c: char) -> Self {
        return Card::from(c.to_string().as_str());
    }
}

impl ToString for Card {
    fn to_string(&self) -> String {
        match self {
            Card::Two => String::from("2"),
            Card::Three => String::from("3"),
            Card::Four => String::from("4"),
            Card::Five => String::from("5"),
            Card::Six => String::from("6"),
            Card::Seven => String::from("7"),
            Card::Eight => String::from("8"),
            Card::Nine => String::from("9"),
            Card::Ten => String::from("T"),
            Card::Jester => String::from("J"),
            Card::Queen => String::from("Q"),
            Card::King => String::from("K"),
            Card::Ace => String::from("A"),
        }
    }
}

impl IntoIterator for Card {
    type Item = Card;
    type IntoIter = std::vec::IntoIter<Self::Item>;
    fn into_iter(self) -> Self::IntoIter {
        vec![self].into_iter()
    }
}

impl Card {
    pub(crate) fn freq_map(cards: &[Card]) -> HashMap<&Card, u8> {
        let mut map = HashMap::new();

        cards.iter().for_each(|c| {
            *map.entry(c).or_insert(0) += 1;
        });

        if let Some(&jester_count) = map.get(&Card::Jester) {
            println!("\tfound jesters: {}", jester_count);
            if let Some(max_and_not_jester) = map
                .iter_mut()
                .filter(|(c, _)| !c.is_jester())
                .max_by(|a, b| {
                    if a.1 == b.1 {
                        a.0.cmp(b.0)
                    } else {
                        a.1.cmp(&b.1)
                    }
                })
                .map(|(c, _)| *c)
            {
                *map.entry(max_and_not_jester).or_insert(0) += jester_count;
                *map.entry(&Card::Jester).or_insert(0) = 0;
            }
        }
        map
    }
    pub(crate) fn is_jester(&self) -> bool {
        *self == Card::Jester
    }
}
