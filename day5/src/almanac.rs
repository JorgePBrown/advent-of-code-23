use std::str::Lines;

#[derive(Debug)]
pub struct Almanac {
    pub needed_seeds: Vec<u64>,
    seed_to_soil: Vec<(u64, u64, u64)>,
    soil_to_fertilizer: Vec<(u64, u64, u64)>,
    fertilizer_to_water: Vec<(u64, u64, u64)>,
    water_to_light: Vec<(u64, u64, u64)>,
    light_to_temperature: Vec<(u64, u64, u64)>,
    temperature_to_humidity: Vec<(u64, u64, u64)>,
    humidity_to_location: Vec<(u64, u64, u64)>,
}

impl Almanac {
    pub fn get_location_for_seed(self: &Self, seed: u64) -> u64 {
        Almanac::find_range_for_value(
            Almanac::find_range_for_value(
                Almanac::find_range_for_value(
                    Almanac::find_range_for_value(
                        Almanac::find_range_for_value(
                            Almanac::find_range_for_value(
                                Almanac::find_range_for_value(seed, &self.seed_to_soil),
                                &self.soil_to_fertilizer,
                            ),
                            &self.fertilizer_to_water,
                        ),
                        &self.water_to_light,
                    ),
                    &self.light_to_temperature,
                ),
                &self.temperature_to_humidity,
            ),
            &self.humidity_to_location,
        )
    }

    fn find_range_for_value(v: u64, ranges: &Vec<(u64, u64, u64)>) -> u64 {
        // binary search
        let mut left = 0;
        let mut right = ranges.len() - 1;
        let mut range = None;
        while left <= right {
            let middle = (left + right) / 2;

            let m_value = ranges[middle];

            if m_value.1 <= v && m_value.1 + m_value.2 - 1 >= v {
                range = Some(m_value);
                break;
            } else if m_value.1 > v {
                if middle == 0 {
                    break;
                }
                right = middle - 1;
            } else {
                left = middle + 1;
            }
        }

        match range {
            Some(r) => return r.0 + (v - r.1),
            None => v,
        }
    }
}

impl From<&str> for Almanac {
    fn from(value: &str) -> Self {
        let mut lines = value.lines();

        let needed_seeds = lines.next().unwrap()[7..]
            .trim()
            .split(" ")
            .map(|s| s.parse::<u64>().unwrap())
            .collect();

        skip_lines_to_text(&mut lines);
        let seed_to_soil = get_map_from_lines(&mut lines);

        skip_lines_to_text(&mut lines);
        let soil_to_fertilizer = get_map_from_lines(&mut lines);

        skip_lines_to_text(&mut lines);
        let fertilizer_to_water = get_map_from_lines(&mut lines);

        skip_lines_to_text(&mut lines);
        let water_to_light = get_map_from_lines(&mut lines);

        skip_lines_to_text(&mut lines);
        let light_to_temperature = get_map_from_lines(&mut lines);

        skip_lines_to_text(&mut lines);
        let temperature_to_humidity = get_map_from_lines(&mut lines);

        skip_lines_to_text(&mut lines);
        let humidity_to_location = get_map_from_lines(&mut lines);

        Almanac {
            needed_seeds,
            seed_to_soil,
            soil_to_fertilizer,
            fertilizer_to_water,
            water_to_light,
            light_to_temperature,
            temperature_to_humidity,
            humidity_to_location,
        }
    }
}

fn skip_lines_to_text(lines: &mut Lines) {
    for s in lines {
        if s.len() > 0 {
            break;
        }
    }
}

fn get_map_from_lines(lines: &mut Lines) -> Vec<(u64, u64, u64)> {
    let mut v = vec![];

    for s in lines {
        if s.len() == 0 {
            break;
        }

        let values = s
            .split_ascii_whitespace()
            .map(|s| s.parse::<u64>().unwrap())
            .collect::<Vec<u64>>();

        v.push((values[0], values[1], values[2]));
    }

    v.sort_by(|(_, v2, _), (_, v21, _)| v2.cmp(&v21));

    v
}
