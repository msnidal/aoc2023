use serde::{Deserialize, Serialize};
use std::fs::File;
use std::io::Read;
use std::path::Path;

#[derive(Serialize, Deserialize)]
pub struct Challenge {
    pub input: String,
    pub output: Option<String>,
}

pub fn read_yaml<P: AsRef<Path>>(path: P) -> Result<Challenge, serde_yaml::Error> {
    let mut file = File::open(path).expect("File not found");
    let mut contents = String::new();
    file.read_to_string(&mut contents)
        .expect("Something went wrong reading the file");
    serde_yaml::from_str(&contents)
}
