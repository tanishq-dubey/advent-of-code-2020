use std::fs::File;
use std::io::{BufRead, BufReader};
use std::collections::{HashMap, HashSet};

fn parse_line(line: String) -> Option<(String, Vec<(i32, String)>)> {
    let raw_split: Vec<String> = line.split("bags contain").map(|s| s.to_string().trim().to_string()).collect::<Vec<String>>();
    if !raw_split.get(1).unwrap().contains("no") {
        let raw_bags: Vec<String> =  raw_split.get(1).unwrap().split(",").map(|s| s.to_string().trim().to_string()).collect::<Vec<String>>();
        let clean_bags: Vec<String> = raw_bags.iter().map(|s| s.replace("bags.", "").replace("bags", "").replace("bag.", "").replace("bag", "").trim().to_string()).collect::<Vec<String>>();
        let mut ret_bags: Vec<(i32, String)> = Vec::new();
        for bag in clean_bags.iter() {
            let individual: Vec<String> = bag.splitn(2, " ").map(|s| s.to_string().trim().to_string()).collect::<Vec<String>>();
            ret_bags.push((individual.get(0).unwrap().parse::<i32>().unwrap(), individual.get(1).unwrap().to_string()));
        }
        return Some((raw_split.get(0).unwrap().to_string(), ret_bags));
    }

    return None;
}

fn create_map(in_data: Vec<String>) -> HashMap<String, Vec<(i32, String)>> {
    let mut bags: HashMap<String, Vec<(i32, String)>> = HashMap::new();

    for i in in_data.iter() {
        match parse_line(i.to_string()) {
            Some(x) => {
                for d in x.1.iter() {
                    if bags.contains_key(&d.1) {
                        let mut current_val = bags.get(&d.1).unwrap().clone();
                        current_val.push((d.0, x.0.clone()));
                        bags.insert(d.1.clone(), current_val);
                    } else {
                        let mut current_val: Vec<(i32, String)> = Vec::new();
                        current_val.push((d.0, x.0.clone()));
                        bags.insert(d.1.clone(), current_val);
                    }
                }
            },
            None => {}
        };
    }

    return bags;
}

fn find_bags(target: String, bags: HashMap<String, Vec<(i32, String)>>, visited: &mut HashSet<String>) {
    if bags.contains_key(&target) {
        let data = bags.get(&target).unwrap().clone();
        for d in data.iter() {
            visited.insert(d.1.clone());
            find_bags(d.1.clone(), bags.clone(), visited)
        }
    }
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

    let bags: HashMap<String, Vec<(i32, String)>> = create_map(in_data);
    let mut visited: HashSet<String> = HashSet::new();

    find_bags("shiny gold".to_string(), bags, &mut visited);
    println!("{:?}", visited.len());
}
