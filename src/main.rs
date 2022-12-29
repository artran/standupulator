use std::fs::File;
use std::io::{self, BufRead};

use rand::{thread_rng, seq::SliceRandom};

fn main() {
    let file_result = File::open("input.txt");
    let mut line = String::new();
    let mut names = match file_result {
        Ok(file) => {
            let mut reader = io::BufReader::new(file);

            let names = match reader.read_line(&mut line) {
                Ok(_ok) => {
                    line.split(",").map(|s| s.trim()).collect()
                },
                Err(_err) => get_default_names(),
            };
            names
        },
        Err(_error) => get_default_names()
    };

    let mut rng = thread_rng();
    names.shuffle(&mut rng);
    println!("Today's order:   {:?}", names);
}


fn get_default_names() -> Vec<&'static str> {
    vec!["Ray", "Martin", "Robin", "Huw", "Al", "Steve", "Neil", "Will", "John"]
}
