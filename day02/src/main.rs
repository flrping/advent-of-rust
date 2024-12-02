use std::fs::File;
use std::io::{BufRead, BufReader};

fn main() {
    let file = File::open("input.txt").unwrap();
    let reader = BufReader::new(file);

    let mut safe_reports = 0;

    for line_result in reader.lines() {
        match line_result {
            Ok(line) => {
                let parts: Vec<&str> = line.split(" ").collect();
                let mut is_increasing = false;
                let mut is_safe = true;
                for (i, part) in parts.iter().enumerate() {
                    let current = part.parse::<i32>().unwrap();
                    if i == 0 {
                        continue;
                    }

                    let previous = parts[i - 1].parse::<i32>().unwrap();
                    if !in_limits(current, previous) {
                        is_safe = false;
                        break;
                    }

                    if i == 1 {
                        is_increasing = current > previous;
                    }

                    if is_increasing && current < previous {
                        is_safe = false;
                        break;
                    } else if !is_increasing && current > previous {
                        is_safe = false;
                        break;
                    }
                }
                if is_safe { safe_reports += 1; }
            }
            Err(e) => {
                eprintln!("Error reading line: {}", e);
            }
        }
    }

    println!("Safe reports: {}", safe_reports);
}

fn in_limits(num1: i32, num2: i32) -> bool {
    let diff = (num1 - num2).abs();
    diff >= 1 && diff <= 3
}