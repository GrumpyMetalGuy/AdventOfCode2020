use anyhow::{anyhow, Error};
use std::collections::HashSet;
use std::iter::successors;
use AdventOfCode2020::utils::lines_from_file;

fn part_one() -> Result<(), Error> {
    let input = lines_from_file("src/day13.txt")?;

    let earliest_time = input[0].parse::<i32>().unwrap();
    let mut min_distances = input[1]
        .split(',')
        .filter(|b| *b != "x")
        .collect::<Vec<_>>()
        .iter()
        .map(|c| {
            let id = c.parse::<i32>().unwrap();
            (id, id - earliest_time % id)
        })
        .collect::<Vec<_>>();

    min_distances.sort_by_key(|(id, dist)| *dist);

    let (id, shortest_distance) = min_distances[0];

    println!("Part one answer: {}", id * shortest_distance);

    Ok(())
}

// Shamelessly lifted from Rosetta Code after reading about the Chinese Remainder Theorem. Yep,
// no credit for this one at all.

fn egcd(a: i64, b: i64) -> (i64, i64, i64) {
    if a == 0 {
        (b, 0, 1)
    } else {
        let (g, x, y) = egcd(b % a, a);
        (g, y - (b / a) * x, x)
    }
}

fn mod_inv(x: i64, n: i64) -> Option<i64> {
    let (g, x, _) = egcd(x, n);
    if g == 1 {
        Some((x % n + n) % n)
    } else {
        None
    }
}

fn chinese_remainder(residues: &[i64], modulii: &[i64]) -> Option<i64> {
    let prod = modulii.iter().product::<i64>();

    let mut sum = 0;

    for (&residue, &modulus) in residues.iter().zip(modulii) {
        let p = prod / modulus;
        sum += residue * mod_inv(p, modulus)? * p
    }

    Some(sum % prod)
}

fn part_two() -> Result<(), Error> {
    let input = lines_from_file("src/day13.txt")?;

    let IDs = input[1]
        .split(',')
        .map(|n| n.parse::<i64>().unwrap_or(5_000_000))
        .enumerate()
        .filter(|(c, i)| *i != 5_000_000)
        .collect::<Vec<_>>();

    // IDs.drain(0..1);

    let modulii = IDs.iter().map(|&(_, r)| r as i64).collect::<Vec<_>>();
    let residues = IDs.iter().map(|&(m, r)| r - m as i64).collect::<Vec<_>>();

    match chinese_remainder(&residues, &modulii) {
        Some(sol) => println!("Part two answer: {}", sol),
        None => println!("modulii not pairwise coprime"),
    }

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
