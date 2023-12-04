use std::collections::HashMap;

use card::Card;

mod card;

fn solve1(input: &str) -> u32 {
    let cards = input.lines().map(|line| Card::from(line));

    cards.map(|c| c.get_points()).sum()
}

type SolvedCards = HashMap<u32, Vec<u32>>;

fn solve2(input: &str) -> u32 {
    let cards = input.lines().map(|line| Card::from(line));

    let mut unresolved_cards: Vec<u32> = cards.clone().map(|c| c.id).collect();

    let mut solved_cards: SolvedCards = HashMap::new();

    cards.for_each(|c| {
        let winning_cards = c.get_winning_cards();
        solved_cards.insert(c.id, winning_cards);
    });

    let mut total_cards = unresolved_cards.len();

    while !unresolved_cards.is_empty() {
        let card_id = unresolved_cards.pop().unwrap();

        let list_of_won_cards = solved_cards
            .get(&card_id)
            .expect("Every id should already exist in the map.");

        total_cards += list_of_won_cards.len();

        for id in list_of_won_cards {
            unresolved_cards.push(*id);
        }
    }

    total_cards.try_into().unwrap()
}

fn main() {
    let s = include_str!("../input.txt");
    println!("{}, {}", solve1(s), solve2(s));
}

#[cfg(test)]
mod tests {
    use crate::{solve1, solve2};

    #[test]
    fn test() {
        let s = include_str!("../test-input.txt");
        assert_eq!(solve1(s), 13);
    }

    #[test]
    fn test2() {
        let s = include_str!("../test-input.txt");
        assert_eq!(solve2(s), 30);
    }
}
