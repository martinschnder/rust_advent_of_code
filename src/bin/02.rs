pub fn score(other: i32, me: i32) -> i32 {
    let score = (3 - (2 + other - me) % 3) % 3 * 3;
    score + me + 1
}

pub fn part_one(input: &str) -> Option<i32> {
    let mut total: i32 = 0;
    for line in input.lines() {
        total += score(line.chars().next().unwrap() as i32 - 65, line.chars().nth(2).unwrap() as i32 - 88);
    }
    Some(total)
}

pub fn part_two(input: &str) -> Option<i32> {
    let mut total = 0;
    for line in input.lines() {
        let other = line.chars().nth(0).unwrap() as i32 - 65;
        let result = line.chars().nth(2).unwrap() as i32 - 88;
        let me = match result {
            0 => (other + 2) % 3,
            1 => other,
            2 => (other + 1) % 3,
            _ => unreachable!(),
        };
        total += score(other, me);
    }
    Some(total)
}

fn main() {
    let input = &advent_of_code::read_file("inputs", 2);
    advent_of_code::solve!(1, part_one, input);
    advent_of_code::solve!(2, part_two, input);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let input = advent_of_code::read_file("examples", 2);
        assert_eq!(part_one(&input), Some(15));
    }

    #[test]
    fn test_part_two() {
        let input = advent_of_code::read_file("examples", 2);
        assert_eq!(part_two(&input), Some(12));
    }
}
