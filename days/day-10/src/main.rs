use std::{
    fmt::Display,
    io::{self, BufRead},
};

enum Instruction {
    Noop,
    AddxLoad,
    Addx(i32),
}

fn parse_instructions(lines: &[String]) -> Vec<Instruction> {
    lines
        .iter()
        .flat_map(|m| {
            let parts = m.trim().split(' ').collect::<Vec<_>>();
            match parts[0] {
                "noop" => Vec::from([Instruction::Noop]),
                "addx" => Vec::from([
                    Instruction::AddxLoad,
                    Instruction::Addx(parts[1].parse().unwrap()),
                ]),
                _ => panic!(""),
            }
        })
        .collect()
}

struct State {
    register_x: i32,
    clock: i32,
}

impl State {
    fn new() -> Self {
        Self {
            register_x: 1,
            clock: 1,
        }
    }

    fn tick(&mut self, instr: Instruction) {
        match instr {
            Instruction::Addx(val) => self.register_x += val,
            _ => (),
        }

        self.clock += 1
    }
}

struct Crt {
    curr_index: usize,
    pixels: [[char; 40]; 6],
}

impl Crt {
    fn new() -> Self {
        Self {
            curr_index: 0,
            pixels: [[' '; 40]; 6],
        }
    }

    fn tick(&mut self, candidates: &[i32; 3]) {
        self.curr_index += 1;
        let row = self.curr_index / 40;
        let col = self.curr_index % 40;

        if candidates.contains(&(col as i32)) {
            self.pixels[row][col] = '#';
        }
    }

    fn active_pos(&mut self, index: &usize, reg_value: &i32) {
        let row = (index / 40);
        self.pixels[row][(reg_value - 1) as usize] = '#';
        self.pixels[row][*reg_value as usize] = '#';
        if reg_value + 1 < 40 {
            self.pixels[row][(reg_value + 1) as usize] = '#';
        }
    }
}

impl Display for Crt {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}\n", self.pixels[0].into_iter().collect::<String>());
        write!(f, "{}\n", self.pixels[1].into_iter().collect::<String>());
        write!(f, "{}\n", self.pixels[2].into_iter().collect::<String>());
        write!(f, "{}\n", self.pixels[3].into_iter().collect::<String>());
        write!(f, "{}\n", self.pixels[4].into_iter().collect::<String>());
        write!(f, "{}\n", self.pixels[5].into_iter().collect::<String>())
    }
}

fn main() {
    let lines: Vec<_> = io::stdin().lock().lines().map(|ln| ln.unwrap()).collect();
    let instructions = parse_instructions(&lines);
    let mut state = State::new();
    let mut crt = Crt::new();
    let snapshot_cycle = [20, 60, 100, 140, 180, 220];
    let mut snapshots = Vec::new();

    for (i, instr) in instructions.into_iter().enumerate() {
        state.tick(instr);

        crt.tick(&[state.register_x - 1, state.register_x, state.register_x + 1]);
        if snapshot_cycle.contains(&(i + 2)) {
            snapshots.push(state.register_x * state.clock);
        }
    }

    println!("Part 1: {}", snapshots.iter().sum::<i32>());
    println!("Part 2: \n{}", crt);
}

#[cfg(test)]
mod test {
    use crate::{parse_instructions, State};

    #[test]
    fn test_run_instructions() {
        let file_content = include_str!("../input_test.txt");
        let file_lines: Vec<String> = file_content
            .split('\n')
            .into_iter()
            .map(|m| m.to_owned())
            .collect();
        let instructions = parse_instructions(&file_lines);
        let mut state = State::new();
        let snapshot_cycle = [20, 60, 100, 140, 180, 220];
        let mut snapshots = Vec::new();

        for (i, instr) in instructions.into_iter().enumerate() {
            state.tick(instr);
            if snapshot_cycle.contains(&(i + 2)) {
                snapshots.push(state.register_x * state.clock);
            }
        }

        assert_eq!(snapshots.iter().sum::<i32>(), 13140);
    }
}
