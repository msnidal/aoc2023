mod io;
mod trie;

use io::{read_yaml, Challenge};
use std::collections::HashMap;
use trie::TrieNode;

fn build_number_word_trie(number_map: &HashMap<&str, u32>) -> TrieNode {
    let mut root = TrieNode::new();

    for (word, _) in number_map.iter() {
        root.insert(word);
    }

    root
}

fn process_string(input: &str, trie: &TrieNode, number_map: &HashMap<&str, u32>) -> Vec<u32> {
    let mut numbers = Vec::new();
    let mut potential_matches = vec![trie];

    for ch in input.chars() {
        let mut new_matches = Vec::new();

        for node in potential_matches.iter() {
            if let Some(next_node) = node.children.get(&ch) {
                new_matches.push(next_node);
                if next_node.is_end_of_word.is_some() {
                    // Found complete word
                    let str_ref = next_node.is_end_of_word.as_ref().unwrap().as_str();
                    if let Some(&number) = number_map.get(str_ref) {
                        numbers.push(number);
                    }
                }
            }
        }

        if ch.is_digit(10) {
            numbers.push(ch.to_digit(10).unwrap());
            new_matches.clear();
        }

        new_matches.push(trie); // Always include the root for new potential matches
        potential_matches = new_matches;
    }

    println!(
        "String: {}\nNumbers: {}",
        input,
        numbers
            .iter()
            .map(|&n| n.to_string())
            .collect::<Vec<String>>()
            .join(", ")
    );

    numbers
}

fn process_input(challenge: &Challenge) -> Result<u32, ()> {
    let lines: Vec<String> = challenge.input.lines().map(String::from).collect();
    let number_map: HashMap<&str, u32> = [
        ("one", 1),
        ("two", 2),
        ("three", 3),
        ("four", 4),
        ("five", 5),
        ("six", 6),
        ("seven", 7),
        ("eight", 8),
        ("nine", 9),
    ]
    .iter()
    .cloned()
    .collect();
    let trie = build_number_word_trie(&number_map);

    let mut challenge_sum: u32 = 0;
    for line in lines {
        let digits = process_string(&line, &trie, &number_map);
        if let (Some(&first), Some(&last)) = (digits.first(), digits.last()) {
            let line_number = first * 10 + last;
            challenge_sum += line_number;
        } else {
            println!("No digits found in line: {}", line);
        }
    }

    Ok(challenge_sum)
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
