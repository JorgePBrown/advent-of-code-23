use std::cmp::Ordering;

#[derive(Debug, PartialEq, Eq)]
pub struct Hand {
    cards: Vec<char>,
    pub bid: u64,
}
impl Hand {
    pub fn multiple_from(input: &str) -> Vec<Self> {
        input.lines().map(|line| Hand::from(line)).collect()
    }
}

impl From<&str> for Hand {
    fn from(value: &str) -> Self {
        let spl: Vec<&str> = value.split_whitespace().collect();

        let cards = spl[0].chars().collect();

        let bid = spl[1].parse::<u64>().unwrap();

        Hand { cards, bid }
    }
}

impl PartialOrd for Hand {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        let c = compare_cards(&self.cards, &other.cards);

        if c == Ordering::Equal {
            self.bid.partial_cmp(&other.bid)
        } else {
            Some(c)
        }
    }
}

impl Ord for Hand {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        let c = compare_cards(&self.cards, &other.cards);

        if c == Ordering::Equal {
            self.bid.cmp(&other.bid)
        } else {
            c
        }
    }
}

impl Hand {
    pub fn cmp2(&self, other: &Self) -> std::cmp::Ordering {
        let c = compare_cards2(&self.cards, &other.cards);

        if c == Ordering::Equal {
            self.bid.cmp(&other.bid)
        } else {
            c
        }
    }
}

#[derive(Debug, Eq, PartialEq)]
enum HandType {
    FiveOfAKind,
    FourOfAKind,
    ThreeOfAKind,
    FullHouse,
    TwoPair,
    OnePair,
    HighCard,
}

impl HandType {
    fn to_rank(&self) -> u8 {
        match self {
            HandType::FiveOfAKind => 1,
            HandType::FourOfAKind => 2,
            HandType::FullHouse => 3,
            HandType::ThreeOfAKind => 4,
            HandType::TwoPair => 5,
            HandType::OnePair => 6,
            HandType::HighCard => 7,
        }
    }
}

impl From<&Vec<char>> for HandType {
    fn from(value: &Vec<char>) -> Self {
        let mut count: Vec<(char, u8)> = Vec::with_capacity(5);

        for v in value {
            let mut c: Option<&mut (char, u8)> = None;
            for cnt in count.iter_mut() {
                if cnt.0 == *v {
                    c = Some(cnt);
                    break;
                }
            }

            match c {
                None => count.push((*v, 1)),
                Some(tuple) => tuple.1 += 1,
            }
        }

        match count.len() {
            5 => HandType::HighCard,
            4 => HandType::OnePair,
            3 => {
                if count.iter().any(|c| c.1 == 3) {
                    HandType::ThreeOfAKind
                } else {
                    HandType::TwoPair
                }
            }
            2 => {
                if count.iter().any(|c| c.1 == 4) {
                    HandType::FourOfAKind
                } else {
                    HandType::FullHouse
                }
            }
            1 => HandType::FiveOfAKind,
            _ => panic!("WTF"),
        }
    }
}

impl HandType {
    fn from2(value: &Vec<char>) -> Self {
        let mut jokers = 0u8;
        let mut count: Vec<(char, u8)> = Vec::with_capacity(5);

        for v in value {
            if *v == 'J' {
                jokers += 1;
                continue;
            }
            let mut c: Option<&mut (char, u8)> = None;
            for cnt in count.iter_mut() {
                if cnt.0 == *v {
                    c = Some(cnt);
                    break;
                }
            }

            match c {
                None => count.push((*v, 1)),
                Some(tuple) => tuple.1 += 1,
            }
        }

        match count.len() {
            5 => HandType::HighCard,
            4 => HandType::OnePair,
            3 => {
                if jokers > 0 || count.iter().any(|c| c.1 == 3) {
                    HandType::ThreeOfAKind
                } else {
                    HandType::TwoPair
                }
            }
            2 => {
                if jokers >= 2
                    || jokers == 1 && count.iter().any(|c| c.1 == 3)
                    || count.iter().any(|c| c.1 == 4)
                {
                    HandType::FourOfAKind
                } else {
                    HandType::FullHouse
                }
            }
            1 | 0 => HandType::FiveOfAKind,
            _ => panic!("WTF"),
        }
    }
}

impl Ord for HandType {
    fn cmp(&self, other: &Self) -> Ordering {
        other.to_rank().cmp(&self.to_rank())
    }
}

impl PartialOrd for HandType {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

fn compare_cards(c1: &Vec<char>, c2: &Vec<char>) -> std::cmp::Ordering {
    let h_type_1 = HandType::from(c1);
    let h_type_2 = HandType::from(c2);

    let cmp = h_type_1.cmp(&h_type_2);
    if cmp != Ordering::Equal {
        return cmp;
    }

    for (card1, card2) in c1.iter().zip(c2) {
        if card1 != card2 {
            let rank1 = get_card_rank(card1);
            let rank2 = get_card_rank(card2);
            return rank2.cmp(&rank1);
        }
    }

    Ordering::Equal
}

fn get_card_rank(card: &char) -> u8 {
    match card {
        'A' => 1,
        'K' => 2,
        'Q' => 3,
        'J' => 4,
        'T' => 5,
        '9' => 6,
        '8' => 7,
        '7' => 8,
        '6' => 9,
        '5' => 10,
        '4' => 11,
        '3' => 12,
        '2' => 13,
        _ => panic!("{card} is not a card"),
    }
}

fn compare_cards2(c1: &Vec<char>, c2: &Vec<char>) -> std::cmp::Ordering {
    let h_type_1 = HandType::from2(c1);
    let h_type_2 = HandType::from2(c2);

    let cmp = h_type_1.cmp(&h_type_2);
    if cmp != Ordering::Equal {
        return cmp;
    }

    for (card1, card2) in c1.iter().zip(c2) {
        if card1 != card2 {
            let rank1 = get_card_rank2(card1);
            let rank2 = get_card_rank2(card2);
            return rank2.cmp(&rank1);
        }
    }

    Ordering::Equal
}

fn get_card_rank2(card: &char) -> u8 {
    match card {
        'A' => 1,
        'K' => 2,
        'Q' => 3,
        'T' => 5,
        '9' => 6,
        '8' => 7,
        '7' => 8,
        '6' => 9,
        '5' => 10,
        '4' => 11,
        '3' => 12,
        '2' => 13,
        'J' => 14,
        _ => panic!("{card} is not a card"),
    }
}

#[cfg(test)]
mod test_jokers {
    use crate::{hand::HandType, util::permutations};

    #[test]
    fn five() {
        assert_eq!(
            HandType::from2(&vec!['J', 'J', 'J', 'J', 'J']),
            HandType::FiveOfAKind
        );
        assert_eq!(
            HandType::from2(&vec!['K', 'K', 'K', 'K', 'K']),
            HandType::FiveOfAKind
        );
    }

    #[test]
    fn four() {
        let expected_h_type = HandType::FiveOfAKind;
        for p in permutations(vec!['K', 'J', 'J', 'J', 'J']) {
            assert_eq!(
                HandType::from2(&p),
                expected_h_type,
                "{:?} was not a {:?}",
                p,
                expected_h_type
            );
        }
    }

    #[test]
    fn three() {
        let expected_h_type = HandType::FourOfAKind;
        for p in permutations(vec!['K', 'A', 'J', 'J', 'J']) {
            assert_eq!(
                HandType::from2(&p),
                expected_h_type,
                "{:?} was not a {:?}",
                p,
                expected_h_type
            );
        }
        let expected_h_type = HandType::FiveOfAKind;
        for p in permutations(vec!['K', 'K', 'J', 'J', 'J']) {
            assert_eq!(
                HandType::from2(&p),
                expected_h_type,
                "{:?} was not a {:?}",
                p,
                expected_h_type
            );
        }
    }

    #[test]
    fn two() {
        let expected_h_type = HandType::ThreeOfAKind;
        for p in permutations(vec!['K', 'A', 'Q', 'J', 'J']) {
            assert_eq!(
                HandType::from2(&p),
                expected_h_type,
                "{:?} was not a {:?}",
                p,
                expected_h_type
            );
        }
        let expected_h_type = HandType::FourOfAKind;
        for p in permutations(vec!['K', 'A', 'A', 'J', 'J']) {
            assert_eq!(
                HandType::from2(&p),
                expected_h_type,
                "{:?} was not a {:?}",
                p,
                expected_h_type
            );
        }
        let expected_h_type = HandType::FiveOfAKind;
        for p in permutations(vec!['A', 'A', 'A', 'J', 'J']) {
            assert_eq!(
                HandType::from2(&p),
                expected_h_type,
                "{:?} was not a {:?}",
                p,
                expected_h_type
            );
        }
    }

    #[test]
    fn one() {
        let expected_h_type = HandType::OnePair;
        for p in permutations(vec!['K', 'A', 'Q', 'T', 'J']) {
            assert_eq!(
                HandType::from2(&p),
                expected_h_type,
                "{:?} was not a {:?}",
                p,
                expected_h_type
            );
        }
        let expected_h_type = HandType::ThreeOfAKind;
        for p in permutations(vec!['K', 'A', 'Q', 'Q', 'J']) {
            assert_eq!(
                HandType::from2(&p),
                expected_h_type,
                "{:?} was not a {:?}",
                p,
                expected_h_type
            );
        }
        let expected_h_type = HandType::FullHouse;
        for p in permutations(vec!['K', 'K', 'Q', 'Q', 'J']) {
            assert_eq!(
                HandType::from2(&p),
                expected_h_type,
                "{:?} was not a {:?}",
                p,
                expected_h_type
            );
        }
        let expected_h_type = HandType::FourOfAKind;
        for p in permutations(vec!['K', 'Q', 'Q', 'Q', 'J']) {
            assert_eq!(
                HandType::from2(&p),
                expected_h_type,
                "{:?} was not a {:?}",
                p,
                expected_h_type
            );
        }
        let expected_h_type = HandType::FiveOfAKind;
        for p in permutations(vec!['Q', 'Q', 'Q', 'Q', 'J']) {
            assert_eq!(
                HandType::from2(&p),
                expected_h_type,
                "{:?} was not a {:?}",
                p,
                expected_h_type
            );
        }
    }
}
