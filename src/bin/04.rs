use std::{cmp, ops::Range};

type Input = Vec<(Range<u32>, Range<u32>)>;

pub fn parse(input: &str) -> Input {
    input
        .lines()
        .filter_map(|line| {
            let (a, b) = line.split_once(',')?;
            let (a_start, a_end) = a.split_once('-')?;
            let (b_start, b_end) = b.split_once('-')?;
            Some((
                (a_start.parse().ok()?)..(a_end.parse().ok()?),
                (b_start.parse().ok()?)..(b_end.parse().ok()?),
            ))
        })
        .collect()
}

pub fn envelops(a: &Range<u32>, b: &Range<u32>) -> bool {
    a.start <= b.start && a.end >= b.end || b.start <= a.start && b.end >= a.end
}

pub fn overlaps(a: &Range<u32>, b: &Range<u32>) -> bool {
    cmp::max(a.start, b.start) <= cmp::min(a.end, b.end)
}

pub fn part_one(input: &str) -> Option<usize> {
    let parsed_input = parse(input);
    Some(
        parsed_input
            .iter()
            .filter(|(a, b)| envelops(a, b) || envelops(b, a))
            .count(),
    )
}

pub fn part_two(input: &str) -> Option<usize> {
    let parsed_input = parse(input);
    Some(
        parsed_input
            .iter()
            .filter(|(a, b)| overlaps(a, b))
            .count(),
    )
}

fn main() {
    let input = &advent_of_code::read_file("inputs", 4);
    advent_of_code::solve!(1, part_one, input);
    advent_of_code::solve!(2, part_two, input);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let input = advent_of_code::read_file("examples", 4);
        assert_eq!(part_one(&input), Some(2));
    }

    #[test]
    fn test_part_two() {
        let input = advent_of_code::read_file("examples", 4);
        assert_eq!(part_two(&input), Some(4));
    }
}
