use std::env;
use std::fs::File;
use std::io::{BufRead, BufReader};
use regex::Regex;


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
        lines.push(line.expect("Failed to read line"));
    }
    lines
}



fn part_one(lines: &Vec<String>) {
    let regex = Regex::new("mul\\(([1-9]{1}[0-9]{0,2}),([1-9]{1}[0-9]{0,2})\\)").unwrap();

    let mut pairs: Vec<Vec<i32>> = Vec::new();

    for line in lines {

        let matches = regex.captures_iter(line);

        for capture in matches {
            if let (Ok(first), Ok(second)) = (capture[1].parse(), capture[2].parse()) {
                pairs.push(vec![first, second]);
            }
        }
    }        

    let sum = pairs.iter().map(|pair| pair[0] * pair[1]).sum::<i32>();

    println!("total (part 1): {}", sum);

}


fn part_two(lines: &Vec<String>) {
    let regex = Regex::new("mul\\(([1-9]{1}[0-9]{0,2}),([1-9]{1}[0-9]{0,2})\\)").unwrap();

    let mut enabled = true;
    let mut sum = 0;
    let mut in_multiplication = String::new();

    for line in lines {
        let chars: Vec<char> = line.chars().collect();
        let mut i = 0;
        while i < chars.len() {
            let curr_char = chars[i];



            if in_multiplication != "" {
                in_multiplication.push(curr_char);
                

                if curr_char == ')' {

                    // does the string in_multiplication exactly match the regex?
                    if regex.is_match(&in_multiplication) {
                        let captures = regex.captures(&in_multiplication).unwrap();
                        let first = captures[1].parse::<i32>().unwrap();
                        let second = captures[2].parse::<i32>().unwrap();
                        
                        if enabled {
                            sum += first * second;
                        }
                    }

                    in_multiplication = String::new();
                }

            }
            

            // if the next set of chars is exatcly "don't()"
            // get the next 7 chars
            let next_chars = chars.iter().skip(i).take(7).copied().collect::<Vec<char>>();
            if next_chars == ['d', 'o', 'n', '\'', 't', '(', ')'] {
                enabled = false;
                // skip the next 7 chars
                i += 7;
                continue;
            }
            // if the next set of chars is exatcly "do()"
            // get the next 4 chars
            let next_chars = chars.iter().skip(i).take(4).copied().collect::<Vec<char>>();
            if next_chars == ['d', 'o', '(', ')'] {
                enabled = true;
                // skip the next 4 chars
                i += 4;
                continue;
            }

            // if the next set of chars is exatcly "mul("
            // get the next 4 chars
            let next_chars = chars.iter().skip(i).take(4).copied().collect::<Vec<char>>();
            if next_chars == ['m', 'u', 'l', '('] {
                in_multiplication = "mul(".to_string();
                // skip the next 4 chars
                i += 4;
                continue;
            }



            i += 1;
        }
    }

    println!("total (part 2): {}", sum);
}


fn part_two_better(lines: &Vec<String>, enabled_disabled: bool) {
    let regex = Regex::new("mul\\(([1-9]{1}[0-9]{0,2}),([1-9]{1}[0-9]{0,2})\\)|don't\\(\\)|do\\(\\)").unwrap();

    let mut enabled = true;
    let mut sum = 0;

    for line in lines {
        let matches = regex.find_iter(line);

        for curr_match in matches {

            if curr_match.as_str() == "don't()" {
                enabled = false;
                continue;
            }
            if curr_match.as_str() == "do()" {
                enabled = true;
                continue;
            }

            // get group 1 and group 2
            let captures = regex.captures(curr_match.as_str()).unwrap();
            let first = captures[1].parse::<i32>().unwrap();
            let second = captures[2].parse::<i32>().unwrap();
            if enabled_disabled || enabled {
                sum += first * second;
            }
        }

    }

    if enabled_disabled {
        println!("total (part 1): {}", sum);
    } else {
        println!("total (part 2): {}", sum);
    }

}

fn main() {
    let lines = read_lines();

    part_two_better(&lines, true);
    part_two_better(&lines, false);

    
}

