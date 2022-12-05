use std::{
    collections::HashMap,
    io::{self, BufRead},
};

fn main() {
    let lines: Vec<_> = io::stdin().lock().lines().map(|ln| ln.unwrap()).collect();
    let score_choice: i32 = lines.iter().map(|ln| rock_paper_scissors_choice(ln)).sum();
    let score_outcome: i32 = lines.iter().map(|ln| rock_paper_scissors_outcome(ln)).sum();

    println!("Part 1: {}", score_choice);
    println!("Part 2: {}", score_outcome);
}

/// Plays rock, paper, scissors and returns the score.
fn rock_paper_scissors_choice(line: &str) -> i32 {
    let scores = HashMap::from([('X', 1i32), ('Y', 2i32), ('Z', 3i32)]);
    let map = HashMap::from([
        ('A', ('X', 'Y', 'Z')),
        ('B', ('Y', 'Z', 'X')),
        ('C', ('Z', 'X', 'Y')),
    ]);
    let choices: Vec<_> = line
        .split(' ')
        .map(|item| item.chars().next().unwrap())
        .collect();
    let opponent = map.get(&choices[0]).unwrap();

    let mut score = *scores.get(&choices[1]).unwrap();
    assert!(score != 0);
    score += match choices[1] {
        choice if choice == opponent.0 => 3,
        choice if choice == opponent.1 => 6,
        choice if choice == opponent.2 => 0,
        _ => unreachable!(),
    };

    score
}

/// Plays rock, paper, scissors and returns the score.
fn rock_paper_scissors_outcome(line: &str) -> i32 {
    let scores = HashMap::from([('X', 0i32), ('Y', 3i32), ('Z', 6i32)]);
    let map = HashMap::from([('A', (1, 2, 3)), ('B', (2, 3, 1)), ('C', (3, 1, 2))]);
    let choices: Vec<_> = line
        .split(' ')
        .map(|item| item.chars().next().unwrap())
        .collect();
    let opponent = map.get(&choices[0]).unwrap();

    let mut score = *scores.get(&choices[1]).unwrap();

    score += match choices[1] {
        choice if choice == 'X' => opponent.2,
        choice if choice == 'Y' => opponent.0,
        choice if choice == 'Z' => opponent.1,
        _ => unreachable!(),
    };

    score
}
