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
        lines.push(line.expect("Failed to read line"));
    }
    lines
}


fn check_direction(grid: &Vec<Vec<String>>, start: [i32; 2], direction: [i32; 2]) -> bool {
    // we are assuming we are starting form X

    let need_to_find = vec!["X", "M", "A", "S"];
    // check if we are out of bounds
    if start[0] + (direction[0] * 3) >= (grid.len() as i32)     
        || start[1] + (direction[1] * 3) >= (grid[0].len() as i32) 
        || start[0] + (direction[0] * 3) < 0 
        || start[1] + (direction[1] * 3) < 0 {
        return false;
    }

    for i in 1..4 {
        if grid[(start[0] + (direction[0] * i)) as usize][(start[1] + (direction[1] * i)) as usize] != need_to_find[i as usize] {
            return false;
        }
    }
    

    return true;


}

fn check_right(grid: &Vec<Vec<String>>, start: [i32; 2]) -> bool {
    check_direction(grid, start, [0, 1])
}

fn check_left(grid: &Vec<Vec<String>>, start: [i32; 2]) -> bool {
    check_direction(grid, start, [0, -1])
}

fn check_down(grid: &Vec<Vec<String>>, start: [i32; 2]) -> bool {
    check_direction(grid, start, [1, 0])
}

fn check_up(grid: &Vec<Vec<String>>, start: [i32; 2]) -> bool {
    check_direction(grid, start, [-1, 0])
}

fn check_down_right(grid: &Vec<Vec<String>>, start: [i32; 2]) -> bool {
    check_direction(grid, start, [1, 1])
}

fn check_up_right(grid: &Vec<Vec<String>>, start: [i32; 2]) -> bool {
    check_direction(grid, start, [-1, 1])
}

fn check_up_left(grid: &Vec<Vec<String>>, start: [i32; 2]) -> bool {
    check_direction(grid, start, [-1, -1])
}


fn check_down_left(grid: &Vec<Vec<String>>, start: [i32; 2]) -> bool {
    check_direction(grid, start, [1, -1])
}

fn main() {
    let lines = read_lines();


    let mut grid: Vec<Vec<String>> = Vec::new();
    for line in lines {
        grid.push(line.chars().map(|c| c.to_string()).collect());
    }



    println!("grid: {:?}", grid);
    let mut total_finds = 0;

    for i in 0..grid.len() as i32{
        for j in 0..grid[0].len() as i32{
            if grid[i as usize][j as usize] == "X" {
                if check_right(&grid, [i, j]) {
                    println!("Found at {:?}", [i, j]);
                    total_finds += 1;
                }
                if check_left(&grid, [i, j]) {
                    println!("Found at {:?}", [i, j]);
                    total_finds += 1;
                }
                if check_up(&grid, [i, j]) {
                    println!("Found at {:?}", [i, j]);
                    total_finds += 1;
                }
                if check_down(&grid, [i, j]) {
                    println!("Found at {:?}", [i, j]);
                    total_finds += 1;
                }
                if check_down_right(&grid, [i, j]) {
                    println!("Found at {:?}", [i, j]);
                    total_finds += 1;
                }
                if check_up_right(&grid, [i, j]) {
                    println!("Found at {:?}", [i, j]);
                    total_finds += 1;
                }
                if check_up_left(&grid, [i, j]) {
                    println!("Found at {:?}", [i, j]);
                    total_finds += 1;
                }
                if check_down_left(&grid, [i, j]) {
                    println!("Found at {:?}", [i, j]);
                    total_finds += 1;
                }
            }
        }
    }

    println!("Total finds: {}", total_finds);



}

