use core::panic;
use std::collections::HashMap;

pub enum Instruction {
    Left,
    Right,
}

pub struct Map {
    pub instructions: Vec<Instruction>,
    locations: HashMap<String, (String, String)>,
}

impl Map {
    pub fn get_location(&self, start: &str, instruction: &Instruction) -> &str {
        match instruction {
            Instruction::Left => &self.locations.get(start).unwrap().0,
            Instruction::Right => &self.locations.get(start).unwrap().1,
        }
    }
}

impl From<&str> for Map {
    fn from(value: &str) -> Self {
        let mut lines = value.lines();

        let instructions = lines
            .next()
            .unwrap()
            .chars()
            .map(|c| match c {
                'L' => Instruction::Left,
                'R' => Instruction::Right,
                _ => panic!("Don't know this instruction {c}"),
            })
            .collect();

        let lines = lines.skip(1);

        let locations = lines
            .map(|l| {
                let start = l[0..3].to_owned();

                let left = l[7..10].to_owned();
                let right = l[12..15].to_owned();

                (start, (left, right))
            })
            .collect();

        Map {
            instructions,
            locations,
        }
    }
}
