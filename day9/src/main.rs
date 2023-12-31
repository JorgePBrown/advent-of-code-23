use sequence::Sequence;

mod sequence;

fn solve1(input: &str) -> i64 {
    input
        .lines()
        .map(|l| Sequence::from(l).get_next_value())
        .sum()
}

fn solve2(input: &str) -> i64 {
    input
        .lines()
        .map(|l| Sequence::from(l).get_previous_value())
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
        let s = r"
0 3 6 9 12 15
1 3 6 10 15 21
10 13 16 21 30 45
"
        .trim();
        assert_eq!(solve1(s), 114);
    }

    #[test]
    fn part2() {
        let s = r"
0 3 6 9 12 15
1 3 6 10 15 21
10 13 16 21 30 45
"
        .trim();
        assert_eq!(solve2(s), 2);
    }
}
