use std::env;
use std::fs::File;
use std::io::{BufRead, BufReader};
use std::collections::HashMap;

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

fn permutation_generator(length: usize, digits: Vec<char>) -> Vec<Vec<char>> {
    if length == 0 {
        return vec![vec![]];
    }
    
    let prev_perms = permutation_generator(length - 1, digits.clone());
    prev_perms.iter()
        .flat_map(|perm| {
            digits.iter().map(move |&d| {
                let mut new_perm = perm.clone();
                new_perm.push(d);
                new_perm
            })
        })
        .collect()
}


fn main() {
    let lines = read_lines();

    let mut sum = 0;
    for line in &lines {
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
    println!("part 1: {}", sum);


    let mut sum2 = 0;

    let mut permutations_cache: HashMap<usize, Vec<Vec<char>>> = HashMap::new();

    for line in &lines {
        let (target, numbers) = line.split_once(": ").unwrap();
        let target: i64 = target.parse().unwrap();
        let numbers: Vec<i64> = numbers.split(" ").map(|n| n.parse().unwrap()).collect();
        
        // Use a reference to the combinations
        let combinations = if !permutations_cache.contains_key(&(numbers.len() - 1)) {
            let mut new_combinations = permutation_generator(numbers.len() - 1, vec!['0', '1', '2']);
            // sort
            new_combinations.sort_by_key(|arr| arr.iter().map(|&c| c.to_digit(10).unwrap()).sum::<u32>());
            permutations_cache.insert(numbers.len() - 1, new_combinations);
            &permutations_cache[&(numbers.len() - 1)]
        } else {
            &permutations_cache[&(numbers.len() - 1)]
        };
         

         for ternary_digits in combinations {
            

            let mut curr_value = numbers[0];
            for j in 1..numbers.len() {
                let digit = ternary_digits[j-1];
                if digit == '1' {
                    curr_value *= numbers[j];
                } else if digit == '0' {
                    curr_value += numbers[j];
                } else if digit == '2' {
                    // turn cur_value to string
                    let s = curr_value.to_string();
                    // turn number to string
                    let n = numbers[j].to_string();
                    // put the strings together
                    let res = s + &n;
                    curr_value = res.parse::<i64>().unwrap();
                }

                if curr_value > target {
                    break;
                }
            }


            if curr_value == target {
                // sum all the numbers
                sum2 += target;
                break;
            }
        }
    }
    println!("part 2: {}", sum2);

}
