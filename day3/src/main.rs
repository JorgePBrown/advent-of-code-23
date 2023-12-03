fn solve1(input: &str) -> usize {
    let mut lines = input.lines();

    let mut current_line = lines.next().unwrap().chars().collect();
    let mut next_line_opt = lines.next();

    if next_line_opt.is_none() {
        return check_line(&current_line, &vec![]);
    }
    let mut next_line = next_line_opt.unwrap().chars().collect();

    let mut sum = 0;
    sum += check_line(&current_line, &vec![&next_line]);

    loop {
        let prev_line = current_line;
        current_line = next_line;
        next_line_opt = lines.next();

        if next_line_opt.is_none() {
            sum += check_line(&current_line, &vec![&prev_line]);
            break;
        }
        next_line = next_line_opt.unwrap().chars().collect();
        sum += check_line(&current_line, &vec![&prev_line, &next_line]);
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

fn solve2(input: &str) -> u32 {
    let mut lines = input.lines();

    let mut current_line = lines.next().unwrap().chars().collect();
    let mut next_line_opt = lines.next();

    if next_line_opt.is_none() {
        return check_line_ratio(&current_line, &vec![]);
    }
    let mut next_line = next_line_opt.unwrap().chars().collect();

    let mut sum = 0;
    sum += check_line_ratio(&current_line, &vec![&next_line]);

    loop {
        let prev_line = current_line;
        current_line = next_line;
        next_line_opt = lines.next();

        if next_line_opt.is_none() {
            sum += check_line_ratio(&current_line, &vec![&prev_line]);
            break;
        }
        next_line = next_line_opt.unwrap().chars().collect();
        sum += check_line_ratio(&current_line, &vec![&prev_line, &next_line]);
    }

    sum
}

fn check_line_ratio(line: &Vec<char>, adjacents: &Vec<&Vec<char>>) -> u32 {
    let mut sum = 0;
    for (i, c) in line.iter().enumerate() {
        if *c == '*' {
            let mut part_numbers = Vec::<u32>::new();

            if i > 0 && line[i - 1].is_digit(10) {
                part_numbers.push(get_surrounding_number(line, i - 1));
            }
            if i < line.len() - 1 && line[i + 1].is_digit(10) {
                part_numbers.push(get_surrounding_number(line, i + 1));
            }

            for a in adjacents {
                if a[i].is_digit(10) {
                    part_numbers.push(get_surrounding_number(&a, i));
                } else {
                    if i > 0 && a[i - 1].is_digit(10) {
                        part_numbers.push(get_surrounding_number(&a, i - 1));
                    }
                    if i < line.len() - 1 && a[i + 1].is_digit(10) {
                        part_numbers.push(get_surrounding_number(&a, i + 1));
                    }
                }
            } 

            if part_numbers.len() == 2 {
                sum += part_numbers[0] * part_numbers[1];
            }
        }
    } 

    sum
} 

fn get_surrounding_number(vec: &Vec<char>, idx: usize) -> u32 {
    // go to start
    let mut i = idx;
    for (j, c) in vec[0..idx].iter().enumerate().rev() {
        if !c.is_digit(10) {
            break;
        }
        i = j;
    }
    let mut sum = 0;
    for c in vec[i..].iter() {
        match c.to_digit(10) {
            Some(n) => { sum *= 10; sum += n; },
            None => break
        }
    }
    sum
}

fn main() {
    let s = include_str!("../input.txt");
    println!("{}, {}", solve1(s), solve2(s));
}

#[cfg(test)]
mod tests {
    use crate::{solve1, solve2};

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
    #[test]
    fn part2() {
        let s = include_str!("../test-input.txt");
        assert_eq!(solve2(s), 467835);
    }
}
