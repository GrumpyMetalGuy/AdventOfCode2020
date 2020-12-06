use anyhow::Error;
use counter::Counter;
use std::collections::HashSet;
use AdventOfCode2020::utils::lines_from_file;

fn run() -> Result<(), Error> {
    let input = lines_from_file("src/day6.txt")?;

    let mut part_one_count = 0;
    let mut part_two_count = 0;
    let mut questions: Vec<char> = Vec::new();
    let mut people: usize = 0;

    for line in input {
        if line.trim().is_empty() {
            part_one_count += questions.iter().cloned().collect::<HashSet<char>>().len();

            let counted_answers = questions.iter().cloned().collect::<Counter<_>>();

            for (_, count) in counted_answers.iter() {
                if *count == people {
                    part_two_count += 1;
                }
            }

            questions.clear();
            people = 0;

            continue;
        }

        for question in line.trim().chars() {
            questions.push(question);
        }

        people += 1;
    }

    println!("Part one answer: {}", part_one_count);
    println!("Part two answer: {}", part_two_count);

    Ok(())
}

fn main() -> Result<(), Error> {
    run()?;
    Ok(())
}
