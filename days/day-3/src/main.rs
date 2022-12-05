use std::{
    collections::HashSet,
    io::{self, BufRead},
};

fn main() {
    print_all_priorities();
    let lines: Vec<_> = io::stdin().lock().lines().map(|ln| ln.unwrap()).collect();
    let part1: i32 = lines.iter().map(|ln| get_invalid_priorities(ln)).sum();
    let part2: i32 = lines.chunks(3).map(get_badge_item).sum();
    println!("Part 1: {}", part1);
    println!("Part 2: {}", part2);
}

fn get_invalid_priorities(line: &str) -> i32 {
    let mut items = HashSet::new();
    let mut found = HashSet::new();
    let mid = line.len() / 2;

    line[..mid].bytes().for_each(|c| {
        let _ = &items.insert(c);
    });
    line[mid..]
        .bytes()
        .map(|c: u8| {
            if items.contains(&c) && !found.contains(&c) {
                let _ = &found.insert(c);
                get_char_priority(&c)
            } else {
                0i32
            }
        })
        .sum::<i32>()
}

fn get_badge_item(lines: &[String]) -> i32 {
    let mut found_1 = HashSet::new();
    let mut found_2 = HashSet::new();
    let mut found_all = HashSet::new();
    lines[0].bytes().for_each(|ln| {
        let _ = &found_1.insert(ln);
    });

    lines[1].bytes().for_each(|ln| {
        if found_1.contains(&ln) {
            let _ = found_2.insert(ln);
        }
    });

    lines[2].bytes().for_each(|ln| {
        if found_2.contains(&ln) {
            let _ = found_all.insert(ln);
        }
    });

    println!("{:?}", found_all);

    found_all.iter().map(get_char_priority).sum::<i32>()
}

fn get_char_priority(character: &u8) -> i32 {
    if *character >= 97 {
        *character as i32 - 96i32
    } else {
        *character as i32 - 38i32
    }
}

fn print_all_priorities() {
    println!("Lowercase");
    (97u8..=122u8)
        .map(|i| get_char_priority(&i))
        .for_each(|e| println!("{}", e));
    println!("Uppercase");
    (65u8..=90u8)
        .map(|i| get_char_priority(&i))
        .for_each(|e| println!("{}", e));
}
