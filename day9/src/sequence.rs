use std::cmp::max;

#[derive(Debug)]
pub struct Sequence {
    values: Vec<i64>,
}

impl Sequence {
    pub fn get_next_value(&self) -> i64 {
        Sequence::get_next_value_rec(&self.values)
    }
    fn get_next_value_rec(values: &[i64]) -> i64 {
        if values.iter().all(|v| *v == 0) {
            return 0;
        }

        let mut diff_vec = Vec::with_capacity(max(1, values.len() - 1));

        diff_vec.push(values[0]);
        for i in 1..values.len() - 1 {
            diff_vec[i - 1] = values[i] - diff_vec[i - 1];
            diff_vec.push(values[i]);
        }
        let last_idx = diff_vec.len() - 1;
        diff_vec[last_idx] = values[values.len() - 1] - diff_vec[last_idx];

        values.last().unwrap() + Sequence::get_next_value_rec(&diff_vec)
    }
}

impl From<&str> for Sequence {
    fn from(value: &str) -> Self {
        let values = value
            .split_whitespace()
            .map(|s| s.parse::<i64>().unwrap())
            .collect();

        Sequence { values }
    }
}
