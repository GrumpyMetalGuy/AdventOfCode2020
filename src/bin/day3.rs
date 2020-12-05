use anyhow::Error;
use ndarray::{Array2, ArrayBase};
use std::iter::Enumerate;
use std::ops::Index;
use AdventOfCode2020::utils::lines_from_file;

fn run() -> Result<(), Error> {
    let lines = lines_from_file("src/day3.txt")?;

    let number_of_lines = lines.len();
    let number_of_columns = lines[0].len();

    let mut input = Array2::from_elem((number_of_lines, number_of_columns), false);

    for (line_count, line) in lines.iter().enumerate() {
        for (column_count, tree) in line.chars().enumerate() {
            input[[line_count, column_count]] = tree == '#';
        }
    }

    let mut part_2_result = 1;

    for (x_delta, y_delta) in vec![(3, 1), (1, 1), (5, 1), (7, 1), (1, 2)] {
        let mut current_x: usize = 0;
        let mut current_y: usize = 0;

        let mut tree_count = 0;

        while current_y < number_of_lines {
            if input[[current_y, current_x]] == true {
                tree_count += 1;
            }

            current_x = (current_x + x_delta) % number_of_columns;
            current_y += y_delta;
        }

        if x_delta == 3 && y_delta == 1 {
            println!("Part 1 answer: {}", tree_count);
        }

        part_2_result *= tree_count;
    }

    println!("Part 2 answer: {}", part_2_result);

    Ok(())
}

fn main() -> Result<(), Error> {
    run()?;
    Ok(())
}
