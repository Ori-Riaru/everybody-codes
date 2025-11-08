use std::fs;

fn parse_input(path: String) -> Vec<i32> {
    let input = fs::read_to_string(path).expect("Failed to read input file");
    let inputs: Vec<i32> = input
        .trim()
        .split(',')
        .map(|s| s.parse().unwrap())
        .collect();

    return inputs;
}

fn part1(creates: Vec<i32>) -> i32 {
    
    todo!()
}

fn part2(creates: Vec<i32>) -> i32 {
    
    todo!()
}

fn part3(creates: Vec<i32>) -> i32 {
    
    todo!()
}

fn main() {
    // Part 1
    let inputs = parse_input("day5/test1.txt".to_string());
    println!("Part 1: {}", part1(inputs));

    // Part 2
    let inputs = parse_input("day5/test2.txt".to_string());
    println!("Part 2: {}", part2(inputs));

    // Part 3
    let inputs = parse_input("day5/test3.txt".to_string());
    println!("Part 3: {}", part3(inputs));
}
