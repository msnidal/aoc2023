mod io;

use io::{read_yaml, Challenge};
use regex::Regex;

struct Game {
    id: u32,
    rounds: Vec<Round>,
}

struct Round {
    blue: u32,
    red: u32,
    green: u32,
}

impl Game {
    fn from_str(input: &str) -> Self {
        let game_regex = Regex::new(r"Game (\d+): (.*)").unwrap();
        let round_regex = Regex::new(r"(\d+) (blue|red|green)").unwrap();

        let caps = game_regex.captures(input).unwrap();
        let id: u32 = caps[1].parse().unwrap();
        let rounds_str = &caps[2];

        let mut rounds = Vec::new();
        for caps in round_regex.captures_iter(rounds_str) {
            let count: u32 = caps[1].parse().unwrap();
            let color = &caps[2];
            match color {
                "blue" => rounds.push(Round {
                    blue: count,
                    red: 0,
                    green: 0,
                }),
                "red" => rounds.push(Round {
                    blue: 0,
                    red: count,
                    green: 0,
                }),
                "green" => rounds.push(Round {
                    blue: 0,
                    red: 0,
                    green: count,
                }),
                _ => (),
            }
        }

        Self { id, rounds }
    }
}

fn process_input(challenge: &Challenge) -> Result<u32, ()> {
    let mut power_sum: u32 = 0;
    for line in challenge.input.lines() {
        let game = Game::from_str(line);

        let max_blue = game
            .rounds
            .iter()
            .map(|round| round.blue)
            .max()
            .unwrap_or(0);
        let max_red = game.rounds.iter().map(|round| round.red).max().unwrap_or(0);
        let max_green = game
            .rounds
            .iter()
            .map(|round| round.green)
            .max()
            .unwrap_or(0);

        power_sum += max_blue * max_red * max_green;
    }
    Ok(power_sum)
}

fn main() {
    let args: Vec<String> = std::env::args().collect();
    let part = args.get(1).expect("No part provided");
    let filename = args.get(2).expect("No filename provided");

    let challenge = read_yaml(format!("data/{part}/{filename}")).expect("Failed to read yaml");
    let result = process_input(&challenge).unwrap();

    if challenge.output.is_some() {
        assert!(
            challenge.output.unwrap().parse::<u32>().unwrap() == result,
            "Challenge output does not match"
        );
    }
    println!("Challenge sum: {}", result);
}
