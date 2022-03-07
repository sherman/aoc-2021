use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

#[cfg(test)]
mod tests {
    use crate::read_lines;

    #[test]
    fn day1() {
        let mut prev = String::new();
        let mut counter = 0;
        if let Ok(lines) = read_lines("input/day_1") {
            for line in lines {
                if let Ok(data) = line {
                    if !prev.is_empty() {
                        let prev_number: i32 = prev.parse().unwrap();
                        let current_number: i32 = data.parse().unwrap();
                        if prev_number < current_number {
                            counter = counter + 1;
                        }
                    }
                    prev = data;
                }
            }
        }

        println!("Result: {}", counter);
    }
}

fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
    where P: AsRef<Path>, {
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}
