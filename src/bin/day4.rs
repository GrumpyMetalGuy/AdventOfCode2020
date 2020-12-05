use anyhow::Error;
use regex::Regex;
use std::collections::HashSet;
use AdventOfCode2020::utils::lines_from_file;

fn run() -> Result<(), Error> {
    let input = lines_from_file("src/day4.txt")?;

    let mut current_passport_keys: HashSet<&str> = HashSet::new();
    let mut current_strict_keys: HashSet<&str> = HashSet::new();

    let required_keys: HashSet<&str> = ["byr", "iyr", "eyr", "hgt", "hcl", "ecl", "pid"]
        .iter()
        .cloned()
        .collect();

    let mut valid_passports = 0;
    let mut strict_valid_passports = 0;

    let height_regex = Regex::new("^([0-9]+)((cm)|(in))$").expect("Unable to create height regex");
    let hair_regex = Regex::new("^#([0-9a-fA-F]){6}$").expect("Unable to create hair regex");
    let passport_id_regex = Regex::new("^[0-9]{9}$").expect("Unable to create passport ID regex");
    let valid_eye_colours: HashSet<&str> = ["amb", "blu", "brn", "gry", "grn", "hzl", "oth"]
        .iter()
        .cloned()
        .collect();

    for input_line in &input {
        if input_line.trim() == "" {
            // End of record. Check keys.
            if required_keys.is_subset(&current_passport_keys) {
                valid_passports += 1;
            }

            if required_keys.is_subset(&current_strict_keys) {
                strict_valid_passports += 1;
            }

            current_passport_keys.clear();
            current_strict_keys.clear();

            continue;
        }

        for passport_entry in input_line.split(" ") {
            let mut tokens = passport_entry.split(":");
            let key = tokens.next().unwrap().trim();
            let value = tokens.next().unwrap().trim();

            current_passport_keys.insert(&key);

            // Second part requires further validation
            let value_matches = match key {
                "byr" => {
                    let year = value.parse::<usize>().unwrap();
                    year >= 1920 && year <= 2002
                }
                "iyr" => {
                    let year = value.parse::<usize>().unwrap();
                    year >= 2010 && year <= 2020
                }
                "eyr" => {
                    let year = value.parse::<usize>().unwrap();
                    year >= 2020 && year <= 2030
                }
                "hgt" => {
                    let captures = height_regex.captures_iter(value).next();

                    if let Some(first_capture) = captures {
                        let height = first_capture.get(1).unwrap().as_str().parse::<usize>()?;
                        let units = first_capture.get(2).unwrap().as_str();

                        match units {
                            "in" => height >= 59 && height <= 76,
                            "cm" => height >= 150 && height <= 193,
                            &_ => {panic!("Unknown units")}
                        }
                    } else {
                        false
                    }
                }
                "hcl" => hair_regex.is_match(value),
                "ecl" => valid_eye_colours.contains(value.to_lowercase().as_str()),
                "pid" => passport_id_regex.is_match(value),
                "cid" => false,
                &_ => {
                    false
                }
            };

            if value_matches == true {
                current_strict_keys.insert(key);
            }
        }
    }

    println!("Part 1 answer: {}", valid_passports);
    println!("Part 2 answer: {}", strict_valid_passports);

    Ok(())
}

fn main() -> Result<(), Error> {
    run()?;
    Ok(())
}
