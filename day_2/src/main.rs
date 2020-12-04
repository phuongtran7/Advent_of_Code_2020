use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

struct password {
    min: i32,
    max: i32,
    requirement: char,
    password: String,
}

fn main() {
    if let Some(result) = read_input() {
        println!("There are: {} passwords", result.len());
        let mut valid_pass_count = 0;
        for pass in result {
            if check_password_validaty(&pass) {
                valid_pass_count += 1;
            }
        }
        println!("There are: {} valid passwords", valid_pass_count);
    }
}

fn check_password_validaty(input: &password) -> bool {
    let pass = &input.password;
    let mut count = 0;

    let temp_vec: Vec<char> = pass.chars().collect();
    for c in temp_vec {
        if c == input.requirement {
            count += 1;
        }
    }

    if count >= input.min && count <= input.max {
        return true;
    }
    false
}

fn read_input() -> Option<Vec<password>> {
    if let Ok(lines) = read_lines("input.txt") {
        let mut passwords: Vec<password> = Vec::new();

        for line in lines {
            if let Ok(value) = line {
                let v: Vec<&str> = value.split(": ").collect();
                let first_part = v[0];

                let min_max_req: Vec<&str> = first_part.split(' ').collect();
                let min_max: Vec<&str> = min_max_req[0].split('-').collect();

                let min_value: i32 = min_max[0].parse::<i32>().unwrap();
                let max_value: i32 = min_max[1].parse::<i32>().unwrap();

                let req = min_max_req[1].chars().nth(0).unwrap();

                passwords.push(password {
                    min: min_value,
                    max: max_value,
                    requirement: req.clone(),
                    password: v[1].to_string(),
                });
            }
        }
        return Some(passwords);
    }
    None
}

fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where
    P: AsRef<Path>,
{
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}
