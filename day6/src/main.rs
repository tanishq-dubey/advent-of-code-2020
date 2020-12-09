use std::fs::File;
use std::io::{BufRead, BufReader};
use std::collections::HashSet;

fn get_unique_question_count(raw: String) -> usize {
    let mut uniques: HashSet<char> = HashSet::new();
    for c in raw.chars() {
        uniques.insert(c);
    }
    return uniques.len();
}

fn clean_questions(raw: Vec<String>) -> Vec<String> {
    let mut clean_line: String = "".to_string();
    let mut ret_vec: Vec<String> = Vec::new();

    for line in raw.iter() {
        if line == "" {
            ret_vec.push(clean_line.trim().to_string());
            clean_line = "".to_string();
        } else {
            clean_line = clean_line + line;
        }
    }
    ret_vec.push(clean_line.trim().to_string());

    return ret_vec;
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

    let clean_data = clean_questions(in_data);
    
    let mut sum = 0;

    for c in clean_data.iter() {
        sum = sum + get_unique_question_count(c.to_string());
    }
    println!("{:?}", sum);
}
