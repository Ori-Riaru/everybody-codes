use std::fs;

struct Gear {
    upper_teeth: i32,
    lower_teeth: i32,
}

impl Gear {
    fn new(upper_teeth: i32, lower_teeth: i32) -> Gear {
        return Gear {
            upper_teeth,
            lower_teeth,
        };
    }
}

fn parse_input(path: String) -> Vec<Gear> {
    let input = fs::read_to_string(path).expect("Failed to read input file");

    let gears: Vec<Gear> = input
        .trim()
        .lines()
        .map(|s| {
            let parts = s.split('|').collect::<Vec<&str>>();
            let upper_teeth: i32 = parts[0].trim().parse().unwrap();
            let lower_teeth: i32 = if parts.len() > 1 {
                parts[1].trim().parse().unwrap()
            } else {
                upper_teeth
            };
            Gear::new(upper_teeth, lower_teeth)
        })
        .collect();

    return gears;
}

fn calc_ratio(gears: Vec<Gear>) -> f64 {
    let mut ratio_sum = 1.0;
    for window in gears.windows(2) {
        ratio_sum /= window[1].upper_teeth as f64 / window[0].lower_teeth as f64;
    }

    return ratio_sum;
}

fn part1(speed: f64, gears: Vec<Gear>) -> f64 {
    return speed * calc_ratio(gears);
}

fn part2(speed: f64, gears: Vec<Gear>) -> f64 {
    return speed / calc_ratio(gears);
}

fn main() {
    // Part 1
    let gears = parse_input("day4/part1.txt".to_string());
    println!("Part 1: {}", part1(2025.0, gears).floor());

    // Part 2
    let gears = parse_input("day4/part2.txt".to_string());
    println!("Part 2: {}", part2(10000000000000.0, gears).ceil());

    // Part 3
    let gears = parse_input("day4/part3.txt".to_string());
    println!("Part 3: {}", part1(100.0, gears).floor());
}
