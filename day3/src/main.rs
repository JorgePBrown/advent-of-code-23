fn solve1(input: &str) -> usize {
    let mut lines = input.lines();

    let mut current_line = lines.next().unwrap();
    let mut next_line_opt = lines.next();

    if next_line_opt.is_none() {
        return check_line(&current_line.chars().collect(), &vec![]);
    }
    let mut next_line = next_line_opt.unwrap();

    let mut sum = 0;
    sum += check_line(&current_line.chars().collect(), &vec![&next_line.chars().collect()]);

    loop {
        let prev_line = current_line;
        current_line = next_line;
        next_line_opt = lines.next();

        if next_line_opt.is_none() {
            sum += check_line(&current_line.chars().collect(), &vec![&prev_line.chars().collect()]);
            break;
        }
        next_line = next_line_opt.unwrap();
        sum += check_line(&current_line.chars().collect(), &vec![&prev_line.chars().collect(), &next_line.chars().collect()]);
    }

    sum
}

fn check_line(line: &Vec<char>, adjacents: &Vec<&Vec<char>>) -> usize {
    let mut sum = 0;
    let mut tmp_sum = 0;
 
    let mut adjacent = false;
    for i in 0..line.len() {
        let c = line[i];

        for a in adjacents.iter() {
            if a[i] != '.' && !a[i].is_digit(10) {
                adjacent = true;
            }
        }

        if c.is_digit(10) {
            if i > 0 && tmp_sum == 0 {
                if line[i - 1] != '.' && !line[i - 1].is_digit(10) {
                    adjacent = true;
                }
                for a in adjacents.iter() {
                    if a[i - 1] != '.' && !a[i - 1].is_digit(10) {
                        adjacent = true;
                    }
                }
            }

            tmp_sum *= 10;
            tmp_sum += c.to_digit(10).unwrap();
        } else {
            if c != '.' {
                adjacent = true;
            }
            if adjacent {
                sum += tmp_sum;
                adjacent = false;
            }
            tmp_sum = 0;
        }
    }
    if adjacent {
        sum += tmp_sum;
    }

    sum.try_into().unwrap()
} 

fn main() {
    let s = include_str!("../input.txt");
    println!("{}", solve1(s));
}

#[cfg(test)]
mod tests {
    use crate::solve1;

    #[test]
    fn minipart1() {
        let s = "467..114..";
        assert_eq!(solve1(s), 0);
    }

    #[test]
    fn mini2part1() {
        let s = "467..114..\n**********";
        assert_eq!(solve1(s), 467 + 114);
    }

    #[test]
    fn mini3part1() {
        let s = "467..114\n********";
        assert_eq!(solve1(s), 467 + 114);
    }

    #[test]
    fn mini4part1() {
        let s = "467*.114";
        assert_eq!(solve1(s), 467);
    }

    #[test]
    fn fakeadjacent() {
        let s = "..467..114\n*.........";
        assert_eq!(solve1(s), 0);
    }

    #[test]
    fn part1() {
        let s = include_str!("../test-input.txt");
        assert_eq!(solve1(s), 4361);
    }
}
