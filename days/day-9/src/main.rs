use std::{
    borrow::BorrowMut,
    collections::HashSet,
    io::{self, BufRead},
};

struct State {
    positions: Vec<Position>,
}

impl State {
    fn new(num_positions: usize) -> Self {
        let mut positions = Vec::with_capacity(num_positions);

        (0..num_positions).for_each(|i| positions.insert(i, Position { x: 0, y: 0 }));

        Self { positions }
    }

    fn tail(&self) -> Position {
        *self.positions.last().unwrap()
    }
}

#[derive(Hash, PartialEq, Eq, Clone, Copy)]
struct Position {
    x: i32,
    y: i32,
}

enum Instruction {
    Up,
    Down,
    Left,
    Right,
}

const UNIT_HYPOTENUSE: f32 = 1.41421356237;

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

fn execute_instruction(state: &mut State, instruction: &Instruction) {
    let head_pos = state.positions.first_mut().unwrap();

    match instruction {
        Instruction::Up => {
            head_pos.y += 1;
        }
        Instruction::Down => {
            head_pos.y -= 1;
        }
        Instruction::Left => {
            head_pos.x -= 1;
        }
        Instruction::Right => {
            head_pos.x += 1;
        }
    }

    let mut prev_pos = *head_pos;

    for position in &mut state.positions[1..] {
        move_if_needed(&prev_pos, position);
        prev_pos = *position;
    }
}

fn move_if_needed(prev_pos: &Position, curr_pos: &mut Position) {
    let distance = get_distance(prev_pos, curr_pos);
    if distance.x.abs() == 2 && distance.y == 0 {
        curr_pos.x += if distance.x > 0 { 1 } else { -1 };
    } else if distance.y.abs() == 2 && distance.x == 0 {
        curr_pos.y += if distance.y > 0 { 1 } else { -1 };
    } else if (distance.y.abs() == 2 && [1, 2].contains(&distance.x.abs()))
        || (distance.x.abs() == 2 && [1, 2].contains(&distance.y.abs()))
    {
        curr_pos.x += if distance.x > 0 { 1 } else { -1 };
        curr_pos.y += if distance.y > 0 { 1 } else { -1 };
    }
}

fn get_distance(pos_1: &Position, pos_2: &Position) -> Position {
    Position {
        x: pos_1.x - pos_2.x,
        y: pos_1.y - pos_2.y,
    }
}

fn main() {
    let mut tail_positions = HashSet::new();
    let mut tail_positions_2 = HashSet::new();
    let mut state = State::new(2);
    let mut state_2 = State::new(10);
    let lines: Vec<_> = io::stdin().lock().lines().map(|ln| ln.unwrap()).collect();
    let instructions = parse_instructions(&lines);

    for instruction in &instructions {
        execute_instruction(&mut state, instruction);
        execute_instruction(&mut state_2, instruction);
        tail_positions.insert(state.tail());
        tail_positions_2.insert(state_2.tail());
    }

    println!("Part 1: {}", tail_positions.len());
    println!("Part 2: {}", tail_positions_2.len());
}

#[cfg(test)]
mod test {
    use std::collections::HashSet;

    use crate::{execute_instruction, parse_instructions, State};

    #[test]
    fn test_all() {
        let lines = [
            "R 4".to_owned(),
            "U 4".to_owned(),
            "L 3".to_owned(),
            "D 1".to_owned(),
            "R 4".to_owned(),
            "D 1".to_owned(),
            "L 5".to_owned(),
            "R 2".to_owned(),
        ];
        let mut state = State::new(2);
        let mut tail_positions = HashSet::new();
        let instructions = parse_instructions(&lines);

        for instruction in instructions {
            execute_instruction(&mut state, &instruction);
            tail_positions.insert(state.tail());
        }

        assert_eq!(tail_positions.len(), 13);
    }

    #[test]
    fn test_all_nine_items() {
        let lines = [
            "R 5".to_owned(),
            "U 8".to_owned(),
            "L 8".to_owned(),
            "D 3".to_owned(),
            "R 17".to_owned(),
            "D 10".to_owned(),
            "L 25".to_owned(),
            "U 20".to_owned(),
        ];
        let mut state = State::new(10);
        let mut tail_positions = HashSet::new();
        let instructions = parse_instructions(&lines);

        for instruction in instructions {
            execute_instruction(&mut state, &instruction);
            tail_positions.insert(state.tail());
        }

        assert_eq!(tail_positions.len(), 36);
    }
}
