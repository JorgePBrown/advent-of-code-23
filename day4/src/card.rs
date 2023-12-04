pub struct Card {
    pub id: u32,
    pub winning_numbers: Vec<u32>,
    pub scratch_numbers: Vec<u32>,
}

impl Card {
    pub fn get_points(self: &Self) -> u32 {
        let mut wins = 0;
        for wn in self.winning_numbers.iter() {
            for sn in self.scratch_numbers.iter() {
                if wn == sn {
                    wins += 1;
                }
            }
        }
        if wins > 0 {
            2u32.pow(wins - 1)
        } else {
            0
        }
    }

}

impl From<&str> for Card {
    fn from(value: &str) -> Self {
        let colon_idx = value
            .find(':')
            .expect(&format!("Expected 'Card <id>:...' format. Got {value}"));

        let id = value[5..colon_idx].trim().parse::<u32>().expect(&format!(
            "Colon idx: {colon_idx}. Trying to parse '{}'",
            &value[5..colon_idx]
        ));

        let pipe_idx = value.find('|').expect(&format!(
            "Expected 'Card <id>: <winning numbers> | <scratch numbers>' format. Got {value}"
        ));

        let winning_numbers = value[colon_idx + 1..pipe_idx]
            .trim()
            .split(" ")
            .filter(|s| s.len() > 0)
            .map(|s| s.parse::<u32>().unwrap())
            .collect();
        let scratch_numbers = value[pipe_idx + 1..]
            .trim()
            .split(" ")
            .filter(|s| s.len() > 0)
            .map(|s| s.parse::<u32>().unwrap())
            .collect();

        Card {
            id,
            winning_numbers,
            scratch_numbers,
        }
    }
}
