use std::collections::HashMap;
use std::env;
use std::fs;

fn extract_cubes(s: &str) -> Vec<Vec<HashMap<String, u32>>> {
    let mut games = Vec::new();

    for game in s.split("; ") {
        let mut subsets = Vec::new();
        for subset in game.split(", ") {
            let mut cubes = HashMap::new();
            let parts: Vec<&str> = subset.split_whitespace().collect();
            if let (Some(count), Some(color)) = (parts.get(0), parts.get(1)) {
                if let Ok(count) = count.parse::<u32>() {
                    cubes.insert(color.to_string(), count);
                }
            }
            subsets.push(cubes);
        }
        games.push(subsets);
    }

    games
}

fn is_possible_game(subsets: &[HashMap<String, u32>]) -> bool {
    let mut red_count = 0;
    let mut green_count = 0;
    let mut blue_count = 0;

    for cubes in subsets {
        red_count += cubes.get("red").unwrap_or(&0);
        green_count += cubes.get("green").unwrap_or(&0);
        blue_count += cubes.get("blue").unwrap_or(&0);
    }

    red_count <= 12 && green_count <= 13 && blue_count <= 14
}

fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() != 2 {
        eprintln!("Usage: {} <file_path>", args[0]);
        std::process::exit(1);
    }

    let file_path = &args[1];
    let contents = fs::read_to_string(file_path).expect("Error reading the file");

    let mut total_possible_game_ids = 0;

    for line in contents.lines() {
        let parts: Vec<&str> = line.split(": ").collect();
        if parts.len() == 2 {
            let cubes_info = parts[1];
            let games = extract_cubes(cubes_info);

            println!("Game: {}", parts[0]);
            let mut possible_game = true;
            for subsets in &games {
                if !is_possible_game(subsets) {
                    possible_game = false;
                    break;
                }
            }
            if possible_game {
                println!("Possible");
                if let Ok(id) = parts[0].split_whitespace().last().unwrap_or_default().parse::<u32>() {
                    total_possible_game_ids += id;
                }
            } else {
                println!("Not possible");
            }
        }
    }

    println!("Total value of possible game IDs: {}", total_possible_game_ids);
}
