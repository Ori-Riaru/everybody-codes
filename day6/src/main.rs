use std::fs;

fn parse_input(path: String) -> String {
    let input = fs::read_to_string(path).expect("Failed to read input file");

    return input;
}

fn part1(target: char, line: &String) -> i32 {
    let mentor_target = target.to_ascii_uppercase();
    let mut mentor_count = 0;
    let mut result = 0;

    for person in line.chars() {
        if person == mentor_target {
            mentor_count += 1;
        } else if person == target {
            result += mentor_count;
        }
    }

    result
}

fn part2(line: String) -> i32 {
    let sword_result = part1('a', &line);
    let archer_result = part1('b', &line);
    let magic_result = part1('c', &line);

    sword_result + archer_result + magic_result
}

fn ranged_mentor_count(target: char, range: u32, line: &String, repeat: u32) -> i32 {
    let mentor_target = target.to_ascii_uppercase() as u8;
    let target = target as u8;
    let mut mentor_count = 0;
    let mut result = 0;

    let max = line.len() as i32 + range as i32;
    let min = 0 - range as i32 - 1;
    for center_index in min..max {
        let exit_index = center_index - range as i32 - 1;
        if exit_index >= 0 && exit_index < line.len() as i32 {
            let exit_person = line.as_bytes().get(exit_index as usize).unwrap();
            if *exit_person == mentor_target {
                mentor_count -= 1;
            }
        }

        let enter_index = center_index + range as i32 ;
        if enter_index >= 0 && enter_index < line.len() as i32 {
            let enter_person = line.as_bytes().get(enter_index as usize).unwrap();
            if *enter_person == mentor_target {
                mentor_count += 1;
            }
        }

        let center_index = center_index;
        if center_index >= 0 && center_index < line.len() as i32 {
            let person = line.as_bytes().get(center_index as usize).unwrap();
            if *person == target {
                result += mentor_count
            }
        };
    }

    result
}

fn part3(range: u32, line: &String, repeat: u32) -> i32 {
    let sword_result = ranged_mentor_count('a', range, &line, repeat);
    let archer_result = ranged_mentor_count('b', range, &line, repeat);
    let magic_result = ranged_mentor_count('c', range, &line, repeat);

    sword_result + archer_result + magic_result
}

fn main() {
    // Part 1
    let line = parse_input("day6/part1.txt".to_string());
    println!("Part 1: {}", part1('a', &line));

    // Part 2
    let line = parse_input("day6/part2.txt".to_string());
    println!("Part 2: {}", part2(line));

    // Part 3
    let line = parse_input("day6/part3.txt".to_string()).repeat(1000);
    println!("Part 3: {}", part3(1000, &line, 1000));
}
