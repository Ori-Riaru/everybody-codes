
use std::cmp::{min, max};
use std::fs;

fn parse_input(path: &str) -> (Vec<String>, Vec<i32>, i32) {
    let input = fs::read_to_string(path).expect("Failed to read input file");
    let input_name = input.lines().next().unwrap();
    let input_instructions = input.lines().nth(2).unwrap();

    let names: Vec<String> = input_name.split(',').map(|s| s.to_string()).collect();    
    let instructions: Vec<i32> = input_instructions
        .split(',')
        .map(|elm| if elm.chars().next().unwrap() == 'L' {-1} else {1} * elm[1..].parse::<i32>().unwrap()).collect();
    let bound = names.len() as i32;

    return (names, instructions, bound);
}

fn part1(input: String) -> String {
    let (names, instructions, bound) = parse_input(&input);

    let mut pointer = 0;
    for instruction in &instructions {
        pointer = max(0, min(bound - 1,pointer + instruction));
    }

    return names[pointer as usize].to_string();
}

fn part2(input: String) -> String {
    let (names, instructions, bound) = parse_input(&input);

    let mut pointer = 0;
    for instruction in &instructions {
        pointer =  (((pointer + instruction) % bound) + bound) % bound;
    }

    return names[pointer as usize].to_string();
}

fn part3(input: String) -> String {
    let (mut names, instructions, bound) = parse_input(&input);
    
    for instruction in &instructions {
        let swap_index = (((instruction) % bound) + bound) % bound;
        names.swap(0, swap_index as usize);
    }

    return names[0].to_string();
}


fn main() {
    // Part 1
    println!("Part 1: {}", part1("day1/part1.txt".to_string()));

    // Part 2
    println!("Part 2: {}", part2("day1/part2.txt".to_string()));

    // Part 3
    println!("Part 3: {}", part3("day1/part3.txt".to_string()));
}
