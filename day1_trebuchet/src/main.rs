use std::env;
use std::fs::File;
use std::io::{self, BufRead};

fn extract_calibration(line: &str) -> Option<u32> {
    let first_digit = line.chars().find(|c| c.is_digit(10))?;
    let last_digit = line.chars().rev().find(|c| c.is_digit(10))?;
    
    if let (Some(first), Some(last)) = (first_digit.to_digit(10), last_digit.to_digit(10)) {
        Some(first * 10 + last)
    } else {
        None
    }
}

fn read_input(file_path: &str) -> io::Result<Vec<String>> {
    let file = File::open(file_path)?;
    let reader = io::BufReader::new(file);
    let mut lines = Vec::new();

    for line in reader.lines() {
        lines.push(line?);
    }

    Ok(lines)
}

fn main() -> io::Result<()> {
    let args: Vec<String> = env::args().collect();

    if args.len() != 2 {
        eprintln!("Usage: {} <file_path>", args[0]);
        std::process::exit(1);
    }

    let file_path = &args[1];

    let lines = read_input(file_path)?;

    let mut sum_of_calibration_values = 0;

    for line in &lines {
        if let Some(calibration) = extract_calibration(line) {
            println!("Calibration value for '{}': {}", line, calibration);
            sum_of_calibration_values += calibration;
        } else {
            println!("No calibration value found for '{}'", line);
        }
    }

    println!("Sum of calibration values: {}", sum_of_calibration_values);

    Ok(())
}
