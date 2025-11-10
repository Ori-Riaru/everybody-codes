use std::cmp::Ordering;
use std::fs;
#[derive(Debug, Clone)]
struct Node {
    value: i32,
    left: Option<i32>,
    right: Option<i32>,
    center: Option<Box<Node>>,
}

impl Node {
    fn new(value: i32) -> Self {
        Node {
            value,
            left: None,
            right: None,
            center: None,
        }
    }
}

#[derive(Debug, Clone)]
struct Fishbone {
    id: usize,
    root: Option<Box<Node>>,
}

impl Fishbone {
    fn new(id: usize, nums: Vec<i32>) -> Self {
        let mut bone = Fishbone { id,  root: None };

        for num in nums {
            bone.insert(num)
        }

        bone
    }

    fn insert(&mut self, value: i32) {
        if self.root.is_none() {
            self.root = Some(Box::new(Node::new(value)));
            return;
        }

        let mut current = self.root.as_mut().unwrap();
        loop {
            if value < current.value && current.left.is_none() {
                current.left = Some(value);
                break;
            } else if value > current.value && current.right.is_none() {
                current.right = Some(value);
                break;
            } else if current.center.is_none() {
                current.center = Some(Box::new(Node::new(value)));
                break;
            }

            current = current.center.as_mut().unwrap();
        }
    }

    fn quality(&self) -> i64 {        
        let mut result = String::new();

        let mut current = self.root.as_ref();
        while let Some(node) = current {
            result.push_str(&node.value.to_string());
            current = node.center.as_ref();
        }
        
        result.parse::<i64>().unwrap()
    }
}

fn parse_input(path: String) -> Vec<Fishbone> {
    let input = fs::read_to_string(path).expect("Failed to read input file");
    let fishbones = input
        .lines()
        .map(|line| {
            let parts = line.trim().split(':').collect::<Vec<&str>>();

            let id = parts[0].parse().expect("Failed to parse ID");
            let nums: Vec<i32> = parts[1]
                .trim()
                .split(',')
                .map(|s| s.parse().expect("Failed to parse fishbone num"))
                .collect();

            Fishbone::new(id, nums)
        })
        .collect();
    return fishbones;
}

fn part1(fishbone: Fishbone) -> i64 {
    return fishbone.quality();
}

fn part2(fishbones: Vec<Fishbone>) -> i64 {
    let mut qualities = Vec::new();
    for bone in fishbones {
        qualities.push(bone.quality());
    }
    qualities.sort();

    return qualities.last().unwrap() - qualities.first().unwrap();
}

fn part3(mut fishbones: Vec<Fishbone>) -> i32 {
    fishbones.sort_by(|f1, f2| {
        let compare = f1.quality().cmp(&f2.quality());
        if compare != Ordering::Equal {
            return compare;
        }

        let mut current1 = f1.root.as_ref();
        let mut current2 = f2.root.as_ref();
        loop {
            match (current1, current2) {
                (Some(c1), Some(c2)) => {
                    let mut level1_str = String::new();
                    if let Some(ref left) = c1.left {
                        level1_str.push_str(&left.to_string());
                    }
                    level1_str.push_str(&c1.value.to_string());
                    if let Some(ref right) = c1.right {
                        level1_str.push_str(&right.to_string());
                    }

                    let mut level2_str = String::new();
                    if let Some(ref left) = c2.left {
                        level2_str.push_str(&left.to_string());
                    }
                    level2_str.push_str(&c2.value.to_string());
                    if let Some(ref right) = c2.right {
                        level2_str.push_str(&right.to_string());
                    }

                    if level1_str != level2_str {
                        let level1_num = level1_str.parse::<i64>().unwrap();
                        let level2_num = level2_str.parse::<i64>().unwrap();
                        if level1_num != level2_num {
                            return level1_num.cmp(&level2_num);
                        }
                    }

                    current1 = c1.center.as_ref();
                    current2 = c2.center.as_ref();
                }
                (None, None) => return f1.id.cmp(&f2.id),
                (None, Some(_)) => return Ordering::Greater,
                (Some(_), None) => return Ordering::Less,
            }
        }
    });
    fishbones.reverse();

    let mut result = 0;
    for (i, bone) in fishbones.iter().enumerate() {
        result += bone.id as i32 * (i + 1) as i32;
    }

    return result;
}

fn main() {
    // Part 1
    let fishbones = parse_input("day5/part1.txt".to_string());
    println!("Part 1: {}", part1(fishbones[0].clone()));

    // Part 2
    let fishbones = parse_input("day5/part2.txt".to_string());
    println!("Part 2: {:?}", part2(fishbones));

    // Part 3
    let fishbones = parse_input("day5/part3.txt".to_string());
    println!("Part 3: {}", part3(fishbones));
}
