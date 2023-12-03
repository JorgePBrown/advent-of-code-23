fn main() {
    let s = include_str!("../input.txt");

    let mut v = 0;

    for line in s.lines() {
        let first_digit: u32 = get_first_digit(&line)
            .expect(&format!("There to be a first digit for line {line}"))
            .to_digit(10)
            .unwrap();
        let last_digit: u32 = get_last_digit(&line)
            .expect(&format!("There to be a last digit for line {line}"))
            .to_digit(10)
            .unwrap();

        v += first_digit * 10 + last_digit;
    }

    println!("{v}");
}

const ONE_ARRAY: [char; 3] = ['o', 'n', 'e'];
const TWO_ARRAY: [char; 3] = ['t', 'w', 'o'];
const THREE_ARRAY: [char; 5] = ['t', 'h', 'r', 'e', 'e'];
const FOUR_ARRAY: [char; 4] = ['f', 'o', 'u', 'r'];
const FIVE_ARRAY: [char; 4] = ['f', 'i', 'v', 'e'];
const SIX_ARRAY: [char; 3] = ['s', 'i', 'x'];
const SEVEN_ARRAY: [char; 5] = ['s', 'e', 'v', 'e', 'n'];
const EIGHT_ARRAY: [char; 5] = ['e', 'i', 'g', 'h', 't'];
const NINE_ARRAY: [char; 4] = ['n', 'i', 'n', 'e'];

const REVERSED_ONE_ARRAY: [char; 3] = ['e', 'n', 'o'];
const REVERSED_TWO_ARRAY: [char; 3] = ['o', 'w', 't'];
const REVERSED_THREE_ARRAY: [char; 5] = ['e', 'e', 'r', 'h', 't'];
const REVERSED_FOUR_ARRAY: [char; 4] = ['r', 'u', 'o', 'f'];
const REVERSED_FIVE_ARRAY: [char; 4] = ['e', 'v', 'i', 'f'];
const REVERSED_SIX_ARRAY: [char; 3] = ['x', 'i', 's'];
const REVERSED_SEVEN_ARRAY: [char; 5] = ['n', 'e', 'v', 'e', 's'];
const REVERSED_EIGHT_ARRAY: [char; 5] = ['t', 'h', 'g', 'i', 'e'];
const REVERSED_NINE_ARRAY: [char; 4] = ['e', 'n', 'i', 'n'];

fn get_first_digit(input: &str) -> Option<char> {
    let mut char_counters: Vec<(usize, &[char])> = vec![
        (0, &ONE_ARRAY),
        (0, &TWO_ARRAY),
        (0, &THREE_ARRAY),
        (0, &FOUR_ARRAY),
        (0, &FIVE_ARRAY),
        (0, &SIX_ARRAY),
        (0, &SEVEN_ARRAY),
        (0, &EIGHT_ARRAY),
        (0, &NINE_ARRAY),
    ];

    for c in input.chars() {
        if c >= '0' && c <= '9' {
            return Some(c);
        }

        for (i, cc) in char_counters.iter_mut().enumerate() {
            if cc.1[cc.0] == c {
                cc.0 += 1;
                if cc.0 >= cc.1.len() {
                    return char::from_digit((i + 1).try_into().unwrap(), 10);
                }
            } else {
                if cc.1[0] == c {
                    cc.0 = 1;
                } else {
                    cc.0 = 0;
                }
            }
        }
    }
    return None;
}
fn get_last_digit(input: &str) -> Option<char> {
    let mut char_counters: Vec<(usize, &[char])> = vec![
        (0, &REVERSED_ONE_ARRAY),
        (0, &REVERSED_TWO_ARRAY),
        (0, &REVERSED_THREE_ARRAY),
        (0, &REVERSED_FOUR_ARRAY),
        (0, &REVERSED_FIVE_ARRAY),
        (0, &REVERSED_SIX_ARRAY),
        (0, &REVERSED_SEVEN_ARRAY),
        (0, &REVERSED_EIGHT_ARRAY),
        (0, &REVERSED_NINE_ARRAY),
    ];

    for c in input.chars().rev() {
        if c >= '0' && c <= '9' {
            return Some(c);
        }

        for (i, cc) in char_counters.iter_mut().enumerate() {
            if cc.1[cc.0] == c {
                cc.0 += 1;
                if cc.0 >= cc.1.len() {
                    return char::from_digit((i + 1).try_into().unwrap(), 10);
                }
            } else {
                if cc.1[0] == c {
                    cc.0 = 1;
                } else {
                    cc.0 = 0;
                }
            }
        }
    }
    return None;
}

#[cfg(test)]
mod tests {
    use crate::get_first_digit;

    #[test]
    fn a() {
        assert_eq!(get_first_digit("ffour87fqrvqxqlqrrk"), Some('4'));
    }
}
