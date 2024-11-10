use std::collections::HashMap;

use super::Card;

/// Flush predication
///
/// flush: five card with same suits
pub fn is_flush(cards: &Vec<Card>) -> bool {
    if cards.len() != 5 {
        return false;
    }

    for i in 1..5 {
        let prev = match cards.get(i - 1) {
            Some(p) => p,
            None => return false,
        };
        let cur = match cards.get(i) {
            Some(p) => p,
            None => return false,
        };
        if cur.suit != prev.suit {
            return false;
        }
    }

    true
}

#[test]
fn test_is_flush() {
    use crate::core::core::Hand;
    [
        ("As2s3s4s5s", true),
        ("Ah2h3h4h5h", true),
        ("Ah2h3d4h5h", false),
    ]
    .into_iter()
    .for_each(|(s, expect)| {
        assert_eq!(is_flush(&Hand::from_str(s).unwrap().cards), expect);
    });
}

/// Straight predication
///
/// straight: five card's number are continous
pub fn is_straight(cards: &Vec<Card>) -> bool {
    if cards.len() != 5 {
        return false;
    }

    let mut nums = cards.iter().map(|card| card.num).collect::<Vec<_>>();
    nums.sort();
    for i in (0..nums.len()).rev() {
        nums[i] -= nums[0];
    }

    nums.eq(&vec![0, 1, 2, 3, 4]) || nums.eq(&vec![0, 9, 10, 11, 12])
}

#[test]
fn test_is_straight() {
    use crate::core::core::Hand;
    [
        ("As2d3c4s5s", true),
        ("4s5d6c7s8s", true),
        ("TsJsQsKsAs", true),
        ("JsQdKcAs2s", false),
        ("JsQdKcAsTs", true),
        ("3s2d4c6s5s", true),
        ("3s2d4cAs5s", true),
    ]
    .into_iter()
    .for_each(|(s, expect)| {
        assert_eq!(is_straight(&Hand::from_str(s).unwrap().cards), expect);
    });
}

/// Straight Flush predication
pub fn is_straight_flush(cards: &Vec<Card>) -> bool {
    is_flush(cards) && is_straight(cards)
}

#[test]
fn test_is_straight_flush() {
    use crate::core::core::Hand;
    [
        ("As2s3s4s5s", true),
        ("6s2s3s4s5s", true),
        ("8s6s7s4s5s", true),
        ("JsKsQsAs2s", false),
        ("JsKsQsAsTs", true),
        ("JsKsQsAsTc", false),
        ("JcKcQcAcTc", true),
    ]
    .into_iter()
    .for_each(|(s, expect)| {
        assert_eq!(is_straight_flush(&Hand::from_str(s).unwrap().cards), expect);
    });
}

/// Royal Flush predication
pub fn is_royal_flush(cards: &Vec<Card>) -> bool {
    if !is_straight_flush(cards) {
        return false;
    }

    let mut nums = cards.iter().map(|card| card.num).collect::<Vec<_>>();
    nums.sort();

    nums[4] == 13
}

#[test]
fn test_is_royal_flush() {
    use crate::core::core::Hand;
    [
        ("TsJsQsKsAs", true),
        ("ThJhQhKhAh", true),
        ("TdJdQdKdAd", true),
        // non-sorted
        ("TcJcQcKcAc", true),
        ("TcJcQcAcKc", true),
        ("TcJcAcQcKc", true),
        ("TcKcAcQcJc", true),
        // wrong suits
        ("TcKcAcQcJs", false),
        ("TcKhAcQcJs", false),
        ("TcKhAcQcJs", false),
        // wrong straight
        ("As2s3s4s5s", false),
        ("6s2s3s4s5s", false),
        ("6s7s3s4s5s", false),
        ("6s7s8s4s5s", false),
    ]
    .into_iter()
    .for_each(|(s, expect)| {
        assert_eq!(is_royal_flush(&Hand::from_str(s).unwrap().cards), expect);
    });
}

/// Four of a Kind predication
pub fn is_foru_of_a_kind(cards: &Vec<Card>) -> bool {
    let map = get_num_count(cards);
    map.values().any(|count| *count >= 4)
}

fn get_num_count(cards: &Vec<Card>) -> HashMap<u8, u8> {
    let mut map = HashMap::new();
    for card in cards {
        map.insert(card.num, *map.get(&card.num).unwrap_or(&0) + 1);
    }
    map
}

#[test]
fn test_is_foru_of_a_kind() {
    use crate::core::core::Hand;
    [
        ("AsAsAsAs2s", true),
        ("5s5s5s5s2s", true),
        ("7s7s7s7s2s", true),
        ("KsKsKsKs2s", true),
        ("2sKsKsKsKs", true),
        ("2sKsKsKs2s", false),
        ("2sKsKs2s2s", false),
        ("QsKs2s2s2s", false),
    ]
    .into_iter()
    .for_each(|(s, expect)| {
        assert_eq!(is_foru_of_a_kind(&Hand::from_str(s).unwrap().cards), expect);
    });
}
