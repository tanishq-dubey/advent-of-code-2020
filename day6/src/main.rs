use std::fs::File;
use std::io::{BufRead, BufReader};
use std::collections::HashSet;
use std::collections::HashMap;

fn get_unique_question_count(raw: String) -> usize {
    let mut uniques: HashSet<char> = HashSet::new();
    for c in raw.chars() {
        uniques.insert(c);
    }
    return uniques.len();
}

fn get_unique_group_question_count(raw: String, group_size: i32) -> usize {
    let mut uniques: HashMap<char, i32> = HashMap::new();
    for c in raw.chars() {
        if uniques.contains_key(&c) {
            uniques.insert(c, uniques.get(&c).unwrap() + 1);
        } else {
            uniques.insert(c, 1);
        }
    }
    let mut sum = 0;
    for (_, v) in uniques.iter() {
        if *v == group_size {
            sum = sum + 1;
        }
    }
    return sum;
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

fn clean_questions_with_group(raw: Vec<String>) -> Vec<(String, i32)> {
    let mut clean_line: String = "".to_string();
    let mut count: i32 = 0;
    let mut ret_vec: Vec<(String, i32)> = Vec::new();

    for line in raw.iter() {
        if line == "" {
            ret_vec.push((clean_line.trim().to_string(), count));
            clean_line = "".to_string();
            count = 0;
        } else {
            clean_line = clean_line + line;
            count = count + 1;
        }
    }
    ret_vec.push((clean_line.trim().to_string(), count));

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

    let clean_data = clean_questions(in_data.clone());
    
    let mut sum = 0;

    for c in clean_data.iter() {
        sum = sum + get_unique_question_count(c.to_string());
    }
    println!("{:?}", sum);

    let mut sum = 0;
    let clean_group_data = clean_questions_with_group(in_data.clone());
    for c in clean_group_data.iter() {
        sum = sum + get_unique_group_question_count(c.0.to_string(), c.1);
    }
    println!("{:?}", sum);
}
