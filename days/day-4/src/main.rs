use std::io::{self, BufRead};

fn main() {
    let lines: Vec<_> = io::stdin().lock().lines().map(|ln| ln.unwrap()).collect();
    let part_1: i32 = lines.iter().map(|ln| get_count_of_contained(ln)).sum();
    let part_2: i32 = lines.iter().map(|ln| get_count_of_overlapping(ln)).sum();

    println!("Part 1: {}", part_1);
    println!("Part 2: {}", part_2);
}

fn get_count_of_contained(line: &str) -> i32 {
    let grp_separator = line.find(',').unwrap();
    let section_grp_1 = get_section_params(&line[..grp_separator]);
    let section_grp_2 = get_section_params(&line[grp_separator + 1..]);

    if section_grp_1.0 == section_grp_2.0 || section_grp_1.1 == section_grp_2.1 {
        return 1;
    }

    if section_grp_1.0 > section_grp_2.0 {
        i32::from(section_grp_1.1 < section_grp_2.1)
    } else {
        i32::from(section_grp_2.1 < section_grp_1.1)
    }
}

// Similar to 2d AABB intersection
fn get_count_of_overlapping(line: &str) -> i32 {
    let grp_separator = line.find(',').unwrap();
    let section_grp_1 = get_section_params(&line[..grp_separator]);
    let section_grp_2 = get_section_params(&line[grp_separator + 1..]);

    i32::from(section_grp_1.0.max(section_grp_2.0) <= section_grp_1.1.min(section_grp_2.1))
}

fn get_section_params(section: &str) -> (i32, i32) {
    let range_separator = section
        .find('-')
        .unwrap_or_else(|| panic!("Failed to find range indicator for {}", section));
    let begin = &section[..range_separator]
        .parse::<i32>()
        .unwrap_or_else(|e| {
            panic!(
                "Failed to parse {} as i32 with error {}",
                &section[..range_separator],
                e
            )
        });
    let end = &section[range_separator + 1..]
        .parse::<i32>()
        .unwrap_or_else(|e| {
            panic!(
                "Failed to parse {} as i32 with error {}",
                &section[range_separator + 1..],
                e
            )
        });
    (*begin, *end)
}

#[cfg(test)]
mod test {
    use crate::{get_count_of_contained, get_count_of_overlapping};
    use test_case::test_case;

    #[test_case("2-4,6-8", 0)]
    #[test_case("2-3,4-5", 0)]
    #[test_case("5-7,7-9", 0)]
    #[test_case("2-8,3-7", 1)]
    #[test_case("6-6,4-6", 1)]
    #[test_case("2-6,4-8", 0)]
    #[test_case("1-2,1-2", 1)]
    #[test_case("1-3,1-2", 1)]
    #[test_case("1-2,1-3", 1)]
    #[test_case("1-3,2-3", 1)]
    #[test_case("2-3,1-3", 1)]
    fn test_get_count_of_contained(line: &str, expect: i32) {
        let actual = get_count_of_contained(&line.to_owned());
        assert!(actual == expect);
    }

    #[test_case("2-4,6-8", 0)]
    #[test_case("2-3,4-5", 0)]
    #[test_case("5-7,7-9", 1)]
    #[test_case("2-8,3-7", 1)]
    #[test_case("6-6,4-6", 1)]
    #[test_case("2-6,4-8", 1)]
    fn test_get_count_of_overlapping(line: &str, expect: i32) {
        let actual = get_count_of_overlapping(&line.to_owned());
        assert!(actual == expect);
    }
}
