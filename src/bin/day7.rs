use anyhow::Error;
use regex::Regex;
use std::collections::{HashMap, VecDeque};
use AdventOfCode2020::utils::lines_from_file;

fn run() -> Result<(), Error> {
    let input = lines_from_file("src/day7.txt")?;

    let mut rules: HashMap<&str, Vec<(i32, &str)>> = HashMap::new();

    let rule_regex =
        Regex::new("^([a-z ]+) bags contain (.+)").expect("Unable to create rules regex");

    let bags_regex = Regex::new("([0-9]+) ([a-z ]+) bags?").expect("Unable to create bag regex");

    let mut primary_bags: Vec<&str> = Vec::new();

    let mut part_one_answer = 0;

    for rule in &input {
        for capture in rule_regex.captures_iter(&rule) {
            let bag_type = capture.get(1).unwrap().as_str();

            let other_bag_types = capture.get(2).unwrap().as_str();

            primary_bags.push(bag_type);

            for bag_capture in bags_regex.captures_iter(other_bag_types) {
                let other_bag_count = bag_capture
                    .get(1)
                    .unwrap()
                    .as_str()
                    .trim()
                    .parse::<i32>()
                    .unwrap();
                let other_bag_colour = bag_capture.get(2).unwrap().as_str().trim();

                if other_bag_colour != "no other" {
                    rules
                        .entry(bag_type)
                        .or_insert_with(Vec::<(i32, &str)>::new)
                        .push((other_bag_count, other_bag_colour));
                }
            }
        }
    }

    'bag_loop: for primary_bag in primary_bags {
        let mut bag_queue: VecDeque<&str> = VecDeque::new();

        bag_queue.push_back(primary_bag);

        while !bag_queue.is_empty() {
            let current_bag = bag_queue.pop_front().unwrap();

            if current_bag == "shiny gold" {
                part_one_answer += 1;
                continue 'bag_loop;
            }

            if let Some(other_bags) = rules.get(current_bag) {
                for (_, other_bag_colour) in other_bags {
                    bag_queue.push_back(other_bag_colour);
                }
            }
        }
    }

    // Part 2 copy paste
    let mut bag_queue: VecDeque<&str> = VecDeque::new();
    let mut part_two_answer = 0;

    bag_queue.push_back("shiny gold");

    while !bag_queue.is_empty() {
        let current_bag = bag_queue.pop_front().unwrap();

        if let Some(other_bags) = rules.get(current_bag) {
            part_two_answer += 1;

            for (other_bag_count, other_bag_colour) in other_bags {
                for _ in 0..*other_bag_count {
                    bag_queue.push_back(other_bag_colour);
                }
            }
        } else {
            part_two_answer += 1;
        }
    }

    println!("Part one answer: {}", part_one_answer - 1); // Don't account for the shiny gold bag itself
    println!("Part two answer: {}", part_two_answer - 1); // Don't account for the shiny gold bag itself

    Ok(())
}

fn main() -> Result<(), Error> {
    run()?;
    Ok(())
}
