use std::fmt;
use std::fs;
use std::ops::{Add, Div, Mul};
#[derive(Clone, Debug)]
struct Complex {
    x: i64,
    y: i64,
}

impl Complex {
    fn new(x: i64, y: i64) -> Self {
        Complex { x, y }
    }
}

impl fmt::Display for Complex {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "[{},{}]", self.x, self.y)
    }
}

impl Add for Complex {
    type Output = Complex;

    fn add(self, other: Complex) -> Complex {
        Complex::new(self.x + other.x, self.y + other.y)
    }
}

impl Mul for Complex {
    type Output = Complex;

    fn mul(self, other: Complex) -> Complex {
        Complex::new(
            self.x * other.x - self.y * other.y,
            self.x * other.y + self.y * other.x,
        )
    }
}

impl Div for Complex {
    type Output = Complex;

    fn div(self, other: Complex) -> Complex {
        Complex::new(self.x / other.x, self.y / other.y)
    }
}

fn parse_input(path: String) -> Complex {
    let input = fs::read_to_string(path).expect("Failed to read input file");
    
    let mut parts = input[3..input.len() - 1].split(',');
    let x = parts.next().unwrap().parse().unwrap();
    let y = parts.next().unwrap().parse().unwrap();

    Complex::new(x, y)
}

fn part1(start: Complex) -> Complex {
    let mut result = Complex::new(0, 0);
    for _ in 0..3 {
        result = result.clone() * result;
        result = result / Complex::new(10, 10);
        result = result + start.clone();
    }

    return result;
}

fn part2(start: Complex, grid_dim: i64) -> (i64, Vec<bool>) {
    let cell_count = grid_dim * grid_dim; 
    let mut marked = vec![true; cell_count as usize];
    let mut mark_count = 0;
    let block_size = 1000 / (grid_dim as i64 - 1);

    // Check each point in the grid
    for i in 0..cell_count {
        // Calculate the point
        let point = Complex::new((i % grid_dim) as i64 * block_size, (i / grid_dim) as i64 * block_size) + start.clone();

        // Check validity of the point
        let mut result = Complex::new(0, 0);
        for _ in 0..100 {
            result = result.clone() * result;
            result = result / Complex::new(100000, 100000);
            result = result + point.clone();

            // Point out of bounds
            if result.x >= 1000000 || result.x <= -1000000 || result.y >= 1000000 || result.y <= -1000000 {
                mark_count += 1;
                marked[i as usize] = false;
                break;
            }
        }
    }

    return (cell_count - mark_count, marked);
}

fn print_grid(marked: &Vec<bool>, grid_dim: i64, frac: i64) {
    // Block characters for 2x2 resolution
    const BLOCKS: [&str; 16] = [
        " ",  // 0000
        "▗",  // 0001
        "▖",  // 0010
        "▄",  // 0011
        "▝",  // 0100
        "▐",  // 0101
        "▞",  // 0110
        "▟",  // 0111
        "▘",  // 1000
        "▚",  // 1001
        "▌",  // 1010
        "▙",  // 1011
        "▀",  // 1100
        "▜",  // 1101
        "▛",  // 1110
        "█",  // 1111
    ];

    for y in (0..grid_dim / frac).step_by(2) {
        for x in (0..grid_dim / frac).step_by(2) {
            let mut pattern = 0;
            
            // Top-left
            if y < grid_dim / frac && x < grid_dim / frac {
                let idx = (y * grid_dim * frac + x) as usize;
                if idx < marked.len() && marked[idx] {
                    pattern |= 0b1000;
                }
            }
            
            // Top-right
            if y < grid_dim / frac && x + 1 < grid_dim / frac {
                let idx = (y * grid_dim * frac + x + 1) as usize;
                if idx < marked.len() && marked[idx] {
                    pattern |= 0b0100;
                }
            }
            
            // Bottom-left
            if y + 1 < grid_dim / frac && x < grid_dim / frac {
                let idx = ((y + 1) * grid_dim * frac + x) as usize;
                if idx < marked.len() && marked[idx] {
                    pattern |= 0b0010;
                }
            }
            
            // Bottom-right
            if y + 1 < grid_dim / frac && x + 1 < grid_dim / frac {
                let idx = ((y + 1) * grid_dim * frac + x + 1) as usize;
                if idx < marked.len() && marked[idx] {
                    pattern |= 0b0001;
                }
            }
            
            print!("{}", BLOCKS[pattern]);
        }
        println!();
    }
}

fn main() {
    // Part 1
    let a = parse_input("day2/part1.txt".to_string());
    println!("Part 1: {}", part1(a));

    // Part 2
    let a = parse_input("day2/part2.txt".to_string());
    let (result, pic) = part2(a.clone(), 101);
    println!("Part 2: {}", result);
    print_grid(&pic, 101, 1);

    // Part 3
    let a = parse_input("day2/part3.txt".to_string());
    let (result, pic) = part2(a.clone(), 1001);
    print_grid(&pic, 1001, 2);
    println!("Part 3: {}", result);
}
