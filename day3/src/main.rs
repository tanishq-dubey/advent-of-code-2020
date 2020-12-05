use std::fs::File;
use std::io::{BufRead, BufReader};

fn traverse(map: Vec<String>, sx: usize, sy: usize) -> usize {
    let mut cposx: usize = 0;
    let mut cposy: usize = 0;
    let mut num_trees: usize = 0;

    // While we are not at the bottom of the map
    while cposy < map.len() {
        let row: String = map.get(cposy).expect("CPOSY is not within vector bounds").to_string();
        if cposx > row.len() - 1 {
            // Wrap us back around to the beginning
            cposx = cposx - (row.len() - 1) - 1;
        }
        let pos_char: char = row.chars().nth(cposx).expect("CPOSX is not withing row bounds");
        if pos_char == '#' {
            num_trees = num_trees + 1;
        }
        cposx = cposx + sx;
        cposy = cposy + sy;
    }

    return num_trees;
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

    // Part 1
    println!("Hit {:?} trees", traverse(in_data.clone(), 3, 1));


    // Part 2
    println!("Hit {:?} trees", traverse(in_data.clone(), 1, 1) * traverse(in_data.clone(), 3, 1) * traverse(in_data.clone(), 5, 1) * traverse(in_data.clone(), 7, 1) * traverse(in_data.clone(), 1, 2));
}
