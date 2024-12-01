use std::fs::File;
use std::io::{BufRead, BufReader};

fn main() {
    let file = File::open("input.txt").unwrap();
    let reader = BufReader::new(file);

    let mut from: Vec<i32> = Vec::with_capacity(1000);
    let mut to: Vec<i32> = Vec::with_capacity(1000);

    for line_result in reader.lines() {
        match line_result {
            Ok(line) => {
                let parts: Vec<&str> = line.split("   ").collect();
                if parts.len() > 1 {
                    from.push(parts[0].parse::<i32>().unwrap());
                    to.push(parts[1].parse::<i32>().unwrap());
                } else {
                    println!("Line doesn't contain enough parts: {}", line);
                }
            }
            Err(e) => {
                eprintln!("Error reading line: {}", e);
            }
        }
    }

    from.sort();
    to.sort();

    let mut total_distance = 0;
    for i in 0..from.len() {
        total_distance += (to[i] - from[i]).abs();
    }

    println!("Total distance: {}", total_distance);
}
