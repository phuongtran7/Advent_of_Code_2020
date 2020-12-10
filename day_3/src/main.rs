use std::{
    fs::File,
    io::{BufRead, BufReader},
    path::Path,
};

fn main() {
    let tree_encouterd_right_3_down_1 = get_trees(3, 1);
    println!(
        "Encoutered right 3 down 1: {} trees",
        tree_encouterd_right_3_down_1
    );

    let tree_encouterd_right_1_down_1 = get_trees(1, 1);
    println!(
        "Encoutered right 1 down 1: {} trees",
        tree_encouterd_right_1_down_1
    );

    let tree_encouterd_right_5_down_1 = get_trees(5, 1);
    println!(
        "Encoutered right 5 down 1: {} trees",
        tree_encouterd_right_5_down_1
    );

    let tree_encouterd_right_7_down_1 = get_trees(7, 1);
    println!(
        "Encoutered right 7 down 1: {} trees",
        tree_encouterd_right_7_down_1
    );

    let tree_encouterd_right_1_down_2 = get_trees(1, 2);
    println!(
        "Encoutered right 1 down 2: {} trees",
        tree_encouterd_right_1_down_2
    );

    let result: i128 = (tree_encouterd_right_3_down_1
        * tree_encouterd_right_1_down_1
        * tree_encouterd_right_5_down_1
        * tree_encouterd_right_7_down_1
        * tree_encouterd_right_1_down_2)
        .into();

    println!("Part 2: {}", result);
}

fn get_trees(right_step: i32, down_step: i32) -> i32 {
    let lines = lines_from_file("input.txt");
    let mut start_location = right_step;
    let start_line = down_step;
    let mut num_tree = 0;

    for num in (start_line as usize..(lines.len() - 1)).step_by(down_step as usize) {
        if check_location_tree(&lines[num], map_location(start_location as usize)) {
            num_tree += 1;
        }
        start_location += right_step;
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
