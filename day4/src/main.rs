use std::fs::File;
use std::io::{BufRead, BufReader};
use std::collections::HashMap;

fn true_validate_passport(passport: String) -> bool {
    let passport_split: Vec<&str> = passport.split_whitespace().collect::<Vec<&str>>();
    if passport_split.len() > 8 || passport_split.len() < 7 {
        return false;
    }
    
    let mut fields: HashMap<String, String> = HashMap::new();

    for field in passport_split {
        let raw_field = field.split(':').collect::<Vec<&str>>();
        fields.insert(raw_field.get(0).expect("Didn't find 0th field").to_string(), raw_field.get(1).expect("Didn't find 1st field").to_string());
    }

    // Birth Year
    let byr_val_get: String = match fields.get("byr") {
        Some(x) => x.to_string(),
        None => "".to_string()
    };
    if byr_val_get.len() == 0 {
        return false
    }

    let byr_val = match byr_val_get.parse::<i32>() {
        Ok(year) => year,
        Err(_) => 0,
    };
    if !(byr_val > 1919 && byr_val < 2003) {
        return false
    }

    // Issue Year
    let iyr_val_get  = match fields.get("iyr") {
        Some(x) => x.to_string(),
        None => "".to_string()
    };
    if iyr_val_get.len() == 0 {
        return false;
    }

    let iyr_val: i32 = match iyr_val_get.parse::<i32>() {
        Ok(year) => year,
        Err(_) => 0,
    };
    if !(iyr_val > 2009 && iyr_val < 2021) {
        return false
    }

    // Expiration Year
    let eyr_val_get = match fields.get("eyr") {
        Some(x) => x.to_string(),
        None => "".to_string(),
    };
    if eyr_val_get.len() == 0 {
        return false;
    }

    let eyr_val = match eyr_val_get.parse::<i32>() {
        Ok(year) => year,
        Err(_) => 0,
    };
    if !(eyr_val > 2019 && eyr_val < 2031) {
        return false
    }

    // Height
    let hgt_val_get = match fields.get("hgt") {
        Some(x) => x.to_string(),
        None => "".to_string()
    };
    if hgt_val_get.len() == 0 {
        return false;
    }

    let hgt_val_raw: Vec<char> = hgt_val_get.to_string().chars().collect();
    let hgt_units: String = hgt_val_raw.iter().rev().take(2).collect::<Vec<&char>>().into_iter().rev().collect();
    if hgt_units != "cm" && hgt_units != "in" {
        return false
    }

    let hgt_number_raw: Vec<&char> = hgt_val_raw.iter().take(hgt_val_raw.len() - 2).collect();
    let hgt_number = match hgt_number_raw.clone().into_iter().collect::<String>().parse::<i32>() {
        Ok(x) => x,
        Err(_) => 0
    };
    if hgt_units == "cm" {
        if !(hgt_number >= 150 && hgt_number <= 193) {
            return false;
        }
    } else if hgt_units == "in" {
        if !(hgt_number >= 59 && hgt_number <= 76) {
            return false;
        }
    }

    // Hair Color
    let hcl_val_get = match fields.get("hcl") {
        Some(x) => x.to_string(),
        None => "".to_string()
    };
    if hcl_val_get.len() == 0 {
        return false
    }
    let mut hcl_val_raw = hcl_val_get.chars();

    if hcl_val_raw.next().expect("Could not get first character") != '#' {
        return false;
    }
    for c in hcl_val_raw {
        if !((c >= std::char::from_u32(48).unwrap() && c <= std::char::from_u32(57).unwrap()) || (c >= std::char::from_u32(97).unwrap() && c <= std::char::from_u32(102).unwrap())) {
            return false;
        }
    }

    // Eye Color
    let ecl_val = match fields.get("ecl") {
        Some(x) => x.to_string(),
        None => "".to_string()
    };
    let eye_valid = ["amb".to_string(), "blu".to_string(), "brn".to_string(), "gry".to_string(), "grn".to_string(), "hzl".to_string(), "oth".to_string()];
    if !eye_valid.contains(&ecl_val) {
        return false
    }

    // Password ID
    let pid_val = match fields.get("pid") {
        Some(x) => x.to_string(),
        None => "".to_string()
    };
    if pid_val.len() != 9 {
        return false;
    }
    let pid_numeric: Vec<bool> = pid_val.chars().map(|c|c.is_numeric()).collect();
    if pid_numeric.contains(&false) {
        return false;
    }

    return true;
}

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
            if field[0..3].to_string() == "cid" {
                return false;
            }
        }
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

    let cleaned_passports: Vec<String> = clean_passports(in_data.clone());

    let mut valid_passports = 0;
    for passport in cleaned_passports.iter() {
        if validate_passport(passport.to_string()) {
            valid_passports = valid_passports + 1;
        }
    }

    println!("Found {:?} valid passports", valid_passports);

    let mut valid_passports = 0;
    for passport in cleaned_passports.iter() {
        if true_validate_passport(passport.to_string()) {
            valid_passports = valid_passports + 1;
        }
    }

    println!("Found {:?} true valid passports", valid_passports);
}
