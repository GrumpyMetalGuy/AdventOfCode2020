use anyhow::Error;
use counter::Counter;
use memoise::memoise;
use std::collections::HashSet;
use AdventOfCode2020::utils::lines_from_file;

fn part_one() -> Result<(), Error> {
    let input = lines_from_file("src/day10.txt")?;
    let mut nodes: HashSet<u32> = input.iter().map(|n| n.parse::<u32>().unwrap()).collect();
    let mut path: Vec<u32> = Vec::new();

    let mut current_node: u32 = 0;

    nodes.insert(0);

    while !nodes.is_empty() {
        path.push(current_node);
        nodes.remove(&current_node);

        if nodes.contains(&(current_node + 1)) {
            current_node += 1;
        } else if nodes.contains(&(current_node + 3)) {
            current_node += 3;
        }
    }

    path.push(path.last().unwrap() + 3);

    let diffs = path
        .windows(2)
        .into_iter()
        .map(|pair| pair[1] - pair[0])
        .collect::<Vec<u32>>()
        .iter()
        .cloned()
        .collect::<Counter<u32>>();

    println!("Part one answer: {}", diffs[&1] * diffs[&3]);

    Ok(())
}

fn part_two() -> Result<(), Error> {
    let input = lines_from_file("src/day10.txt")?;

    let mut nodes = input
        .iter()
        .map(|n| n.parse::<u32>().unwrap())
        .collect::<HashSet<u32>>();

    #[memoise(current_node <= 200)]
    fn walk(current_node: u32, nodes: &HashSet<u32>) -> u64 {
        return if current_node == *nodes.iter().max().unwrap() {
            1
        } else {
            let mut path_count = 0;

            for increment in 1..=3 {
                if nodes.contains(&(current_node + increment)) {
                    path_count += walk(current_node + increment, nodes);
                }
            }

            path_count
        };
    }

    let answer = walk(0, &nodes);

    println!("Part two answer: {}", answer);

    Ok(())
}

fn run() -> Result<(), Error> {
    part_one();
    part_two();

    Ok(())
}

fn main() -> Result<(), Error> {
    run()?;
    Ok(())
}
