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

    let output = part1(&input);
    println!("{}", output);
}

fn part1(input: &str) -> i32 {
    let mut total_sum = 0;
    let lines = input.lines();

    println!("{}",input);
    println!("");
    
    let mut nums :Vec<Number> = Vec::new();
    let mut newnum = Number{str_contents: "".to_owned(), length: 0, coordinates: Coordinates { x: 0, y: 0 }, symbols: Vec::new()};
    let mut appending_num = false;

    let line_len = lines.clone().nth(0).expect("Should have at least 1 line").len();

    for (y, line) in lines.clone().enumerate(){
        for (x, c) in line.chars().enumerate(){            
            // println!("{}, {} => {}", x, y, c);
            if !c.is_digit(10) && !appending_num {
                continue;
            }
            if c.is_digit(10) {
                if !appending_num {
                    // println!("Found start of number {}", c);
                    newnum = Number { str_contents: c.to_string(), length: 1, coordinates: Coordinates { x: x, y: y }, symbols: Vec::new()};
                    appending_num = true;
                }else{
                    newnum.str_contents += &c.to_string();
                    newnum.length += 1;
                }
            }

            if appending_num && (!c.is_digit(10) || x == line_len - 1){
                let min_x = if newnum.coordinates.x == 0 { 0 } else {newnum.coordinates.x - 1};
                let max_x = if newnum.coordinates.x + newnum.length < line_len { newnum.coordinates.x + newnum.length } else { line_len - 1 };

                let min_y = if newnum.coordinates.y > 0 { y - 1 } else { y };
                let max_y = if newnum.coordinates.y < line_len - 1 { y + 1} else { y };
            
                for check_y in min_y..=max_y {
                    for check_char in lines.clone().nth(check_y).expect("Should be within range")[min_x..=max_x].chars(){
                        if !check_char.is_digit(10) && check_char != '.' {
                            newnum.symbols.push(check_char);
                        }
                        // print!("{}", check_char);
                    }
                    // println!("");
                }
                if newnum.symbols.len() == 0 {
                    for check_y in min_y..=max_y {
                        print!("{}: ", check_y);
                        for check_char in lines.clone().nth(check_y).expect("Should be within range")[min_x..=max_x].chars(){
                            print!("{}",check_char);
                        }
                        println!("");
                    }
                    println!("Number completed: {}, Length: {}, Coordinates: {:?}, Symbols: {}", newnum.str_contents, newnum.length, newnum.coordinates, newnum.symbols.len());
                }


                appending_num = false;
                nums.push(newnum.clone());
            }
        }
    }

    for num in nums {
        if num.symbols.len() > 0 {
            total_sum += num.str_contents.parse::<i32>().expect("Should be a string repr. of a number");
        } else {
            // println!("{:?}", num);

        }
    }
    total_sum
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part1() {
        let input = "467..114..\n...*......\n..35...633\n......#...\n617*......\n.....+.58.\n..592.....\n......755.\n...$.*....\n.664.598..";
        let result = part1(input);
        assert_eq!(result, 4361);
    }

    #[test]
    fn test_part1_full_input() {
        let input = include_str!("./input3.txt");
        let result = part1(input);
        assert_eq!(result, 549908);
    }
}