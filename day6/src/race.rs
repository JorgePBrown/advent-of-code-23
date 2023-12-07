#[derive(Debug)]
pub struct Race {
    pub record: u64,
    pub time: u64,
}

impl Race {
    pub fn from_stats_2(value: &str) -> Self {
        let mut lines = value.lines();
        let time = lines.next().unwrap()[5..]
            .trim()
            .split_whitespace()
            .map(|s| s.to_owned())
            .reduce(|acc, s| acc + &s)
            .unwrap()
            .parse::<u64>()
            .unwrap();

        let record = lines.next().unwrap()[9..]
            .trim()
            .split_whitespace()
            .map(|s| s.to_owned())
            .reduce(|acc, s| acc + &s)
            .unwrap()
            .parse::<u64>()
            .unwrap();

        Race { time, record }
    }

    pub fn from_stats(value: &str) -> Vec<Self> {
        let mut lines = value.lines();
        let mut times = lines.next().unwrap()[5..]
            .trim()
            .split_whitespace()
            .map(|s| s.parse::<u64>().unwrap());
        let mut distances = lines.next().unwrap()[9..]
            .trim()
            .split_whitespace()
            .map(|s| s.parse::<u64>().unwrap());

        let mut v = Vec::new();

        while let Some(time) = times.next() {
            let record = distances.next().unwrap();

            v.push(Race { time, record })
        }

        v
    }

    pub fn get_number_of_record_beating_solutions(&self) -> u64 {
        let roots = solve_quadratic_equation(-1f64, self.time as f64, -(self.record as f64));

        match roots {
            QuadraticEquationSolution::None => 0,
            QuadraticEquationSolution::One(_) => 1,
            QuadraticEquationSolution::Two(r1, r2) => (r2 as i64 - r1 as i64) as u64,
        }
    }
}

enum QuadraticEquationSolution {
    None,
    One(f64),
    Two(f64, f64),
}
fn solve_quadratic_equation(a: f64, b: f64, c: f64) -> QuadraticEquationSolution {
    if a == 0.0 {
        return QuadraticEquationSolution::None;
    }

    let root = (b.powi(2) - 4f64 * a * c).sqrt();

    let r1 = (-b + root) / (2f64 * a) + 0.000001;
    let r2 = (-b - root) / (2f64 * a) - 0.000001;

    if r1 == r2 {
        QuadraticEquationSolution::One(r1)
    } else {
        if r1 < r2 {
            QuadraticEquationSolution::Two(r1, r2)
        } else {
            QuadraticEquationSolution::Two(r2, r1)
        }
    }
}
