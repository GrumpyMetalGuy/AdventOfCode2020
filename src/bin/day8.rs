use anyhow::Error;
use std::collections::HashSet;
use AdventOfCode2020::utils::lines_from_file;

#[derive(Clone, Eq, PartialEq)]
struct Instruction {
    pub instruction: String,
    count: i32,
}

impl Instruction {
    pub fn new(instruction: &String) -> Self {
        let instruction_tokens = instruction.split(" ").collect::<Vec<_>>();

        Instruction {
            instruction: instruction_tokens[0].to_string(),
            count: instruction_tokens[1].parse::<i32>().unwrap(),
        }
    }
}

fn part_one(program: &Vec<Instruction>) -> Result<(bool, i32), Error> {
    let mut accumulator = 0;
    let mut visited: HashSet<i32> = HashSet::new();
    let mut program_counter: i32 = 0;
    let program_length = program.len() as i32;

    while !visited.contains(&program_counter) && program_counter < program_length {
        visited.insert(program_counter);

        let instruction = &program[program_counter as usize];

        match instruction.instruction.as_str() {
            "acc" => {
                accumulator += instruction.count;
                program_counter += 1;
            }
            "jmp" => program_counter += instruction.count,
            "nop" => program_counter += 1,
            &_ => {}
        }
    }

    Ok((!visited.contains(&program_counter), accumulator))
}

fn part_two(program: &Vec<Instruction>) -> Result<i32, Error> {
    for program_line_count in 0..program.len() {
        if program[program_line_count as usize].instruction != "acc" {
            let mut program_copy = program.clone();

            let program_instruction = &mut program_copy[program_line_count as usize];

            if program_instruction.instruction == "jmp" {
                program_instruction.instruction = "nop".to_string();
            } else {
                program_instruction.instruction = "jmp".to_string();
            }

            let (finished, accumulator) = part_one(program_copy.as_ref())?;

            if finished == true {
                return Ok(accumulator);
            }
        }
    }

    Ok(-1)
}

fn run() -> Result<(), Error> {
    let input = lines_from_file("src/day8.txt")?;
    let program = input.iter().map(|l| Instruction::new(l)).collect();

    let (_, part_one_answer) = part_one(&program)?;

    println!("Part one answer: {}", part_one_answer);

    let part_two_answer = part_two(&program)?;

    println!("Part two answer: {}", part_two_answer);

    Ok(())
}

fn main() -> Result<(), Error> {
    run()?;
    Ok(())
}
