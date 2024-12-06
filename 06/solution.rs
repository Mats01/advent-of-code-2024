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

#[derive(Debug, PartialEq, Clone)]
enum TileType {
    Obstacle,
    Empty,
}

#[derive(Debug, Clone)]
struct Tile {
    tile_type: TileType,
    visits: i32,
    last_visit_direction: Option<(i32, i32)>,
}

#[derive(Debug)]
struct Guard {
    position: (i32, i32),
    direction: (i32, i32),
}

#[derive(Debug, PartialEq)]
enum MoveResult {
    Moved,
    Turned,
    Exited,
    Stuck,
}

fn move_guard(guard: &mut Guard, tiles: &mut Vec<Vec<Tile>>) -> MoveResult {
    let new_position = (
        guard.position.0 + guard.direction.0,
        guard.position.1 + guard.direction.1,
    );

    // check if the new position would go out of bounds
    let rows = tiles.len() as i32;
    let cols = tiles[0].len() as i32;

    if new_position.0 >= cols || new_position.1 >= rows || new_position.0 < 0 || new_position.1 < 0
    {
        return MoveResult::Exited;
    }

    if tiles[new_position.1 as usize][new_position.0 as usize].tile_type == TileType::Obstacle {
        // turn 90 degs
        // 0,-1 -> 1,0
        // 1,0 -> 0,1
        // 0,1 -> -1,0
        // -1,0 -> 0,-1

        let directions = [(0, -1), (1, 0), (0, 1), (-1, 0)];

        guard.direction = directions[(directions
            .iter()
            .position(|&d| d == guard.direction)
            .unwrap()
            + 1)
            % 4];
        return MoveResult::Turned;
    }

    guard.position = new_position;

    let new_tile = &mut tiles[new_position.1 as usize][new_position.0 as usize];

    if new_tile.visits > 0 && new_tile.last_visit_direction == Some(guard.direction) {
        return MoveResult::Stuck;
    }

    new_tile.visits += 1;
    new_tile.last_visit_direction = Some(guard.direction);
    return MoveResult::Moved;
}

fn main() {
    let lines = read_lines();

    let mut initial_position = (0, 0);
    let initial_direction = (0, -1);

    let mut guard = Guard {
        position: (0, 0),
        direction: (0, -1),
    };

    let mut tiles: Vec<Vec<Tile>> = Vec::new();
    for (y, line) in lines.iter().enumerate() {
        let mut row: Vec<Tile> = Vec::new();
        for (x, c) in line.chars().enumerate() {
            let mut new_tile = Tile {
                tile_type: if c == '#' {
                    TileType::Obstacle
                } else {
                    TileType::Empty
                },
                visits: 0,
                last_visit_direction: None,
            };
            if c == '^' {
                guard.position = (x as i32, y as i32);
                initial_position = (x as i32, y as i32);
                new_tile.visits = 1;
                new_tile.last_visit_direction = Some(guard.direction);
            }
            row.push(new_tile);
        }
        tiles.push(row);
    }

    let mut copied_tiles = tiles.clone();

    loop {
        let result = move_guard(&mut guard, &mut copied_tiles);
        if result == MoveResult::Exited {
            break;
        }
    }

    let mut visited_tiles = 0;
    for row in &copied_tiles {
        for tile in row {
            if tile.visits > 0 {
                visited_tiles += 1;
            }
        }
    }

    println!("part 1: {}", visited_tiles);

    let mut total_loops = 0;
    for (y, row) in tiles.iter().enumerate() {
        for (x, tile) in row.iter().enumerate() {
            if tile.tile_type != TileType::Obstacle {
                copied_tiles = tiles.clone();
                copied_tiles[y][x].tile_type = TileType::Obstacle;

                // reset guard position
                guard.position = initial_position;
                guard.direction = initial_direction;
                loop {
                    let result = move_guard(&mut guard, &mut copied_tiles);
                    if result == MoveResult::Exited {
                        break;
                    }
                    if result == MoveResult::Stuck {
                        total_loops += 1;
                        break;
                    }
                }
            }
        }
    }

    println!("part 2: {}", total_loops);
}
