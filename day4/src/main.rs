use card::Card;

mod card;

fn solve1(input: &str) -> u32 {
    let cards = input.lines().map(|line| Card::from(line));

    cards.map(|c| c.get_points()).sum()
}

fn main() {
    let s = include_str!("../input.txt");
    println!("{}", solve1(s));
}

#[cfg(test)]
mod tests {
    use crate::solve1;

    #[test]
    fn test() {
        let s = include_str!("../test-input.txt");
        assert_eq!(solve1(s), 13);
    }
}
