use std::{
    collections::HashSet,
    io::{self, BufRead},
};

struct State {
    head_pos: Position,
    tail_pos: Position,
}

impl State {
    fn new() -> Self {
        Self {
            head_pos: Position { x: 0, y: 0 },
            tail_pos: Position { x: 0, y: 0 },
        }
    }
}

#[derive(Hash, PartialEq, Eq, Clone, Copy)]
struct Position {
    x: usize,
    y: usize,
}

enum Instruction {
    Up,
    Down,
    Left,
    Right,
}

fn parse_instructions(lines: &[String]) -> Vec<Instruction> {
    let mut instructions = Vec::new();
    for line in lines {
        let line_parts: Vec<&str> = line.split(' ').collect();
        let count: u32 = line_parts[1].parse().unwrap();
        for _ in 0..count {
            instructions.push(match line_parts[0] {
                "U" => Instruction::Up,
                "D" => Instruction::Down,
                "L" => Instruction::Left,
                "R" => Instruction::Right,
                _ => panic!(),
            })
        }
    }
    instructions
}

fn execute_instruction(state: &mut State, instruction: Instruction) {
    match instruction {
        Instruction::Up => {
            state.head_pos.y += 1;
            state.tail_pos.y += 1;
        }
        Instruction::Down => {
            state.head_pos.y -= 1;
            state.tail_pos.y -= 1;
        }
        Instruction::Left => {
            state.head_pos.x -= 1;
            state.tail_pos.x -= 1;
        }
        Instruction::Right => {
            state.head_pos.x += 1;
            state.tail_pos.x += 1;
        }
    }
}

fn get_distance(pos_1: &Position, pos_2: &Position) -> usize {}

fn main() {
    let mut tail_positions = HashSet::new();
    let mut state = State::new();
    let lines: Vec<_> = io::stdin().lock().lines().map(|ln| ln.unwrap()).collect();
    let instructions = parse_instructions(&lines);

    for instruction in instructions {
        execute_instruction(&mut state, instruction);
        tail_positions.insert(state.tail_pos);
    }

    println!("Part 1: {}", "");
    println!("Part 2: {}", "");
}

#[cfg(test)]
mod test {}
