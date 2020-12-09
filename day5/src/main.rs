use std::fs::File;
use std::io::{BufRead, BufReader};

const MAX_ROWS: i32 = 127;
const MAX_COLS: i32 = 7;

fn get_seat_pos(seat_id: String) -> (i32, i32) {
    let mut curr_r: i32 = MAX_ROWS;
    let mut curr_rm: i32 = 0;
    let mut curr_c: i32 = MAX_COLS;
    let mut curr_cm: i32 = 0;

    for c in seat_id.chars() {
        match c {
            'F' => {
                curr_r = curr_r - ((curr_r - curr_rm) / 2) - 1;
                println!("F\t\t vals are {} {} {} {}", curr_r, curr_rm, curr_c, curr_cm);
            },
            'B' => {
                curr_rm = curr_rm + ((curr_r - curr_rm) / 2) + 1;
                println!("B\t\t vals are {} {} {} {}", curr_r, curr_rm, curr_c, curr_cm);
            },
            'R' => {
                curr_cm = curr_cm + ((curr_c - curr_cm) / 2) + 1;
                println!("R\t\t vals are {} {} {} {}", curr_r, curr_rm, curr_c, curr_cm);
            },
            'L' => {
                curr_c = curr_c - ((curr_c - curr_cm) / 2) - 1;
                println!("L\t\t vals are {} {} {} {}", curr_r, curr_rm, curr_c, curr_cm);
            },
            _ => {
                panic!("Invalid input handed")
            }
        }
    }
    return (curr_rm, curr_cm)
}

fn get_seat_id(row: i32, col: i32) -> i32 {
    return row * 8 + col;
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

    let mut max_id = 0;

    for id in in_data.iter() {
        let pos = get_seat_pos(id.to_string());
        let s_id = get_seat_id(pos.0, pos.1);
        if s_id > max_id {
            max_id = s_id;
        }
    }

    println!("Max ID is {:?}", max_id);

    // Test Cases
    let t1_pos = get_seat_pos("FBFBBFFRLR".to_string());
    println!("T1 Pos : {} {}", t1_pos.0, t1_pos.1);
    let t1_id = get_seat_id(t1_pos.0, t1_pos.1);
    println!("T1 id : {}", t1_id);
    println!("=================");
    let t2_pos = get_seat_pos("FFFBBBFRRR".to_string());
    println!("T2 Pos : {} {}", t2_pos.0, t2_pos.1);
    let t2_id = get_seat_id(t2_pos.0, t2_pos.1);
    println!("T2 id : {}", t2_id);
    println!("=================");
    let t3_pos = get_seat_pos("BBFFBBFRLL".to_string());
    println!("T3 Pos : {} {}", t3_pos.0, t3_pos.1);
    let t3_id = get_seat_id(t3_pos.0, t3_pos.1);
    println!("T3 id : {}", t3_id);

    let mut seats: [[u32; (MAX_COLS + 1) as usize]; MAX_ROWS as usize] = [[0; (MAX_COLS + 1) as usize]; MAX_ROWS as usize];
    for id in in_data.iter() {
        let pos = get_seat_pos(id.to_string());
        seats[pos.0 as usize][pos.1 as usize] = 1;
    }
    for (i, s) in seats.iter().enumerate() {
        println!("{:?}\t {:?}", i, s);
    }
}
