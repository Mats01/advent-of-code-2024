use std::env;
use std::fs::File;
use std::io::{BufRead, BufReader};
use std::collections::HashMap;


fn read_lines() -> (Vec<String>, Vec<String>) {
    let args: Vec<String> = env::args().collect();

    if args.len() != 2 {
        eprintln!("Usage: {} <file_path>", args[0]);
        std::process::exit(1);
    }
    let file_path = &args[1];
    let mut lines_top = Vec::new();
    let mut lines_bottom = Vec::new();
    let file = File::open(file_path).expect("Failed to open file");
    let reader = BufReader::new(file);
    let mut found_empty = false;
    for line in reader.lines() {
        if let Ok(line) = line {
            if line.is_empty() {
                found_empty = true;
            } else if !found_empty {
                lines_top.push(line);
            } else {
                lines_bottom.push(line);
            }
        }
    }
    (lines_top, lines_bottom)
}

fn custom_sort(line: &Vec<i32>, cant_be_after: &HashMap<i32, Vec<i32>>) -> Vec<i32> {
    let mut fixed_line = line.clone();
    fixed_line.sort_by(|a, b| {
        if cant_be_after.contains_key(&a) {
            if cant_be_after[&a].contains(&b) {
                std::cmp::Ordering::Less
            } else {
                std::cmp::Ordering::Equal
            }
        } else {
            std::cmp::Ordering::Equal
        }
    });
    fixed_line
}

fn main() {
    let (lines_top, lines_bottom) = read_lines();

    let mut cant_be_after: HashMap<i32, Vec<i32>> = HashMap::new();

    for line in lines_top {
        let numbers = line.split("|").map(|s| s.parse().unwrap()).collect::<Vec<i32>>();
        cant_be_after.entry(numbers[0]).or_insert(Vec::new()).push(numbers[1]);
    }


    let mut good_lines: Vec<Vec<i32>> = Vec::new();
    let mut fixed_lines: Vec<Vec<i32>> = Vec::new();
    for line in lines_bottom {
        let numbers = line.split(",").map(|s| s.parse().unwrap()).collect::<Vec<i32>>();
        let sorted_line = custom_sort(&numbers, &cant_be_after);
        if sorted_line == numbers {
            good_lines.push(numbers);
        } else {
            fixed_lines.push(sorted_line);
        }
    }
    
    let mut sum_of_middles = 0;
    for line in good_lines {
        sum_of_middles += line[line.len() / 2];
    }
    println!("part 1: {}", sum_of_middles);

    sum_of_middles = 0;
    for line in fixed_lines {
        sum_of_middles += line[line.len() / 2];
    }
    println!("part 2: {}", sum_of_middles);

}
