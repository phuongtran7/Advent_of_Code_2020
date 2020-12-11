use std::{
    fs::File,
    io::{BufRead, BufReader},
    path::Path,
};

#[derive(Default, Debug)]
struct Passport {
    byr: Option<String>,
    iyr: Option<String>,
    eyr: Option<String>,
    hgt: Option<String>,
    hcl: Option<String>,
    ecl: Option<String>,
    pid: Option<String>,
    cid: Option<String>,
}

#[allow(non_camel_case_types)]
enum FieldType {
    byr,
    iyr,
    eyr,
    hgt,
    hcl,
    ecl,
    pid,
    cid,
}

fn main() {
    let lines = lines_from_file("input.txt");
    let mut section_string: String = String::new();
    let mut passports: Vec<Passport> = Vec::new();

    for line_num in 0..lines.len() {
        if line_num == lines.len() - 1 {
            // We are at the end so just parse the last section string
            section_string.push_str(" ");
            section_string.push_str(&lines[line_num]);
            let pass = parse_section(section_string.trim());
            passports.push(pass);
            section_string.clear();
        } else {
            if lines[line_num].is_empty() {
                let pass = parse_section(section_string.trim());
                passports.push(pass);
                section_string.clear();
            } else {
                section_string.push_str(" ");
                section_string.push_str(&lines[line_num]);
            }
        }
    }

    println!("There are {} passport.", passports.len());

    let mut count = 0;
    let mut with_cid = 0;
    for passport in passports {
        if passport.byr.is_some()
            && passport.iyr.is_some()
            && passport.eyr.is_some()
            && passport.hgt.is_some()
            && passport.hcl.is_some()
            && passport.ecl.is_some()
            && passport.pid.is_some()
        {
            //println!("{:?}", passport);

            // We don't need to check for CID field
            count += 1;
            if passport.cid.is_some() {
                with_cid += 1;
            }
        } else {
            continue;
        }
    }

    println!("Total valid passport are {}", count);
    println!("Total valid passport with CID are {}", with_cid);
}

fn validate_type(input: &str, input_type: FieldType) -> Option<String> {
    match input_type {
        FieldType::byr => {
            if input.len() != 4 {
                return None;
            } else {
                let num: i32 = input.parse().unwrap();
                if num >= 1920 && num <= 2002 {
                    return Some(input.to_string());
                }
                None
            }
        }
        FieldType::iyr => {
            if input.len() != 4 {
                return None;
            } else {
                let num: i32 = input.parse().unwrap();
                if num >= 2010 && num <= 2020 {
                    return Some(input.to_string());
                }
                None
            }
        }
        FieldType::eyr => {
            if input.len() != 4 {
                return None;
            } else {
                let num: i32 = input.parse().unwrap();
                if num >= 2020 && num <= 2030 {
                    return Some(input.to_string());
                }
                None
            }
        }
        FieldType::hgt => {
            if input.contains("cm") {
                if let Some(index) = input.find("cm") {
                    let num: i32 = input[..index].parse().unwrap();
                    if num >= 150 && num <= 193 {
                        return Some(input.to_string());
                    }
                }
                return None;
            } else if input.contains("in") {
                if let Some(index) = input.find("in") {
                    let num: i32 = input[..index].parse().unwrap();
                    if num >= 59 && num <= 76 {
                        return Some(input.to_string());
                    }
                }
                return None;
            }
            None
        }
        FieldType::hcl => {
            if input.chars().nth(0).unwrap() == '#' {
                if input.len() == 7 {
                    return Some(input.to_string());
                }
                return None;
            }
            None
        }
        FieldType::ecl => {
            if input == "amb"
                || input == "blu"
                || input == "brn"
                || input == "gry"
                || input == "grn"
                || input == "hzl"
                || input == "oth"
            {
                return Some(input.to_string());
            }
            None
        }
        FieldType::pid => {
            if input.len() == 9 {
                match input.parse::<i32>() {
                    Ok(_) => {
                        return Some(input.to_string());
                    }
                    Err(_) => {
                        return None;
                    }
                }
            }
            None
        }
        FieldType::cid => {
            // Ignore this field
            return Some(input.to_string());
        }
    }
}

fn parse_section(input: &str) -> Passport {
    let mut return_val: Passport = { Default::default() };

    let vec = input.split(" ").collect::<Vec<&str>>();

    for pair in vec {
        // Loop through pairs of item in each line
        let temp: Vec<&str> = pair.split(':').collect();

        match temp[0] {
            "byr" => {
                return_val.byr = validate_type(temp[1], FieldType::byr);
            }
            "iyr" => {
                return_val.iyr = validate_type(temp[1], FieldType::iyr);
            }
            "eyr" => {
                return_val.eyr = validate_type(temp[1], FieldType::eyr);
            }
            "hgt" => {
                return_val.hgt = validate_type(temp[1], FieldType::hgt);
            }
            "hcl" => {
                return_val.hcl = validate_type(temp[1], FieldType::hcl);
            }
            "ecl" => {
                return_val.ecl = validate_type(temp[1], FieldType::ecl);
            }
            "pid" => {
                return_val.pid = validate_type(temp[1], FieldType::pid);
            }
            "cid" => {
                return_val.cid = validate_type(temp[1], FieldType::cid);
            }
            _ => {}
        }
    }

    return_val
}

fn lines_from_file(filename: impl AsRef<Path>) -> Vec<String> {
    let file = File::open(filename).expect("no such file");
    let buf = BufReader::new(file);
    buf.lines()
        .map(|l| l.expect("Could not parse line"))
        .collect()
}
