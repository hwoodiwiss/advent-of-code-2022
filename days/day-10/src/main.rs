use std::io::{self, BufRead};

fn main() {
    let lines: Vec<_> = io::stdin().lock().lines().map(|ln| ln.unwrap()).collect();
    let instructions = parse_instructions(&lines);

    for instruction in &instructions {
        execute_instruction(&mut state, instruction);
        execute_instruction(&mut state_2, instruction);
        tail_positions.insert(state.tail());
        tail_positions_2.insert(state_2.tail());
    }

    println!("Part 1: {}", "");
    println!("Part 2: {}", "");
}

#[cfg(test)]
mod test {}
