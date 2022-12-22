use std::error::Error;
use std::fs;

fn final_position(input: &Vec<&str>) -> (usize, usize) {
    let (mut i, mut horizontal, mut depth) = (0,0,0);
    while i != input.len() {
        let command: Vec<&str> = input[i].split_ascii_whitespace().collect();
        // println!("{}: {}", i, input[i]);
        let op = command[0];
        let value = command[1].parse::<usize>().expect("Could not parse value");
        match op {
            "forward" => horizontal += value,
            "down" => depth += value,
            "up" => depth -= value,
            _ => println!("Unknown operation found: {}", op),
        }
        i += 1;
    }
    (horizontal, depth)
}

fn final_position_with_aim(input: &Vec<&str>) -> (usize, usize) {
    let (mut i, mut horizontal, mut depth, mut aim) = (0,0,0,0);
    while i != input.len() {
        let command: Vec<&str> = input[i].split_ascii_whitespace().collect();
        let op = command[0];
        let value = command[1].parse::<usize>().expect("Could not parse value");
        match op {
            "forward" => { horizontal += value; depth += aim * value; },
            "down" => aim += value,
            "up" => aim -= value,
            _ => println!("Unknown operation found: {}", op),
        }
        i += 1;
    }
    (horizontal, depth)
}

fn main() -> Result<(), Box<dyn Error>> {
    let contents: String = fs::read_to_string("input.txt")?;
    let input: Vec<&str> = contents
        .trim()
        .split("\n")
        .collect();

    let (horizontal, depth) = final_position(&input);
    println!("Horizontal:{}, Depth:{}", horizontal, depth);
    println!("Answer part 1 {}", horizontal * depth);

    let (horizontal, depth) = final_position_with_aim(&input);
    println!("Horizontal:{}, Depth:{}", horizontal, depth);
    println!("Answer part 2 {}", horizontal * depth);
    Ok(())
}
