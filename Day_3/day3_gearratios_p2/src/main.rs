use std::env;
use std::fs;

#[derive(Clone, Debug)]
struct Coordinates {
    x: usize,
    y: usize,
}

#[derive(Clone, Debug)]
struct Number {
    str_contents: String,
    length: usize,
    coordinates: Coordinates,
    symbols: Vec<char>,
}

fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() != 2 {
        println!("Usage: {} <input_file>", args[0]);
        return;
    }

    let filename = &args[1];
    let input = match fs::read_to_string(filename) {
        Ok(content) => content,
        Err(_) => {
            println!("Error reading the file");
            return;
        }
    };

    let output = part2(&input);
    println!("{}", output);
}

fn part2(input: &str) -> i64 {
    let lines: Vec<&str> = input.lines().collect();
    let mut gears: Vec<i64> = Vec::new();

    let directions = vec![(1, 0), (0, 1), (-1, 0), (0, -1), (1, 1), (-1, 1), (1, -1), (-1, -1)];

    for y in 0..lines.len() {
        let line = lines[y];
        for x in 0..line.len() {
            if line.chars().nth(x).unwrap() != '*' {
                continue; // Skip characters that are not '*'
            }

            let mut adjacent_numbers = Vec::new();

            for (dx, dy) in &directions {
                let mut nx = x as i32 + dx;
                let mut ny = y as i32 + dy;

                let mut num = String::new();

                while nx >= 0
                    && ny >= 0
                    && ny < lines.len() as i32
                    && nx < lines[ny as usize].len() as i32
                    && lines[ny as usize].chars().nth(nx as usize).unwrap().is_ascii_digit()
                {
                    num.push(lines[ny as usize].chars().nth(nx as usize).unwrap());
                    nx += dx;
                    ny += dy;
                }

                if !num.is_empty() {
                    adjacent_numbers.push(num.parse::<i64>().unwrap());
                }
            }

            if adjacent_numbers.len() == 2 {
                let gear_ratio = adjacent_numbers[0] * adjacent_numbers[1];
                gears.push(gear_ratio);
            }
        }
    }

    let total_gear_ratio: i64 = gears.iter().sum();
    total_gear_ratio
}

