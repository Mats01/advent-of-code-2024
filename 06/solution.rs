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




#[derive(Debug, PartialEq)]
enum TileType {
    Obstacle,
    Empty,
}

#[derive(Debug)]
struct Tile {
    tile_type: TileType,
    visits: i32,
}

#[derive(Debug)]
struct Guard {
    position: (i32, i32),
    direction: (i32, i32),
}




fn move_guard(guard: &mut Guard, tiles: &mut Vec<Vec<Tile>>) -> bool {
    let new_position = (guard.position.0 + guard.direction.0, guard.position.1 + guard.direction.1);



    // check if the new position would go out of bounds
    let rows = tiles.len() as i32;
    let cols = tiles[0].len() as i32;

    if new_position.0 >= cols || new_position.1 >= rows || new_position.0 < 0 || new_position.1 < 0 {
        return false;
    }

    if tiles[new_position.1 as usize][new_position.0 as usize].tile_type == TileType::Obstacle {
        // turn 90 degs
        // 0,-1 -> 1,0
        // 1,0 -> 0,1
        // 0,1 -> -1,0
        // -1,0 -> 0,-1

        let directions = [(0, -1), (1, 0), (0, 1), (-1, 0)];

        guard.direction = directions[(directions.iter().position(|&d| d == guard.direction).unwrap() + 1) % 4];
        return true;
    }

    guard.position = new_position;
    tiles[new_position.1 as usize][new_position.0 as usize].visits += 1;
    return true;
}



fn main() {
    let lines = read_lines();


   

    let mut guard = Guard { position: (0, 0), direction: (0, -1) };
 
    let mut tiles: Vec<Vec<Tile>> = Vec::new();
    for (y, line) in lines.iter().enumerate() {
        let mut row: Vec<Tile> = Vec::new();
        for (x, c) in line.chars().enumerate() {
            let mut new_tile = Tile { tile_type: if c == '#' { TileType::Obstacle } else { TileType::Empty }, visits: 0 };
            if c == '^' {
                guard.position = (x as i32, y as i32);
                new_tile.visits = 1;
            }
            row.push(new_tile);
        }
        tiles.push(row);
    }

    while move_guard(&mut guard, &mut tiles) {}

    let mut visited_tiles = 0;
    for row in &tiles {
        for tile in row {
            if tile.visits > 0 {
                visited_tiles += 1;
            }
        }
    }

    println!("Visited tiles: {}", visited_tiles);

}
