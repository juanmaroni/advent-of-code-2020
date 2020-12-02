use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

// From documentation "Rust by Example"
fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>> where P:AsRef<Path>, {
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}

pub fn numbers_from_file(filename: &str, year: i16) -> Vec<i16> {
    let mut numbers: Vec<i16> = Vec::new();

    // Assumptions:
    // - The input has only positive integers (unsigned int), so numbers higher than 2020 are out.
    // - u16 and i16 are enough fo the few years to come.
    if let Ok(lines) = read_lines(filename) {
        for line in lines {
            if let Ok(number) = line {
                let n: i16 = number.parse::<i16>().unwrap();    // TODO: I could handle error on this

                // Numbers can't be greater than the year, so I will exclude them.
                if n <= year {
                    numbers.push(n);
                }
            }
        }
    }

    numbers
}