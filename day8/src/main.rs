use map::Map;

mod map;

fn solve1(input: &str) -> u64 {
    let map = Map::from(input);

    let mut moves = 0;
    let mut current_location = "AAA";
    for instruction in map.instructions.iter().cycle() {
        current_location = map.get_location(current_location, instruction);
        moves += 1;
        if current_location == "ZZZ" {
            break;
        }
    }

    moves
}

fn main() {
    let s = include_str!("../input.txt");
    println!("{}", solve1(s));
}

#[cfg(test)]
mod tests {
    use crate::solve1;

    #[test]
    fn part11() {
        let s = include_str!("../test-input.txt");
        assert_eq!(solve1(s), 2);
    }

    #[test]
    fn part12() {
        let s = include_str!("../test-input-2.txt");
        assert_eq!(solve1(s), 6);
    }
}
