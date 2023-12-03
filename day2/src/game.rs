use core::panic;
use std::usize;

#[derive(Debug, PartialEq)]
pub struct Game {
    pub id: usize,
    pub rounds: Vec<CubeSet>,
}

impl Game {
    pub fn new(id: usize, rounds: Vec<CubeSet>) -> Game {
        Game { id, rounds }
    }
}

impl From<&str> for Game {
    fn from(value: &str) -> Self {
        let game_info_colon = value.find(':').expect("Expected the 'Game <id>:' format.");

        let id = value[5..game_info_colon].parse::<usize>().unwrap();

        let rounds = value[game_info_colon + 1..]
            .split(";")
            .map(|s| CubeSet::from(s))
            .collect();

        Game::new(id, rounds)
    }
}

#[derive(Debug, PartialEq)]
pub struct CubeSet {
    pub blue: usize,
    pub red: usize,
    pub green: usize,
}

impl CubeSet {
    pub fn new(blue: usize, red: usize, green: usize) -> CubeSet {
        CubeSet { blue, red, green }
    }
}

impl From<&str> for CubeSet {
    fn from(s: &str) -> Self {
        let round = s.trim();

        let mut blue = 0;
        let mut red = 0;
        let mut green = 0;

        for cubes in round.split(",") {
            let cubes = cubes.trim();

            let space = cubes.find(' ').expect("Expected '<n> <color>' format");

            let color = &cubes[space + 1..];
            let n = cubes[0..space].parse::<usize>().unwrap();

            match color {
                "blue" => blue = n,
                "red" => red = n,
                "green" => green = n,
                _ => panic!("Unknown color"),
            }
        }

        CubeSet::new(blue, red, green)
    }
}
