use std::collections::HashSet;

#[cfg(test)]
mod tests {
    use crate::find_marker;
    use test_case::test_case;

    #[test_case(7, "mjqjpqmgbljsphdztnvjfqwrcgsmlb", 4)]
    #[test_case(19, "mjqjpqmgbljsphdztnvjfqwrcgsmlb", 14)]
    #[test_case(5, "bvwbjplbgvbhsrlpgdmjqwftvncz", 4)]
    #[test_case(23, "bvwbjplbgvbhsrlpgdmjqwftvncz", 14)]
    #[test_case(10, "nznrnfrfntjfmvfwmzdfjlvtqnbhcprsg", 4)]
    #[test_case(29, "nznrnfrfntjfmvfwmzdfjlvtqnbhcprsg", 14)]
    #[test_case(11, "zcfzfwzzqfrljwzlrfnpqdbhtmscgvjw", 4)]
    #[test_case(26, "zcfzfwzzqfrljwzlrfnpqdbhtmscgvjw", 14)]
    fn test_find_marker(index: usize, input: &str, buffer_size: usize) {
        assert_eq!(Some(index), find_marker(input, buffer_size))
    }
}

fn find_marker(input: &str, buffer_size: usize) -> Option<usize> {
    input
        .as_bytes()
        .windows(buffer_size)
        .position(|window| window.iter().collect::<HashSet<_>>().len() == buffer_size)
        .map(|n| n + buffer_size)
}

fn main() -> color_eyre::Result<()> {
    let answer = find_marker(include_str!("input.txt"), 4);
    dbg!(answer);

    let answer = find_marker(include_str!("input.txt"), 14);
    dbg!(answer);

    Ok(())
}
