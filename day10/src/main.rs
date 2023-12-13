use pipe_loop::PipeLoop;

mod pipe_loop;

fn solve1(input: &str) -> u32 {
    PipeLoop::from(input).longest_distance_from_start()
}

fn main() {
    let s = include_str!("../input.txt");
    println!("{}", solve1(s));
}

#[cfg(test)]
mod tests {
    use crate::solve1;

    #[test]
    fn part12() {
        let s = r"
.....
.S-7.
.|.|.
.L-J.
.....
        "
        .trim();
        assert_eq!(solve1(s), 4);
    }

    #[test]
    fn part1() {
        let s = r"
..F7.
.FJ|.
SJ.L7
|F--J
LJ...
        "
        .trim();
        assert_eq!(solve1(s), 8);
    }
}
