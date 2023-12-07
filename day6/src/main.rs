use race::Race;

mod race;

fn solve1(input: &str) -> u64 {
    let races = Race::from_stats(input);

    races
        .iter()
        .map(|race| race.get_number_of_record_beating_solutions())
        .reduce(|acc, v| acc * v)
        .unwrap()
}

fn solve2(input: &str) -> u64 {
    let race = Race::from_stats_2(input);

    race.get_number_of_record_beating_solutions()
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
        assert_eq!(solve1(s), 288);
    }

    #[test]
    fn part2() {
        let s = include_str!("../test-input.txt");
        assert_eq!(solve2(s), 71503);
    }
}
