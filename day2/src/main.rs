mod io;

use io::{read_yaml, Challenge};


fn process_input(challenge: &Challenge) -> Result<u32, ()> {
    let sum = 0;
    for line in challenge.input.lines() {
        println!("{}", line)
    }
    Ok(sum)
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
