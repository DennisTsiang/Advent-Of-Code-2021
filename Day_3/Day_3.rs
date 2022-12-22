use std::error::Error;
use std::fs;

struct BitCounter {
    zero: usize,
    one: usize,
}

fn count_bits(input: &Vec<usize>, binary_length: usize) -> Vec<BitCounter> {
    let mut bit_counters: Vec<BitCounter> = Vec::new();
    let mut i = 0;

    while i != input.len() {
        let mut j = 0;
        while j != binary_length {
            let modified_input = input[i] >> j;
            let least_significant_bit = modified_input & 0x1;
            
            if j >= bit_counters.len() {
                let mut bit_counter = BitCounter{ zero: 0, one: 0 };
                if least_significant_bit == 0 {
                    bit_counter.zero = 1;
                } else {
                    bit_counter.one = 1;
                }
                bit_counters.push(bit_counter);
            } else {
                if least_significant_bit == 0 {
                    bit_counters[j].zero += 1;
                } else {
                    bit_counters[j].one += 1;
                }
                
            }
            j += 1;
        }
        i += 1;
    }
    bit_counters
}

fn find_gamma_epsilon(bit_counters: Vec<BitCounter>) -> (usize, usize) {
    let mut gamma = 0;
    let mut epsilon = 0;
    let mut i = 0;

    while i != bit_counters.len() {
        if bit_counters[i].zero > bit_counters[i].one {
            gamma |= 0 << i;
            epsilon |= 1 << i;
        } else {
            gamma |= 1 << i;
            epsilon |= 0 << i;
        }
        i += 1;
        // println!("epsilon {}", epsilon);
    }
    (gamma, epsilon)
}

fn part_one(input: &Vec<usize>, binary_length: usize) -> usize {
    let bit_counters: Vec<BitCounter> = count_bits(input, binary_length);
    let (gamma, epsilon) = find_gamma_epsilon(bit_counters);
    println!("Gamma {} Epsilon {}", gamma, epsilon);
    gamma * epsilon
}

type RatingFunc = fn(usize, usize, &mut Vec<usize>, usize); 
fn part_two_helper(input: &Vec<usize>, binary_length: usize, rating_function: RatingFunc) -> usize {
    let mut input_copy = input.to_vec();
    let mut j = 0;
    while input_copy.len() > 1 && j != binary_length {
        let (mut i, mut zeros, mut ones) = (0, 0, 0);
        while i != input_copy.len() {
            let bit = input_copy[i] & (0x1 << (binary_length - 1 - j));
            if bit == 0 {
                zeros += 1;
            } else {
                ones += 1;
            }
            i += 1;
        }
        rating_function(zeros, ones, &mut input_copy, binary_length - 1 - j);
        j += 1;
    }
    input_copy[0]
}

fn find_co2_rating(zeros: usize, ones: usize, input: &mut Vec<usize>, shift: usize) {
    if zeros <= ones {
        input.retain(|x| x & (0x1 << shift) == 0);
    } else {
        input.retain(|x| x & (0x1 << shift) != 0);
    }
}

fn find_oxygen_rating(zeros: usize, ones: usize, input: &mut Vec<usize>, shift: usize) {
    if ones >= zeros {
        input.retain(|x| x & (0x1 << shift) != 0);
    } else {
        input.retain(|x| x & (0x1 << shift) == 0);
    }
}

fn part_two(input: &Vec<usize>, binary_length: usize) -> usize {
    let oxygen_generator_rating = part_two_helper(&input, binary_length, find_oxygen_rating);
    let co2_scrubber_rating = part_two_helper(&input, binary_length, find_co2_rating);
    oxygen_generator_rating * co2_scrubber_rating
}

fn main() -> Result<(), Box<dyn Error>> {
    let contents: String = fs::read_to_string("input.txt")?;
    let binary_inputs: Vec<&str> = contents
        .trim()
        .split("\n")
        .collect();
    let input: Vec<usize> = binary_inputs
        .iter()
        .map(|x| usize::from_str_radix(x.trim(), 2).expect("Could not parse item"))
        .collect();
    let binary_length = binary_inputs[0].trim().len();
    let part_one_answer = part_one(&input, binary_length);
    println!("Part one: {}", part_one_answer);
    let part_two_answer = part_two(&input, binary_length);
    println!("Part two: {}", part_two_answer);
    Ok(())
}
