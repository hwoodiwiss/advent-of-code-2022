use std::io::{self, BufRead};

enum Instruction {
    Move {
        count: usize,
        source: usize,
        target: usize,
    },
    MoveMul {
        count: usize,
        source: usize,
        target: usize,
    },
}

fn main() {
    let lines: Vec<_> = io::stdin().lock().lines().map(|ln| ln.unwrap()).collect();
    // Split on the \n separating stacks from instructions
    let mut input_parts = lines.split(|ln| ln.is_empty());
    let stack_text = input_parts.next().unwrap();
    let instruction_text = input_parts.next().unwrap();

    // Part 1
    let mut stacks_1 = read_stacks(stack_text);
    let instructions = read_instructions(instruction_text, false);

    instructions
        .iter()
        .for_each(|inst| run_instruction(inst, &mut stacks_1));

    let part_1: String = String::from_iter(
        stacks_1
            .iter()
            .map(|stack| stack.last().unwrap())
            .collect::<Vec<_>>(),
    );

    // Part 2
    let mut stacks_2 = read_stacks(stack_text);
    let instructions_move_mul = read_instructions(instruction_text, true);

    instructions_move_mul
        .iter()
        .for_each(|inst| run_instruction(inst, &mut stacks_2));

    let part_2: String = String::from_iter(
        stacks_2
            .iter()
            .map(|stack| stack.last().unwrap())
            .collect::<Vec<_>>(),
    );

    println!("Part 1: {}", part_1);
    println!("Part 2: {}", part_2);
}

fn read_stacks(lines: &[String]) -> Vec<Vec<char>> {
    let mut stacks = Vec::new();
    let lines_chars = lines
        .iter()
        .map(|s| s.chars().collect::<Vec<char>>())
        .collect::<Vec<Vec<char>>>();
    let last_line = lines_chars.last().unwrap();
    for (i, c) in last_line.iter().enumerate() {
        let mut item_stack = Vec::new();
        if !c.is_whitespace() {
            for item_line in lines_chars.iter().rev().skip(1) {
                if item_line[i].is_alphabetic() {
                    item_stack.push(item_line[i]);
                } else {
                    break;
                }
            }
            stacks.push(item_stack);
        }
    }

    stacks
}

fn read_instructions(lines: &[String], move_mul: bool) -> Vec<Instruction> {
    lines
        .iter()
        .map(|ln| parse_instruction(ln, move_mul))
        .collect()
}

fn parse_instruction(line: &String, move_mul: bool) -> Instruction {
    let split_line = line.split(' ').collect::<Vec<_>>();
    match split_line[0] {
        "move" if move_mul => Instruction::MoveMul {
            count: split_line[1].parse().unwrap(),
            source: split_line[3].parse::<usize>().unwrap() - 1,
            target: split_line[5].parse::<usize>().unwrap() - 1,
        },
        "move" => Instruction::Move {
            count: split_line[1].parse().unwrap(),
            source: split_line[3].parse::<usize>().unwrap() - 1,
            target: split_line[5].parse::<usize>().unwrap() - 1,
        },
        _ => unimplemented!(),
    }
}

fn run_instruction(instruction: &Instruction, state: &mut Vec<Vec<char>>) {
    match instruction {
        Instruction::Move {
            count,
            source,
            target,
        } => {
            for _ in 0..*count {
                let val = state[*source].pop().unwrap();
                state[*target].push(val);
            }
        }
        Instruction::MoveMul {
            count,
            source,
            target,
        } => {
            let len = state[*source].len();
            let mut items = (&state[*source][len - *count..])
                .iter()
                .map(|c| *c)
                .collect::<Vec<char>>();
            state[*target].append(&mut items);
            for i in (len - *count..len).rev() {
                state[*source].remove(i);
            }
        }
    }
}

#[cfg(test)]
mod test {
    use crate::{read_instructions, read_stacks, run_instruction, Instruction};

    #[test]
    fn test_read_stacks() {
        let lines = [
            "    [D]    ".to_owned(),
            "[N] [C]    ".to_owned(),
            "[Z] [M] [P]".to_owned(),
            " 1   2   3 ".to_owned(),
        ];

        let stacks = read_stacks(&lines);

        assert_eq!(3, stacks.len());

        assert_eq!(2, stacks[0].len());
        assert_eq!('Z', stacks[0][0]);
        assert_eq!('N', stacks[0][1]);

        assert_eq!(3, stacks[1].len());
        assert_eq!('M', stacks[1][0]);
        assert_eq!('C', stacks[1][1]);
        assert_eq!('D', stacks[1][2]);

        assert_eq!(1, stacks[2].len());
        assert_eq!('P', stacks[2][0]);
    }

    #[test]
    fn test_read_instructions() {
        let lines = [
            "move 1 from 2 to 1".to_owned(),
            "move 3 from 1 to 3".to_owned(),
            "move 2 from 2 to 1".to_owned(),
            "move 1 from 1 to 2".to_owned(),
        ];

        let instructions = read_instructions(&lines, false);

        assert_eq!(4, instructions.len());

        assert_move_instruction(&instructions[0], 1, 1, 0);
        assert_move_instruction(&instructions[1], 3, 0, 2);
        assert_move_instruction(&instructions[2], 2, 1, 0);
        assert_move_instruction(&instructions[3], 1, 0, 1);
    }

    #[test]
    fn test_run_instructions() {
        let lines = [
            "    [D]    ".to_owned(),
            "[N] [C]    ".to_owned(),
            "[Z] [M] [P]".to_owned(),
            " 1   2   3 ".to_owned(),
        ];

        let mut stacks = read_stacks(&lines);

        let instruction_lines = [
            "move 1 from 2 to 1".to_owned(),
            "move 3 from 1 to 3".to_owned(),
            "move 2 from 2 to 1".to_owned(),
            "move 1 from 1 to 2".to_owned(),
        ];

        let instructions = read_instructions(&instruction_lines, false);

        for inst in instructions {
            run_instruction(&inst, &mut stacks);
            println!("{:?}", stacks);
        }

        let part_1: String = String::from_iter(
            stacks
                .iter()
                .map(|stack| stack.last().unwrap())
                .collect::<Vec<_>>(),
        );

        assert_eq!("CMZ", part_1.as_str())
    }

    #[test]
    fn test_run_instructions_move() {
        let lines = [
            "    [D]    ".to_owned(),
            "[N] [C]    ".to_owned(),
            "[Z] [M] [P]".to_owned(),
            " 1   2   3 ".to_owned(),
        ];

        let mut stacks = read_stacks(&lines);

        let instruction_lines = [
            "move 1 from 2 to 1".to_owned(),
            "move 3 from 1 to 3".to_owned(),
            "move 2 from 2 to 1".to_owned(),
            "move 1 from 1 to 2".to_owned(),
        ];

        let instructions = read_instructions(&instruction_lines, true);

        for inst in instructions {
            run_instruction(&inst, &mut stacks);
            println!("{:?}", stacks);
        }

        let part_2: String = String::from_iter(
            stacks
                .iter()
                .map(|stack| stack.last().unwrap())
                .collect::<Vec<_>>(),
        );

        assert_eq!("MCD", part_2.as_str())
    }

    fn assert_move_instruction(
        instruction: &Instruction,
        expected_count: usize,
        expected_source: usize,
        expected_target: usize,
    ) {
        match instruction {
            Instruction::Move {
                count,
                source,
                target,
            } => {
                assert_eq!(*count, expected_count);
                assert_eq!(*source, expected_source);
                assert_eq!(*target, expected_target);
            }
            Instruction::MoveMul {
                count,
                source,
                target,
            } => {
                assert_eq!(*count, expected_count);
                assert_eq!(*source, expected_source);
                assert_eq!(*target, expected_target);
            }
            _ => assert!(false),
        };
    }
}
