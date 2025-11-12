use std::fs;
use std::collections::{HashMap, HashSet};
use std::path::Prefix;
use itertools::Itertools;

fn parse_input(path: String) -> (Vec<String>, HashMap<char, Vec<char>>) {
    let input = fs::read_to_string(path).expect("Failed to read input file");
    let names: Vec<String> = input.lines().next().unwrap().split(",").map(|s| s.to_string()).collect();
    let rules: HashMap<char, Vec<char>> = input.lines().skip(2)
        .map(|s| {
            let mut parts = s.split(">");
            let key = parts.next().unwrap().chars().next().unwrap();
            let value = parts.next().unwrap().split(",").map(|s| s.trim().chars().next().unwrap()).collect();

            (key, value)
        })
        .collect();

    (names, rules)
}

fn part1(names: Vec<String>,  rules: HashMap<char, Vec<char>>) -> String {
    
    let mut result = "".to_string();
    'outer: for name in names.iter() {
        for (current, next) in name.chars().tuple_windows() {
            if !rules.get(&current).unwrap().contains(&next) {
                continue 'outer;
            }
        }

        result = name.clone();
        break;
    }
    
    result 
}

fn part2(names: Vec<String>, rules: HashMap<char, Vec<char>>) -> (Vec<String>, i32) {
    
    let mut result = 0;
    let mut list = Vec::new();
    'outer: for (i,name) in names.iter().enumerate() {
        for (current, next) in name.chars().tuple_windows() {
            if !rules.get(&current).unwrap().contains(&next) {
                continue 'outer;
            }
        }

        result += i + 1;
        list.push(name.clone());
    }
    
    (list, result as i32)
}

fn part3(prefixes: Vec<String>, rules: HashMap<char, Vec<char>>) -> i32 {
    let mut stack: Vec<String> = Vec::new();
    let mut found: HashSet<String> = HashSet::new();

    let (prefixes, _) = part2(prefixes, rules.clone());
    for prefix in prefixes {
        stack.push(prefix);            
    }
        
    
    let mut count = 0;
    while let Some(item) = stack.pop() {
        if found.contains(&item) {
            continue;
        }
        
        found.insert(item.clone());
        if item.len() >= 7 {
            count += 1;
        }

        for next in  rules.get(&item.chars().last().unwrap()).unwrap_or(&Vec::<char>::new()) {
            let mut new = item.clone();
            new.push(*next);
            
            if new.len() > 11 {
                continue;
            }

            stack.push(new);
        }
    }   

    count
}

fn main() {
    // Part 1
    let (names, rules) = parse_input("day7/part1.txt".to_string());
    println!("Part 1: {}", part1(names, rules));

    // Part 2
    let (names, rules) = parse_input("day7/part2.txt".to_string());
    println!("Part 2: {}", part2(names, rules).1);

    // Part 3
    let (names, rules) = parse_input("day7/part3.txt".to_string());
    println!("Part 3: {}", part3(names, rules));
}
