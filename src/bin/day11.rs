use anyhow::Error;
use counter::Counter;
use ndarray::Array2;
use AdventOfCode2020::utils::lines_from_file;

fn check_seats(part_one: bool) -> Result<(), Error> {
    let input = lines_from_file("src/day11.txt")?;

    let number_of_lines = input.len() as i32;
    let number_of_columns = input[0].len() as i32;

    let mut seats = Array2::from_elem((number_of_lines as usize, number_of_columns as usize), '.');
    let mut new_seats =
        Array2::from_elem((number_of_lines as usize, number_of_columns as usize), '.');

    for (line_count, line) in input.iter().enumerate() {
        for (column_count, position) in line.chars().enumerate() {
            seats[[line_count, column_count]] = position;
        }
    }

    loop {
        let mut changed: bool = false;

        let max_occupancy = if part_one { 4 } else { 5 };

        for line_count in 0..number_of_lines {
            for column_count in 0..number_of_columns {
                let mut occupied_count = 0;

                let current_seat_state = seats[[line_count as usize, column_count as usize]];

                for line_offset in -1..=1 as i32 {
                    for column_offset in -1..=1 as i32 {
                        if !(line_offset == 0 && column_offset == 0) {
                            let mut current_line = line_count as i32 + line_offset;
                            let mut current_column = column_count as i32 + column_offset;

                            while current_line >= 0
                                && current_line < number_of_lines
                                && current_column >= 0
                                && current_column < number_of_columns
                            {
                                let current_seat =
                                    seats[[current_line as usize, current_column as usize]];
                                if current_seat == '#' {
                                    occupied_count += 1;
                                    break;
                                } else if !part_one && current_seat == 'L' {
                                    break;
                                }

                                if part_one {
                                    break;
                                }

                                current_line += line_offset;
                                current_column += column_offset;
                            }
                        }
                    }
                }

                if occupied_count == 0 && current_seat_state == 'L' {
                    new_seats[[line_count as usize, column_count as usize]] = '#';
                    changed = true;
                } else if current_seat_state == '#' && occupied_count >= max_occupancy {
                    new_seats[[line_count as usize, column_count as usize]] = 'L';
                    changed = true;
                } else {
                    new_seats[[line_count as usize, column_count as usize]] = current_seat_state;
                }
            }
        }

        if !changed {
            break;
        }

        seats = new_seats.clone();
    }

    let answer = *seats
        .iter()
        .cloned()
        .collect::<Counter<_>>()
        .get(&'#')
        .unwrap();

    if part_one {
        println!("Part one answer: {}", answer);
    } else {
        println!("Part two answer: {}", answer);
    }

    Ok(())
}

fn run() -> Result<(), Error> {
    check_seats(true)?;
    check_seats(false)?;

    Ok(())
}

fn main() -> Result<(), Error> {
    run()?;
    Ok(())
}
