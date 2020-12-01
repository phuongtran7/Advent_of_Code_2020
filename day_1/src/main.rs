use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

fn main() {
    if let Some(result) = read_input() {
        if let Some((first, second)) = find_pair(&result) {
            println!("{} * {} is {}", first, second, first * second);
        }

        if let Some((first, second, third)) = find_triplet(&result) {
            println!(
                "{} * {} * {} is {}",
                first,
                second,
                third,
                first * second * third
            );
        }
    }
}

fn find_triplet(input: &Vec<i32>) -> Option<(i32, i32, i32)> {
    for num in input.iter() {
        let remainder = 2020 - num;

        for second_num in input.iter() {
            if let Some(found) = input.iter().find(|&&x| x == (remainder - second_num)) {
                return Some((num.to_owned(), second_num.to_owned(), found.to_owned()));
            }
            continue;
        }
    }
    None
}

fn find_pair(input: &Vec<i32>) -> Option<(i32, i32)> {
    for num in input.iter() {
        let other = 2020 - num;
        if let Some(found) = input.iter().find(|&&x| x == other) {
            return Some((num.to_owned(), found.to_owned()));
        }
        continue;
    }
    None
}

fn read_input() -> Option<Vec<i32>> {
    let mut inputs: Vec<i32> = Vec::new();

    if let Ok(lines) = read_lines("input.txt") {
        for line in lines {
            if let Ok(num) = line {
                let value: i32 = num.parse().unwrap();
                inputs.push(value);
            }
        }
        return Some(inputs);
    } else {
        None
    }
}

fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where
    P: AsRef<Path>,
{
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}
