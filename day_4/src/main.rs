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

fn main() {
    let lines = lines_from_file("input.txt");
    let mut section_string: String = String::new();
    let mut passports: Vec<Passport> = Vec::new();

    for line in lines {
        if line.is_empty() {
            let pass = parse_section(section_string.trim());
            passports.push(pass);
            section_string.clear();
        } else {
            section_string.push_str(" ");
            section_string.push_str(&line);
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
            println!("{:?}", passport);

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

fn parse_section(input: &str) -> Passport {
    let mut return_val: Passport = { Default::default() };

    let vec = input.split(" ").collect::<Vec<&str>>();

    for pair in vec {
        // Loop through pairs of item in each line
        let temp: Vec<&str> = pair.split(':').collect();

        match temp[0] {
            "byr" => {
                return_val.byr = Some(temp[1].to_string());
            }
            "iyr" => {
                return_val.iyr = Some(temp[1].to_string());
            }
            "eyr" => {
                return_val.eyr = Some(temp[1].to_string());
            }
            "hgt" => {
                return_val.hgt = Some(temp[1].to_string());
            }
            "hcl" => {
                return_val.hcl = Some(temp[1].to_string());
            }
            "ecl" => {
                return_val.ecl = Some(temp[1].to_string());
            }
            "pid" => {
                return_val.pid = Some(temp[1].to_string());
            }
            "cid" => {
                return_val.cid = Some(temp[1].to_string());
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
