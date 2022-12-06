use std::{
    collections::HashSet,
    io::{self, Read},
};

fn main() {
    let mut input_buf = Vec::new();
    let _: usize = io::stdin().lock().read_to_end(&mut input_buf).unwrap();

    let part_1 = find_message_start(&input_buf, 4);
    let part_2 = find_message_start(&input_buf, 14);
    println!("Part 1: {}", part_1);
    println!("Part 2: {}", part_2);
}

fn find_message_start(input: &[u8], marker_size: usize) -> usize {
    let mut window_start = 0usize;
    let mut message_start = 0usize;

    while message_start == 0usize {
        let mut curr_items = HashSet::new();
        for (index, item) in input[window_start..window_start + marker_size]
            .iter()
            .rev()
            .enumerate()
        {
            if !&curr_items.insert(*item) {
                window_start = window_start + marker_size - index;
                let _ = &curr_items.clear();
                break;
            }
        }

        if curr_items.len() != marker_size {
            continue;
        }

        message_start = window_start + marker_size
    }

    message_start
}

#[cfg(test)]
mod test {
    use test_case::test_case;

    use crate::find_message_start;

    #[test_case(b"bvwbjplbgvbhsrlpgdmjqwftvncz", 4, 5)]
    #[test_case(b"nppdvjthqldpwncqszvftbrmjlhg", 4, 6)]
    #[test_case(b"nznrnfrfntjfmvfwmzdfjlvtqnbhcprsg", 4, 10)]
    #[test_case(b"zcfzfwzzqfrljwzlrfnpqdbhtmscgvjw", 4, 11)]
    #[test_case(b"mjqjpqmgbljsphdztnvjfqwrcgsmlb", 14, 19)]
    #[test_case(b"bvwbjplbgvbhsrlpgdmjqwftvncz", 14, 23)]
    #[test_case(b"nppdvjthqldpwncqszvftbrmjlhg", 14, 23)]
    #[test_case(b"nznrnfrfntjfmvfwmzdfjlvtqnbhcprsg", 14, 29)]
    #[test_case(b"zcfzfwzzqfrljwzlrfnpqdbhtmscgvjw", 14, 26)]
    fn test_find_message_start(buffer: &[u8], marker_size: usize, expected_start: usize) {
        let actual_start = find_message_start(buffer, marker_size);

        assert_eq!(actual_start, expected_start);
    }
}
