type Stack = Vec<Vec<char>>;
type Move = (usize, usize, usize);
type Input = (Stack, Vec<Move>);

pub fn parse(input: &str) -> Input {
    let (stack_str, move_str) = input.split_once("\n\n").unwrap();

    let mut stack_iter = stack_str.lines().rev();
    let mut stack = vec![vec![]; stack_iter.next().unwrap().len() / 4 + 1];

    stack_iter.for_each(|line| {
        line.chars().skip(1).enumerate().for_each(|(index, value)| {
            if index % 4 == 0 && value != ' ' {
                stack[index / 4].push(value);
            }
        });
    });

    let moves = move_str
        .lines()
        .filter_map(|line| {
            let s: Vec<&str> = line.split_ascii_whitespace().collect();
            Some((s[1].parse().ok()?, s[3].parse().ok()?, s[5].parse().ok()?))
        })
        .collect();
    (stack, moves)
}

pub fn move_stacks(stack: &mut Stack, moves: &[Move], part: i8) {
    moves.iter().for_each(|(qty, from, to)| {
        let from = &mut stack[*from - 1];
        let vec = from.split_off(from.len() - qty);
        if part == 1 {
            vec.iter().rev().for_each(|item| stack[to - 1].push(*item));
        } else {
            vec.iter().for_each(|item| stack[to - 1].push(*item));
        }
    })
}

pub fn get_top_row(stack: &Stack) -> String {
    let mut result = vec![];
    for s in stack {
        if !s.is_empty() {
            result.push(*s.last().unwrap());
        }
    }
    result.iter().collect::<String>()
}

pub fn part_one(input: &str) -> Option<String> {
    let (mut stack, moves) = parse(input);
    move_stacks(&mut stack, &moves, 1);
    Some(get_top_row(&stack))
}

pub fn part_two(input: &str) -> Option<String> {
    let (mut stack, moves) = parse(input);
    move_stacks(&mut stack, &moves, 2);
    Some(get_top_row(&stack))
}

fn main() {
    let input = &advent_of_code::read_file("inputs", 5);
    advent_of_code::solve!(1, part_one, input);
    advent_of_code::solve!(2, part_two, input);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let input = advent_of_code::read_file("examples", 5);
        assert_eq!(part_one(&input), Some("CMZ".into()));
    }

    #[test]
    fn test_part_two() {
        let input = advent_of_code::read_file("examples", 5);
        assert_eq!(part_two(&input), Some("MCD".into()));
    }
}
