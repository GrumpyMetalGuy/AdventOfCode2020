use anyhow::Error;
use std::mem;
use AdventOfCode2020::utils::lines_from_file;

fn part_one() -> Result<(), Error> {
    let input = lines_from_file("src/day12.txt")?;

    let mut x_delta = 1;
    let mut y_delta = 0;

    let mut current_x = 0;
    let mut current_y = 0;

    for line in input {
        let (direction_str, distance_str) = line.split_at(1);

        let distance = distance_str.parse::<i32>().unwrap();

        match direction_str {
            "N" => {
                current_y += distance;
            }
            "S" => {
                current_y -= distance;
            }
            "E" => {
                current_x += distance;
            }
            "W" => {
                current_x -= distance;
            }
            "L" => {
                for _ in 0..(distance / 90) {
                    if y_delta == 0 {
                        y_delta = x_delta;
                        x_delta = 0;
                    } else {
                        x_delta = -y_delta;
                        y_delta = 0;
                    }
                }
            }
            "R" => {
                for _ in 0..(distance / 90) {
                    if y_delta == 0 {
                        y_delta = -x_delta;
                        x_delta = 0;
                    } else {
                        x_delta = y_delta;
                        y_delta = 0;
                    }
                }
            }
            "F" => {
                current_x += x_delta * distance;
                current_y += y_delta * distance;
            }
            _ => {
                panic!("Unknown direction")
            }
        }
    }

    println!("Part one answer: {}", current_x.abs() + current_y.abs());

    Ok(())
}

fn part_two() -> Result<(), Error> {
    let input = lines_from_file("src/day12.txt")?;

    let mut x_delta = 10;
    let mut y_delta = 1;

    let mut current_x = 0;
    let mut current_y = 0;

    for line in input {
        let (direction_str, distance_str) = line.split_at(1);

        let distance = distance_str.parse::<i32>().unwrap();

        match direction_str {
            "N" => {
                y_delta += distance;
            }
            "S" => {
                y_delta -= distance;
            }
            "E" => {
                x_delta += distance;
            }
            "W" => {
                x_delta -= distance;
            }
            "L" => {
                for _ in 0..(distance / 90) {
                    mem::swap(&mut x_delta, &mut y_delta);

                    x_delta *= -1;
                }
            }
            "R" => {
                for _ in 0..(distance / 90) {
                    mem::swap(&mut x_delta, &mut y_delta);

                    y_delta *= -1;
                }
            }
            "F" => {
                current_x += x_delta * distance;
                current_y += y_delta * distance;
            }
            _ => {
                panic!("Unknown direction")
            }
        }
    }

    println!("Part two answer: {}", current_x.abs() + current_y.abs());

    Ok(())
}

fn run() -> Result<(), Error> {
    part_one()?;
    part_two()?;

    Ok(())
}

fn main() -> Result<(), Error> {
    run()?;
    Ok(())
}
