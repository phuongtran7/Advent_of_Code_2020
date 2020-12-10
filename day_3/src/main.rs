use std::{
    fs::File,
    io::{BufRead, BufReader},
    path::Path,
};

fn main() {
    let tree_encouterd = get_number_of_tree();
    println!("Encoutered: {} trees", tree_encouterd);
}

fn get_number_of_tree() -> i32 {
    let lines = lines_from_file("input.txt");
    let mut location: usize = 3;
    let mut num_tree = 0;
    // We start counting at the second line as the first line should be ignore when stepping three step to the right
    for num in 1..(lines.len() - 1) {
        if check_location_tree(&lines[num], map_location(location)) {
            num_tree += 1;
        }
        location += 3;
    }
    num_tree
}

fn map_location(location: usize) -> usize {
    // Because the input file is cut off and the patter repeats indefinitely to the right
    // So we need to map it as if the pattern goes on forever
    // We have 32 chars for each line

    let num_of_step = location / 31;
    location - (31 * num_of_step)
}

fn check_location_tree(input: &str, location: usize) -> bool {
    if input.chars().nth(location).unwrap() == '#' {
        return true; // Encounter tree
    }
    false
}

fn lines_from_file(filename: impl AsRef<Path>) -> Vec<String> {
    let file = File::open(filename).expect("no such file");
    let buf = BufReader::new(file);
    buf.lines()
        .map(|l| l.expect("Could not parse line"))
        .collect()
}
