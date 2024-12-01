use std::env;
use std::fs::File;
use std::io::{BufRead, BufReader};


fn calculate_total_distance(left_column: &mut Vec<i32>, right_column: &mut Vec<i32>) -> i32 {
    // sort the left column
    left_column.sort();

    // sort the right column
    right_column.sort();

    // are the columns of the same length?
    if left_column.len() != right_column.len() {
        eprintln!("Columns are not of the same length");
        std::process::exit(1);
    } else {
        println!("Columns are of the same length");
    }

    let mut total_distance = 0;
    for i in 0..left_column.len() {
        total_distance += (left_column[i] - right_column[i]).abs();
    }

    total_distance
}

fn nr_of_occurances(number: i32, column: &Vec<i32>) -> i32 {
    column.iter().filter(|&&x| x == number).count() as i32
}

fn calculate_occurances(left_column: &Vec<i32>, right_column: &Vec<i32>) -> Vec<i32> {
    let mut occurances: Vec<i32> = Vec::new();
    for i in 0..left_column.len() {
        occurances.push(nr_of_occurances(left_column[i], &right_column));
    }
    occurances
}

fn calculate_total_occurances(left_column: &Vec<i32>, occurances: &Vec<i32>) -> i32 {
    let mut total_occurances = 0;
    for i in 0..left_column.len() {
        total_occurances += left_column[i] * occurances[i];
    }
    total_occurances
}

fn main() {
    let args: Vec<String> = env::args().collect();
    
    if args.len() != 2 {
        eprintln!("Usage: {} <file_path>", args[0]);
        std::process::exit(1);
    }

    let file_path = &args[1];
    let file = File::open(file_path).expect("Failed to open file");
    let reader = BufReader::new(file);

    let mut left_column: Vec<i32> = Vec::new();
    let mut right_column: Vec<i32> = Vec::new();

    for line in reader.lines() {
        if let Ok(line) = line {
            // parse the line, split by 4 spaces
            let parts: Vec<&str> = line.split_whitespace().collect();
            left_column.push(parts[0].parse().expect("Failed to parse left column"));
            right_column.push(parts[1].parse().expect("Failed to parse right column"));
        }
    }
    

    let total_distance = calculate_total_distance(&mut left_column, &mut right_column);
    println!("total_distance: {}", total_distance);

    
    let occurances = calculate_occurances(&left_column, &right_column);
    let total_occurances = calculate_total_occurances(&left_column, &occurances);
    println!("total_occurances: {}", total_occurances);


}

