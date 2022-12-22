use std::error::Error;
use std::fs;

fn find_sliding_window_increases(window_size: usize, input: &Vec<usize>) -> usize {
    let mut i = 0;
    let mut previous_sliding_window_measurement = 0;
    while i < window_size {
        previous_sliding_window_measurement += input[i];
        i += 1;
    }

    let mut starting_index = 1;
    let mut counter = 0;
    while starting_index + window_size <= input.len() {
        let next_sliding_window_measurement = previous_sliding_window_measurement
            - input[starting_index - 1]
            + input[starting_index + window_size - 1];
        if previous_sliding_window_measurement < next_sliding_window_measurement {
            counter += 1;
        }
        previous_sliding_window_measurement = next_sliding_window_measurement;
        starting_index += 1;
    }
    counter
}

fn find_number_of_times_depth_increases(input: &Vec<usize>) -> usize {
    let mut prev = input[0];
    let mut i = 0;
    let mut counter = 0;
    loop {
        if i >= input.len() {
            break;
        }
        if prev < input[i] {
            counter += 1;
        }
        prev = input[i];
        i += 1;
    }
    counter
}

fn main() -> Result<(), Box<dyn Error>> {
    let contents = fs::read_to_string("input.txt")?;
    let input: Vec<usize> = contents
        .trim()
        .split_ascii_whitespace()
        .map(|x| x.parse::<usize>().expect("Could not parse item"))
        .collect();
    let mut answer = find_number_of_times_depth_increases(&input);
    println!("Answer part 1:\n{}", answer);
    answer = find_sliding_window_increases(3, &input);
    println!("Answer part 2:\n{}", answer);
    Ok(())
}
