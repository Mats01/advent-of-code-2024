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




#[derive(Debug, Clone)]
struct Point {
    has_antinode: bool,
    antenna: Option<char>,
}

impl std::fmt::Display for Point {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self.antenna {
            Some(c) => write!(f, "{}", c),
            None => {
                if self.has_antinode {
                    write!(f, "#")
                } else {
                    write!(f, ".")
                }
            }
        }
    }
}

fn look_for_sibling(field: &mut Vec<Vec<Point>>, position: (usize, usize)) {
    let start_antenna = field[position.0][position.1].antenna;
    // dont worry about i32 because im only going into positive directions!!!!
    let cols = field[0].len();
    let rows = field.len();


    for i in position.0..cols {
        for j in 0..rows {
            if i == position.0 && j <= position.1 {
                continue;
            }
            let new_position = (i,j);

            
            // curr distance
            let distance_x: i32 = new_position.0 as i32 - position.0 as i32;
            // if more than half the field break
            if distance_x > (cols / 2) as i32 {
                continue;
            }

            let distance_y: i32 = new_position.1 as i32 - position.1 as i32;
            if distance_y > (rows / 2) as i32 {
                continue;
            }

            // print distance
            if field[new_position.0][new_position.1].antenna == start_antenna {
                
                
                // Calculate antinode positions, checking for negative values
                let antinode_above = (position.0 as i32 - distance_x, position.1 as i32 - distance_y);

                // check if its in bounds
                if antinode_above.0 < cols as i32 && antinode_above.1 < rows as i32 && antinode_above.0 >= 0 && antinode_above.1 >= 0 {
                    field[antinode_above.0 as usize][antinode_above.1 as usize].has_antinode = true;
                }


                let antinode_below = (new_position.0 as i32 + distance_x, new_position.1 as i32 + distance_y);
                if antinode_below.0 < cols as i32 && antinode_below.1 < rows as i32 && antinode_below.0 >= 0 && antinode_below.1 >= 0 {
                    field[antinode_below.0 as usize][antinode_below.1 as usize].has_antinode = true;
                }
        
            }
            
        }
    }
}

fn calculate_antinodes(field: &mut Vec<Vec<Point>>) {
    for i in 0..field.len() {
        for j in 0..field[i].len() {
            if field[i][j].antenna.is_some() {
                look_for_sibling(field, (i, j));
            }
        }
    }
}


fn main() {
    let lines = read_lines();

    let mut field = Vec::new();
    for line in lines {
        let mut row = Vec::new();
        for char in line.chars() {
            let mut new_point = Point { has_antinode: false, antenna: None };
            if char != '.' {
                new_point.antenna = Some(char);
            }
            row.push(new_point);
        }
        for point in &row {
            print!("{}", point);
        }
        println!();
        field.push(row);
    }


    calculate_antinodes(&mut field);

    println!();
    println!("Antinodes:");

    let mut antinode_count = 0;
    // print antinodes
    for row in &field {
        for point in row {
            print!("{}", point);
            if point.has_antinode {
                antinode_count += 1;
            }
        }
        println!();
    }


    println!("Antinode count: {}", antinode_count);

    
}
