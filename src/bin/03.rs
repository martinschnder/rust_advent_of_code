pub fn priority(char: char) -> u32 {
    match char.is_lowercase() {
        true => char as u32 - 96,
        false => char as u32 - 38,
    }
}
pub fn part_one(input: &str) -> Option<u32> {
    Some(
        input
            .lines()
            .filter_map(|line| {
                let parts = line.split_at(line.len() / 2);
                let a = parts.0;
                let b = parts.1;
                a.chars()
                    .find(|char| b.contains(*char))
                    .map(|char| priority(char))
            })
            .sum(),
    )
}

pub fn part_two(input: &str) -> Option<u32> {
    Some(
        input
            .lines()
            .collect::<Vec<&str>>()
            .chunks(3)
            .filter_map(|chunk| {
                let mut chunks = chunk.iter();
                let a = chunks.next()?;
                let b = chunks.next()?;
                let c = chunks.next()?;
                a.chars()
                    .find(|char| b.contains(*char) && c.contains(*char))
                    .map(|char| priority(char))
            })
            .sum()
    )
}

fn main() {
    let input = &advent_of_code::read_file("inputs", 3);
    advent_of_code::solve!(1, part_one, input);
    advent_of_code::solve!(2, part_two, input);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let input = advent_of_code::read_file("examples", 3);
        assert_eq!(part_one(&input), Some(157));
    }

    #[test]
    fn test_part_two() {
        let input = advent_of_code::read_file("examples", 3);
        assert_eq!(part_two(&input), Some(70));
    }
}
