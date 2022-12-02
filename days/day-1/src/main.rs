use std::io::{self, BufRead};

fn main() {
    let lines: Vec<_> = io::stdin().lock().lines().map(|ln| ln.unwrap()).collect();
    let mut elves: Vec<i32> = lines
        .split(|ln| ln.is_empty())
        .map(|elf| {
            elf.into_iter()
                .map(|m| m.trim().parse::<i32>().unwrap())
                .sum::<i32>()
        })
        .collect();
    elves.sort();
    let end = elves.len() - 1;

    println!("Part 1: {}", elves[end]);
    println!("Part 2: {}", elves[end - 2..].into_iter().sum::<i32>());
}
