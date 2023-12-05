use core::panic;

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

fn solve2(input: &str) -> u64 {
    let almanac = Almanac::from(input);

    if almanac.needed_seeds.len() & 1 != 0 {
        panic!("Odd number of needed seeds");
    }

    let mut needed_seeds: Vec<(u64, u64)> = Vec::new();
    for i in (0..almanac.needed_seeds.len()).step_by(2) {
        needed_seeds.push((almanac.needed_seeds[i], almanac.needed_seeds[i + 1]))
    }

    needed_seeds
        .iter()
        .map(|(start, length)| {
            (*start..(*start + *length))
                .map(|n| almanac.get_location_for_seed(n))
                .min()
                .unwrap()
        })
        .min()
        .unwrap()
}

fn main() {
    let s = include_str!("../input.txt");
    println!("{} {}", solve1(s), solve2(s));
}

#[cfg(test)]
mod tests {
    use crate::{solve1, solve2};

    #[test]
    fn test1() {
        let s = include_str!("../test-input.txt");
        assert_eq!(solve1(s), 35);
    }

    #[test]
    fn test2() {
        let s = include_str!("../test-input.txt");
        assert_eq!(solve2(s), 46);
    }
}
