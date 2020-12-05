use std::fs::File;
use std::io::{BufRead, BufReader};

fn validate_passport(passport: String) -> bool {
    let passport_split: Vec<&str> = passport.split_whitespace().collect::<Vec<&str>>();
    if passport_split.len() > 8 {
        return false;
    }


    if passport_split.len() == 8 {
        return true;
    }

    if passport_split.len() == 7 {
        for field in passport_split.iter() {
            print!("{:?}", field[0..3].to_string());
            if field[0..3].to_string() == "cid" {
                println!();
                return false;
            }
        }
        println!();
        return true;
    }

    return false;
}

// Take the raw passports and return one passport per line
fn clean_passports(raw: Vec<String>) -> Vec<String> {
    let mut clean_line: String = "".to_string();
    let mut ret_vec: Vec<String> = Vec::new();

    for line in raw.iter() {
        if line == "" {
            ret_vec.push(clean_line.trim().to_string());
            clean_line = "".to_string();
        } else {
            clean_line = clean_line + " " + line;
        }
    }

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

    let cleaned_passports: Vec<String> = clean_passports(in_data.clone());

    let mut valid_passports = 0;
    for passport in cleaned_passports.iter() {
        if validate_passport(passport.to_string()) {
            valid_passports = valid_passports + 1;
        }
    }

    println!("Found {:?} valid passports", valid_passports);
}
