use std::env;
use std::fs::File;
use std::io::{BufRead, BufReader};

fn read_lines() -> Vec<String> {
    let args: Vec<String> = env::args().collect();

    if args.len() != 2 {
        eprintln!("Usage: {} <file_path>", args[0]);
        std::process::exit(1);
    }

    let file_path = &args[1];
    let mut lines = Vec::new();

    let file = File::open(file_path).expect("Failed to open file");
    let reader = BufReader::new(file);
    for line in reader.lines() {
        if let Ok(line) = line {
            lines.push(line);
        }
    }
    lines
}


fn main() {
    let lines = read_lines();

    let mut sum = 0;
    for line in lines {
        let (target, numbers) = line.split_once(": ").unwrap();
        let target: i64 = target.parse().unwrap();
        let numbers: Vec<i64> = numbers.split(" ").map(|n| n.parse().unwrap()).collect();

        

        for i in 0..(2_i64.pow((numbers.len() - 1) as u32)) {

            let mut binary_digits: Vec<char> = format!("{:b}", i).chars().collect();
            // pad with leading zeros
            while binary_digits.len() < (numbers.len() - 1) {
                binary_digits.insert(0, '0');
            }

            let mut curr_value = numbers[0];
            for j in 1..numbers.len() {
                let digit = binary_digits[j-1];
                if digit == '1' {
                    curr_value *= numbers[j];
                } else {
                    curr_value += numbers[j];
                }
                if curr_value > target {
                    break;
                }
            }

            
            if curr_value == target {
                // sum all the numbers
                sum += target;
                break;
            }
        }

        
    }
    println!("{}", sum);

}
