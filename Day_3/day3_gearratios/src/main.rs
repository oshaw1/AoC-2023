use std::env;
use std::fs;

fn main() {
    // Collect command-line arguments
    let args: Vec<String> = env::args().collect();

    // Ensure proper arguments are provided
    if args.len() != 2 {
        eprintln!("Please provide the argument in the format: cargo run --release src/input/input_dayX.txt");
        return;
    }

    // Retrieve the file name from the arguments
    let file_name = &args[1];

    // Read the contents of the file into a string
    let input = match fs::read_to_string(file_name) {
        Ok(content) => content,
        Err(_) => {
            eprintln!("Error reading the file");
            return;
        }
    };

    // Define symbols to check for adjacent digits
    let symbols: Vec<char> = vec!['*', '+', '#', '$', '-', '=', '&', '%', '@'];

    // Split the input into lines
    let lines: Vec<&str> = input.trim().split('\n').collect();

    // Function to check if a character is a symbol
    fn is_symbol(c: char, symbols: &[char]) -> bool {
        symbols.contains(&c)
    }

    // Function to check if a character is a digit
    fn is_digit(c: char) -> bool {
        c.is_digit(10)
    }

    // Function to calculate sum of adjacent digits to symbols
    let mut sum = 0;
    for (y, line) in lines.iter().enumerate() {
        for (x, c) in line.chars().enumerate() {
            if is_symbol(c, &symbols) {
                for dx in -1..=1 {
                    for dy in -1..=1 {
                        let nx = x as i32 + dx;
                        let ny = y as i32 + dy;

                        if nx >= 0 && ny >= 0 && ny < lines.len() as i32 {
                            let row = &lines[ny as usize];
                            if let Some(adj_char) = row.chars().nth(nx as usize) {
                                if is_digit(adj_char) {
                                    sum += adj_char.to_digit(10).unwrap() as i32;
                                }
                            }
                        }
                    }
                }
            }
        }
    }

    println!("The sum of all part numbers adjacent to symbols is: {}", sum);
}
