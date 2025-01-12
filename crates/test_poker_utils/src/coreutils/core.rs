use std::hash::Hash;

use rand::{rngs::ThreadRng, Rng};

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub enum Suits {
    Spades,
    Hearts,
    Diamonds,
    Clubs,
}

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub struct Card {
    pub suit: Suits,
    pub num: u8, // 1-13, 0 for unknown
}

impl Card {
    pub fn from(suit: Suits, num: u8) -> Card {
        Card { suit, num }
    }

    /// Create Card with &str
    ///
    /// As => Spade 1 \
    /// Th => Heart 1 \
    /// Qd => Diamond 12 \
    /// 7c => Club 7
    #[allow(clippy::should_implement_trait)]
    pub fn from_str(s: &str) -> Option<Card> {
        let bytes = s.as_bytes();
        let num_char = bytes[0] as char;
        let suit_char = bytes[1] as char;

        let suit = match suit_char {
            's' => Suits::Spades,
            'h' => Suits::Hearts,
            'd' => Suits::Diamonds,
            'c' => Suits::Clubs,
            _ => return None,
        };
        let num: u8 = match num_char {
            'A' => 1,
            'T' => 10,
            'J' => 11,
            'Q' => 12,
            'K' => 13,
            '2'..='9' => num_char as u8 - b'0',
            _ => return None,
        };

        Some(Card::from(suit, num))
    }

    pub fn from_strs(list: Vec<&str>) -> Option<Vec<Card>> {
        list.into_iter().map(Card::from_str).collect()
    }

    /// Reverse action of from_str
    #[allow(clippy::inherent_to_string)]
    pub fn to_string(&self) -> String {
        let suit_char = match self.suit {
            Suits::Spades => 's',
            Suits::Hearts => 'h',
            Suits::Diamonds => 'd',
            Suits::Clubs => 'c',
        };
        let num_char = match self.num {
            1 => 'A',
            10 => 'T',
            11 => 'J',
            12 => 'Q',
            13 => 'K',
            n => (n + b'0') as char,
        };

        format!("{}{}", num_char, suit_char)
    }
}

impl Hash for Card {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        state.write(&[self.num, self.suit as u8]);
    }
}

#[cfg(test)]
mod card_tests {
    use super::{Card, Suits};

    #[test]
    fn test_from_str() {
        // number
        [
            "As", "2s", "3s", "4s", "5s", "6s", "7s", "8s", "9s", "Ts", "Js", "Qs", "Ks",
        ]
        .into_iter()
        .enumerate()
        .for_each(|(i, s)| {
            let suit = Suits::Spades;
            let num = i as u8 + 1;
            assert_eq!(Card::from_str(s).unwrap(), Card::from(suit, num));
        });

        // suits
        [
            ("As", Suits::Spades),
            ("Ah", Suits::Hearts),
            ("Ad", Suits::Diamonds),
            ("Ac", Suits::Clubs),
        ]
        .into_iter()
        .for_each(|(s, suit)| {
            assert_eq!(Card::from_str(s).unwrap(), Card::from(suit, 1));
        });
    }

    #[test]
    fn test_from_strs() {
        let cards = Card::from_strs(vec!["As", "2s", "3s", "4s", "5s"]).unwrap();

        assert_eq!(
            cards,
            vec![
                Card::from(Suits::Spades, 1),
                Card::from(Suits::Spades, 2),
                Card::from(Suits::Spades, 3),
                Card::from(Suits::Spades, 4),
                Card::from(Suits::Spades, 5),
            ]
        )
    }
}

#[derive(Debug)]
pub struct CardDeck {
    rng: ThreadRng,
    rest: Vec<u8>,
}

impl Default for CardDeck {
    fn default() -> Self {
        Self::new()
    }
}

impl CardDeck {
    pub fn new() -> CardDeck {
        CardDeck {
            rng: rand::thread_rng(),
            rest: (0..52).collect::<Vec<u8>>(),
        }
    }

    pub fn shuffle(&mut self) {
        self.rng = rand::thread_rng();
    }

    pub fn deal(&mut self) -> Option<Card> {
        let num = self.rest.remove(self.rng.gen_range(0..self.rest.len()));
        let suit_seq = num / 13;
        let num = num % 13 + 1;
        let suit = match suit_seq {
            0 => Suits::Spades,
            1 => Suits::Hearts,
            2 => Suits::Diamonds,
            3 => Suits::Clubs,
            _ => return None,
        };

        Some(Card::from(suit, num))
    }

    pub fn len(&self) -> usize {
        self.rest.len()
    }

    pub fn is_empty(&self) -> bool {
        self.rest.is_empty()
    }
}

#[cfg(test)]
mod card_deck_test {
    use std::collections::HashSet;

    use crate::coreutils::CardDeck;

    #[test]
    fn test_deal() {
        let mut deal_count = 0;
        let mut deal_cards = HashSet::new();

        let mut deck = CardDeck::new();
        while !deck.is_empty() {
            deal_count += 1;
            let card = deck.deal().unwrap();
            assert!(deal_cards.insert(card));
        }
        assert_eq!(deal_count, 52);
        assert_eq!(deal_cards.len(), 52);
    }
}

pub enum HandRank {
    RoyalFlush,
    StraightFlush,
    FourOfAKind,
    FullHouse,
    Flush,
    Straight,
    ThreeOfAKind,
    TwoPair,
    OnePair,
    HighCard,
}

#[derive(Debug)]
pub struct Hand {
    pub cards: Vec<Card>,
}

impl Hand {
    #[allow(clippy::should_implement_trait)]
    pub fn from_str(s: &str) -> Option<Hand> {
        if s.len() != 10 {
            return None;
        }

        let cards = s
            .chars()
            .collect::<Vec<_>>()
            .chunks(2)
            .map(|inner_s| Card::from_str(inner_s.iter().collect::<String>().as_str()))
            .collect::<Option<Vec<Card>>>()?;

        Some(Hand { cards })
    }
}

#[test]
fn test_hand_from_str() {
    assert!(Hand::from_str("As2s3s4s5s").is_some());
}
