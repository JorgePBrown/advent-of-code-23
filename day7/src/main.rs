use hand::Hand;

mod hand;
mod util;

fn solve1(input: &str) -> u64 {
    let mut hands = Hand::multiple_from(input);

    hands.sort();

    hands
        .iter()
        .enumerate()
        .map(|(i, hand)| hand.bid * (i as u64 + 1))
        .sum()
}

fn solve2(input: &str) -> u64 {
    let mut hands = Hand::multiple_from(input);

    hands.sort_by(|h1, h2| h1.cmp2(h2));

    hands
        .iter()
        .enumerate()
        .map(|(i, hand)| hand.bid * (i as u64 + 1))
        .sum()
}

fn main() {
    let s = include_str!("../input.txt");
    println!("{}, {}", solve1(s), solve2(s));
}

#[cfg(test)]
mod tests {
    use crate::{solve1, solve2};

    #[test]
    fn part1() {
        let s = include_str!("../test-input.txt");
        assert_eq!(solve1(s), 6440);
    }

    #[test]
    fn part2() {
        let s = include_str!("../test-input.txt");
        assert_eq!(solve2(s), 5905);
    }
}
