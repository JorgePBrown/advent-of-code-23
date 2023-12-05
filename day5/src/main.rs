use almanac::Almanac;

mod almanac;

fn solve1(input: &str) -> u64 {
    let almanac = Almanac::from(input);

    almanac
        .needed_seeds
        .iter()
        .map(|seed| almanac.get_location_for_seed(*seed))
        .min()
        .unwrap()
}

fn main() {
    let s = include_str!("../input.txt");
    println!("{}", solve1(s));
}

#[cfg(test)]
mod tests {
    use crate::solve1;

    #[test]
    fn test1() {
        let s = include_str!("../test-input.txt");
        assert_eq!(solve1(s), 35);
    }
}
