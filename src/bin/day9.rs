use anyhow::{anyhow, Error};
use itertools::Itertools;
use std::collections::{HashSet, VecDeque};
use AdventOfCode2020::utils::lines_from_file;

fn part_one() -> Result<i64, Error> {
    let mut input = lines_from_file("src/day9.txt")?;

    let mut valid_values: VecDeque<i64> = input
        .drain(0..25)
        .map(|l| l.parse::<i64>().unwrap())
        .collect();

    for line in input {
        let next_input = line.parse::<i64>().unwrap();

        let valid_sums: Vec<(&_, &_)> = valid_values
            .iter()
            .tuple_combinations::<(_, _)>()
            .filter(|(&first, &second)| first + second == next_input)
            .collect();

        if valid_sums.is_empty() {
            return Ok(next_input);
        }

        valid_values.pop_front();
        valid_values.push_back(next_input);
    }

    Err(anyhow!("Unable to determine invalid input"))
}

fn part_two(target: i64) -> Result<i64, Error> {
    let input = lines_from_file("src/day9.txt")?
        .iter()
        .map(|l| l.parse::<i64>().unwrap())
        .collect::<Vec<i64>>();

    for range_counter in 2..25 {
        for window in input.windows(range_counter) {
            let sum: i64 = window.iter().sum();

            if sum == target {
                return Ok(window.iter().min().unwrap() + window.iter().max().unwrap());
            }
        }
    }

    Err(anyhow!("Unable to determine valid part two answer"))
}

fn run() -> Result<(), Error> {
    let part_one_result = part_one()?;

    println!("Part one result: {}", part_one_result);

    let part_two_result = part_two(part_one_result)?;

    println!("Part two result: {}", part_two_result);

    Ok(())
}

fn main() -> Result<(), Error> {
    run()?;
    Ok(())
}
