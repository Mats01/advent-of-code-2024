use std::env;
use std::fs::File;
use std::io::{BufRead, BufReader};



fn main() {
    let args: Vec<String> = env::args().collect();
    
    if args.len() != 2 {
        eprintln!("Usage: {} <file_path>", args[0]);
        std::process::exit(1);
    }

    let file_path = &args[1];
    let file = File::open(file_path).expect("Failed to open file");
    let reader = BufReader::new(file);

    const MIN_DIFF: i32 = 1;
    const MAX_DIFF: i32 = 3;


    let mut total = 0;
    
    for line in reader.lines() {
        let mut row: Vec<i32> = Vec::new();
        if let Ok(line) = line {
            // parse the line, split by 4 spaces
            let parts: Vec<&str> = line.split_whitespace().collect();
            for part in parts {
                row.push(part.parse().expect("Failed to parse part"));
            }

            if row.len() <= 1 {
                continue;
            }

            

            'outer: for j in 0..row.len() {
                // remove the jth element
                let mut new_row = row.clone();
                new_row.remove(j);
                if new_row[1] < new_row[0] {
                    // reverse the row
                    new_row.reverse();
                }
                for i in 0..new_row.len()-1 {
                    if new_row[i+1] - new_row[i] < MIN_DIFF || new_row[i+1] - new_row[i] > MAX_DIFF {
                        continue 'outer;
                    }
                }
                total += 1;
                break;
            }


        }

    }
    
    println!("total: {}", total);




}

