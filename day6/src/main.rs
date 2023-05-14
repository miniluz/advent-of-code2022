use std::collections::VecDeque;

fn matches<T>(vector: &VecDeque<T>) -> bool
where
    T: std::cmp::Eq + std::hash::Hash,
{
    use std::collections::HashSet;
    vector.iter().collect::<HashSet<_>>().len() != vector.len()
}

fn main() -> color_eyre::Result<()> {
    let input = include_str!("input.txt");

    let buffer_size: usize = 14;
    let (first, second) = input.split_at(buffer_size - 1);

    let mut buffer: VecDeque<_> = first.chars().take(buffer_size - 1).collect();

    let answer = second
        .chars()
        .enumerate()
        .filter_map(|(i, char)| {
            buffer.push_back(char);
            let i = match matches(&buffer) {
                true => None,
                false => Some(i + buffer_size),
            };
            buffer.pop_front();
            i
        })
        .next()
        .ok_or(color_eyre::eyre::eyre!("Should have a result"))?;

    println!("{answer}");

    Ok(())
}
