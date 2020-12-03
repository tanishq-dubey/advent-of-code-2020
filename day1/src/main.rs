use std::fs::File;
use std::io::{BufRead, BufReader};
use std::collections::HashMap;

fn sum(data: Vec<String>, target: i32) -> Option<(i32, i32)> {
    let mut items: HashMap<i32, i32> = HashMap::new();
    for snum in data.iter() {
        let num: i32 = snum.parse::<i32>().unwrap();
        if items.contains_key(&(target - num)) {
            return Some((num, target - num))
        }
        items.insert(num, 0);
    }
    return None
}

fn tsum(data: Vec<String>, target: i32) -> Option<(i32, i32, i32)> {
    for (i, snum) in data.iter().enumerate() {
        let sub = data[i + 1..data.len()].to_vec();
        let num: i32 = snum.parse::<i32>().unwrap();
        match sum(sub, target - num) {
            None => continue,
            Some(res) => {
                return Some((num, res.0, res.1));
            }
        };
    }
    return None
}

fn main() {
    let file = File::open("./input");
    let file = match file {
        Ok(f) => f,
        Err(err) => {
            panic!("Could not open file: {:?}", err);
        }
    };

    let reader = BufReader::new(file);
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

    match sum(in_data.clone(), 2020) {
        None => panic!("There are no 2 numbers that add to 2020"),
        Some(res) => {
            println!("{}", res.0 * res.1);
        }
    };

    match tsum(in_data.clone(), 2020) {
        None => panic!("There are no 3 numbers that add to 2020"),
        Some(res) => {
            println!("{}", res.0 * res.1 * res.2);
        }
    };
}
