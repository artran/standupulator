use std::fs::File;
use std::io::{self, BufRead};

use rand::{thread_rng, seq::SliceRandom};

fn main() {
    let mut rng = thread_rng();

    let mut names = get_names_from_file();
    names.shuffle(&mut rng);

    println!("Today's order:   {:?}", names);
}

fn get_names_from_file() -> Vec<String> {
    // If the file exists and is readable we read the first line and split it into the result Vector
    // Otherwise we get the names from the defaults

    let file_result = File::open("input.txt");
    match file_result {
        Ok(file) => {
            let mut line = String::new();
            let mut reader = io::BufReader::new(file);

            match reader.read_line(&mut line) {
                Ok(_ok) => line.split(",").map(|s| s.trim()).map(|s| s.to_owned()).collect(),
                Err(_err) => get_default_names(),
            }
        },
        Err(_err) => get_default_names()
    }
}

fn get_default_names() -> Vec<String> {
    vec![
        "Al".to_owned(),
        "Huw".to_owned(),
        "John".to_owned(),
        "Martin".to_owned(),
        "Neil".to_owned(),
        "Ray".to_owned(),
        "Robin".to_owned(),
        "Steve".to_owned(),
        "Will".to_owned(),
    ]
}
