use anyhow::Error;
use AdventOfCode2020::utils::lines_from_file;
use std::collections::HashSet;

fn run() -> Result<(), Error> {
    let input = lines_from_file("src/day5.txt")?;
    let mut seat_IDs: Vec<i32> = Vec::new();

    for line in input {
        let mut min_row = 0;
        let mut max_row = 127;

        let mut min_col = 0;
        let mut max_col = 7;

        let mut row_chars = line.chars().collect::<Vec<_>>();
        let row_config = row_chars.drain(0..7).collect::<Vec<_>>();
        let col_config = row_chars.drain(0..3).collect::<Vec<_>>();

        for input_char in row_config {
            let diff = (max_row - min_row + 1) / 2;
            match input_char {
                'F' => max_row -= diff,
                'B' => min_row += diff,
                _ => {panic!("Unexpected config char for row {}", input_char)}
            }
        }

        for input_char in col_config {
            let diff = (max_col - min_col + 1) / 2;
            match input_char {
                'L' => max_col -= diff,
                'R' => min_col += diff,
                _ => {panic!("Unexpected config char for column {}", input_char)}
            }
        }

        seat_IDs.push(min_row * 8 + min_col);
    }

    seat_IDs.sort();

    let min_ID = *seat_IDs.iter().min().unwrap();
    let max_ID = *seat_IDs.iter().max().unwrap();

    println!("Part 1 answer: {}", max_ID);

    let IDs: HashSet<i32> = seat_IDs.iter().cloned().collect();

    for ID in (min_ID + 1)..=(max_ID - 1) {
        if !IDs.contains(&ID) && IDs.contains(&(ID + 1)) && IDs.contains(&(ID - 1)) {
            println!("Part 2 answer: {}", ID);
        }
    }

    Ok(())
}

fn main() -> Result<(), Error> {
    run()?;
    Ok(())
}
