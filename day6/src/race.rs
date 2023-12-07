#[derive(Debug)]
pub struct Race {
    pub record: u32,
    pub time: u32,
}

impl Race {
    pub fn from_stats(value: &str) -> Vec<Self> {
        let mut lines = value.lines();
        let mut times = lines.next().unwrap()[5..]
            .trim()
            .split_whitespace()
            .map(|s| s.parse::<u32>().unwrap());
        let mut distances = lines.next().unwrap()[9..]
            .trim()
            .split_whitespace()
            .map(|s| s.parse::<u32>().unwrap());

        let mut v = Vec::new();

        while let Some(time) = times.next() {
            let record = distances.next().unwrap();

            v.push(Race { time, record })
        }

        v
    }

    pub fn get_number_of_record_beating_solutions(&self) -> u32 {
        let roots = solve_quadratic_equation(-1f32, self.time as f32, -(self.record as f32));

        match roots {
            QuadraticEquationSolution::None => 0,
            QuadraticEquationSolution::One(_) => 1,
            QuadraticEquationSolution::Two(r1, r2) => (r2 as i32 - r1 as i32) as u32,
        }
    }
}

enum QuadraticEquationSolution {
    None,
    One(f32),
    Two(f32, f32),
}
fn solve_quadratic_equation(a: f32, b: f32, c: f32) -> QuadraticEquationSolution {
    if a == 0.0 {
        return QuadraticEquationSolution::None;
    }

    let root = (b.powi(2) - 4f32 * a * c).sqrt();

    let r1 = (-b + root) / (2f32 * a) + 0.000001;
    let r2 = (-b - root) / (2f32 * a) - 0.000001;

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
