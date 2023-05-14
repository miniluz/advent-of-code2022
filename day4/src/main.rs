use std::ops::RangeInclusive;

use itertools::Itertools;

trait InclusiveRangeExt {
    fn contains_range(&self, other: &Self) -> bool;

    fn contains_or_is_contained(&self, other: &Self) -> bool {
        self.contains_range(other) || other.contains_range(self)
    }

    fn has_overlap(&self, other: &Self) -> bool;
}

impl<T> InclusiveRangeExt for RangeInclusive<T>
where
    T: PartialOrd,
{
    fn contains_range(&self, other: &RangeInclusive<T>) -> bool {
        self.contains(other.start()) && self.contains(other.end())
    }
    fn has_overlap(&self, other: &Self) -> bool {
        self.contains(other.start())
            || self.contains(other.end())
            || other.contains(self.start())
            || other.contains(self.end())
    }
}

fn main() {
    let count = include_str!("input.txt")
        .lines()
        .map(|l| {
            l.split(',')
                .map(|range| {
                    range
                        .split('-')
                        .map(|n| n.parse().expect("should be u32"))
                        .collect_tuple::<(u32, u32)>()
                        .map(|(start, end)| start..=end)
                        .expect("each range must have a start and end!")
                })
                .collect_tuple::<(_, _)>()
                .expect("each line must have a pair of ranges!")
        })
        .filter(|(a, b)| a.has_overlap(b))
        .count();
    dbg!(count);
}
