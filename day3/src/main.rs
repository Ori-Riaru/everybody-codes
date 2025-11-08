use std::collections::{HashSet, HashMap};
use std::fs;

fn parse_input(path: String) -> Vec<i32> {
    let input = fs::read_to_string(path).expect("Failed to read input file");
    let creates: Vec<i32> = input
        .trim()
        .split(',')
        .map(|s| s.parse().unwrap())
        .collect();

    return creates;
}

fn part1(creates: Vec<i32>) -> i32 {
    let mut set = HashSet::new();

    for create in creates.into_iter() {
        set.insert(create);
    }

    return set.into_iter().sum();
}

fn part2(creates: Vec<i32>) -> i32 {
    let mut set = HashSet::new();
    
    for create in creates.into_iter() {
        set.insert(create);
    }

    let mut creates: Vec<i32> = set.into_iter().collect();
    creates.sort();
    
    return creates.iter().take(20).sum(); 
}

fn part3(creates: Vec<i32>) -> i32 {
    let mut remaining_creates: HashMap<i32, i32> = HashMap::new();

    for create in creates.into_iter() {
        if remaining_creates.contains_key(&create) {
            *remaining_creates.get_mut(&create).unwrap() += 1;
        } else {
            remaining_creates.insert(create, 1);
        }
    }

    let mut set_count = 0;

    while !remaining_creates.is_empty() {
        set_count += 1;

        let mut set = HashSet::new();
        for (create, _) in remaining_creates.iter() {
            set.insert(*create);
        }


        for create in set.into_iter() {
            let count = remaining_creates.get_mut(&create).unwrap();
            *count -= 1;
            if *count == 0 {
                remaining_creates.remove(&create);
            }
        }
    }

    return set_count;
}

fn main() {
    // Part 1
    let creates = parse_input("day3/part1.txt".to_string());
    println!("Part 1: {}", part1(creates));

    // Part 2
    let creates = parse_input("day3/part2.txt".to_string());
    println!("Part 2: {}", part2(creates));

    // Part 3
    let creates = parse_input("day3/part3.txt".to_string());
    println!("Part 3: {}", part3(creates));
}
