use game::Game;

mod game;

fn solve(games: &Vec<Game>, (m_blue, m_red, m_green): (usize, usize, usize)) -> usize {
    let mut sum = 0;
    for game in games.iter() {
        let mut possible = true;
        for round in game.rounds.iter() {
            if round.blue > m_blue { possible = false; break; }
            if round.red > m_red { possible = false; break; }
            if round.green > m_green { possible = false; break; }
        }
        if possible {
            sum += game.id;
        }
    }
    sum
}

fn solve2(games: &Vec<Game>) -> usize {
    let mut sum = 0;

    for game in games.iter() {
        let mut min_blue = 0;
        let mut min_red = 0;
        let mut min_green = 0;

        for round in game.rounds.iter() {
            if round.blue > min_blue { min_blue = round.blue; }
            if round.red > min_red { min_red = round.red; }
            if round.green > min_green { min_green = round.green; }
        }

        sum += min_blue * min_red * min_green;
    }

    sum
}

fn main() {
    let input = include_str!("../input.txt");
    let games = input.lines().map(|line| { Game::from(line) }).collect::<Vec<Game>>();

    println!("{} {}", solve(&games, (14, 12, 13)), solve2(&games));
}

#[cfg(test)]
mod tests {
    use crate::{solve, game::{Game, CubeSet}, solve2};

    #[test]
    fn base() {
        let games = vec![
            Game::new(1, vec![
                CubeSet::new(3, 4, 0),
                CubeSet::new(6, 1, 2),
                CubeSet::new(0, 0, 2),
            ]),
            Game::new(2, vec![
                CubeSet::new(1, 0, 2),
                CubeSet::new(4, 1, 3),
                CubeSet::new(1, 0, 1),
            ]),
            Game::new(3, vec![
                CubeSet::new(6, 20, 8),
                CubeSet::new(5, 4, 13),
                CubeSet::new(0, 1, 5),
            ]),
            Game::new(4, vec![
                CubeSet::new(6, 3, 1),
                CubeSet::new(0, 6, 3),
                CubeSet::new(15, 14, 3),
            ]),
            Game::new(5, vec![
                CubeSet::new(1, 6, 3),
                CubeSet::new(2, 1, 2),
            ]),
        ];
        assert_eq!(solve(&games, (14, 12, 13)), 8);
    }

    #[test]
    fn parse() {
        let input = include_str!("../test-input.txt");
        let games = input.lines().map(|line| { Game::from(line) }).collect::<Vec<Game>>();

        assert_eq!(games, 
        vec![
        Game::new(1, vec![
        CubeSet::new(3, 4, 0),
        CubeSet::new(6, 1, 2),
        CubeSet::new(0, 0, 2),
        ]),
        Game::new(2, vec![
        CubeSet::new(1, 0, 2),
        CubeSet::new(4, 1, 3),
        CubeSet::new(1, 0, 1),
        ]),
        Game::new(3, vec![
        CubeSet::new(6, 20, 8),
        CubeSet::new(5, 4, 13),
        CubeSet::new(0, 1, 5),
        ]),
        Game::new(4, vec![
        CubeSet::new(6, 3, 1),
        CubeSet::new(0, 6, 3),
        CubeSet::new(15, 14, 3),
        ]),
        Game::new(5, vec![
        CubeSet::new(1, 6, 3),
        CubeSet::new(2, 1, 2),
        ]),
        ])
    }

    #[test]
    fn test_solve2() {
        let games = vec![
            Game::new(1, vec![
                CubeSet::new(3, 4, 0),
                CubeSet::new(6, 1, 2),
                CubeSet::new(0, 0, 2),
            ]),
            Game::new(2, vec![
                CubeSet::new(1, 0, 2),
                CubeSet::new(4, 1, 3),
                CubeSet::new(1, 0, 1),
            ]),
            Game::new(3, vec![
                CubeSet::new(6, 20, 8),
                CubeSet::new(5, 4, 13),
                CubeSet::new(0, 1, 5),
            ]),
            Game::new(4, vec![
                CubeSet::new(6, 3, 1),
                CubeSet::new(0, 6, 3),
                CubeSet::new(15, 14, 3),
            ]),
            Game::new(5, vec![
                CubeSet::new(1, 6, 3),
                CubeSet::new(2, 1, 2),
            ]),
        ];
        assert_eq!(solve2(&games), 2286);
    }
}
