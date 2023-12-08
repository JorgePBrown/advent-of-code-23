use core::panic;
use std::collections::HashMap;

#[derive(Clone)]
pub enum Instruction {
    Left,
    Right,
}

pub struct Map {
    pub instructions: Vec<Instruction>,
    locations: HashMap<String, (String, String)>,
}

pub struct LinkedMap {
    pub instructions: Vec<Instruction>,
    starting_locations: Vec<usize>,
    locations: Vec<(String, (usize, usize))>,
}
impl LinkedMap {
    pub fn get_location(&self, start: usize, instruction: &Instruction) -> usize {
        match instruction {
            Instruction::Left => self.locations[start].1.0,
            Instruction::Right => self.locations[start].1.1,
        }
    }

    pub fn are_all_ending_locations(&self, locations: &[usize]) -> bool {
        locations.iter().all(|location| {
            &self.locations[*location].0[2..3] == "Z"
        })
    }

    pub fn get_starting_locations(&self) -> Vec<usize> {
        self.starting_locations.clone()
    }
}

impl Map {
    pub fn get_location(&self, start: &str, instruction: &Instruction) -> &str {
        match instruction {
            Instruction::Left => &self.locations.get(start).unwrap().0,
            Instruction::Right => &self.locations.get(start).unwrap().1,
        }
    }

    pub fn as_linked(&self) -> LinkedMap {
        let mut locations = Vec::with_capacity(self.locations.len());
        let mut starting_locations = Vec::new();

        let mut indices: HashMap<String, usize> = HashMap::new();

        for k in self.locations.keys() {
            indices.insert(k.clone(), locations.len());
            if &k[2..] == "A" {
                starting_locations.push(locations.len());
            }
            locations.push((k.clone(), (0, 0)));
        }

        for k in self.locations.keys() {
            let idx = indices.get(k).unwrap();

            let (left, right) = self.locations.get(k).unwrap();

            locations.get_mut(*idx).unwrap().1 = (*indices.get(left).unwrap(), *indices.get(right).unwrap());
        }


        LinkedMap { instructions: self.instructions.clone(), starting_locations, locations }
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
