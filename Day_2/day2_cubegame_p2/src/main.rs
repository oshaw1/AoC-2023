use std::collections::HashMap;
use std::env;
use std::fs;

fn extract_cubes(s: &str) -> Vec<HashMap<String, u32>> {
    let mut subsets = Vec::new();

    for subset in s.split("; ") {
        let mut cubes = HashMap::new();
        for part in subset.split(", ") {
            let parts: Vec<&str> = part.split_whitespace().collect();
            if let (Some(count), Some(color)) = (parts.get(0), parts.get(1)) {
                if let Ok(count) = count.parse::<u32>() {
                    cubes.insert(color.to_string(), count);
                }
            }
        }
        subsets.push(cubes);
    }

    subsets
}

fn calculate_game_power(subsets: &[HashMap<String, u32>]) -> u32 {
    let mut max_red = 0;
    let mut max_green = 0;
    let mut max_blue = 0;

    for cubes in subsets {
        if let Some(red) = cubes.get("red") {
            max_red = max_red.max(*red);
        }
        if let Some(green) = cubes.get("green") {
            max_green = max_green.max(*green);
        }
        if let Some(blue) = cubes.get("blue") {
            max_blue = max_blue.max(*blue);
        }
    }

    max_red * max_green * max_blue
}

fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() != 2 {
        eprintln!("Usage: {} <file_path>", args[0]);
        std::process::exit(1);
    }

    let file_path = &args[1];
    let contents = fs::read_to_string(file_path).expect("Error reading the file");

    let mut total_game_power = 0;

    for line in contents.lines() {
        let parts: Vec<&str> = line.split(": ").collect();
        if parts.len() == 2 {
            let cubes_info = parts[1];
            let subsets = extract_cubes(cubes_info);

            println!("Game: {}", parts[0]);
            let game_power = calculate_game_power(&subsets);
            println!("Game Power: {}", game_power);
            total_game_power += game_power;
        }
    }

    println!("Total value of game power: {}", total_game_power);
}
