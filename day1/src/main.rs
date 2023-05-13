use itertools::Itertools;
use std::{cmp::Reverse, collections::BinaryHeap};

fn main() -> color_eyre::Result<()> {
    color_eyre::install()?;

    let mut group_sums = include_str!("input.txt")
        .lines()
        .map(|v| v.parse::<u64>().ok())
        .batching(|it| {
            let mut sum = None;
            while let Some(Some(v)) = it.next() {
                sum = Some(sum.unwrap_or(0) + v);
            }
            sum
        })
        .map(Reverse);

    let mut heap = BinaryHeap::new();

    for init in (&mut group_sums).take(3) {
        heap.push(init);
    }

    for rest in group_sums {
        heap.push(rest);
        heap.pop();
    }

    let answer = heap.into_iter().map(|Reverse(v)| v).sum::<u64>();

    println!("{:?}", answer);

    Ok(())
}
