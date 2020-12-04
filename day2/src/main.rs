use std::fs::File;
use std::io::{BufRead, BufReader};

fn validate_password(min: usize, max: usize, letter: char, password: String) -> bool {
    let instances: usize = password.matches(letter).count();
    if instances >= min && instances <= max {
        return true
    }
    return false
}

fn parse_line(line: String) -> Option<(usize, usize, char, String)> {
    // Split by space into parts
    let line_split: Vec<&str> = line.split_whitespace().collect();
    if line_split.len() != 3 {
        return None
    }

    // Get Min and Max
    let num_split: Vec<&str> = line_split.get(0).expect("Could not get 0th element of split list").split('-').collect();
    if num_split.len() != 2 {
        return None
    }
    let min: usize  = num_split.get(0).unwrap().parse::<usize>().expect("Could not parse min usize");
    let max: usize  = num_split.get(1).unwrap().parse::<usize>().expect("Could not parse max usize");

    // Get letter
    let mut raw_letter: String = line_split.get(1).expect("Could not get 1st element of split list").to_string();
    raw_letter.truncate(1);
    let ret_char: char = raw_letter.chars().next().expect("Empty string when converting to char");

    return Some((min, max, ret_char, line_split.get(2).expect("Could not get 2nd element of split list").to_string()));
}

fn main() {
    let file = File::open("./input");
    let file: File = match file {
        Ok(f) => f,
        Err(err) => {
            panic!("Could not open file: {:?}", err);
        }
    };

    let reader: BufReader<File> = BufReader::new(file);
    let mut in_data: Vec<String> = Vec::new();

    for (_idx, line) in reader.lines().enumerate() {
        let line = match line {
            Ok(l) => l,
            Err(err) => {
                panic!("Could not read line: {:?}", err);

            }
        };
        in_data.push(line);
    }

    let mut num_valid: i32 = 0;

    for line in in_data.iter() {
        match parse_line(line.to_string()) {
            Some(l) => {
                if validate_password(l.0, l.1, l.2, l.3) {
                    num_valid = num_valid + 1;
                }
            },
            None => panic!("There was an error parsing the line {:?}", line),
        };
    }

    println!("Found {:?} valid passwords", num_valid);
}
