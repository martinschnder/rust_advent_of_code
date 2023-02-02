use std::{collections::HashMap, path::PathBuf};

pub fn part_one(input: &str) -> Option<u32> {
    let mut sizes = HashMap::new();
    let mut affected = Vec::new();

    for line in input.lines() {
        if line.starts_with("$ ls") || line.starts_with("dir") {
            continue;
        }

        let parts: Vec<_> = line.split_whitespace().collect();
        match parts[..] {
            ["$", "cd", ".."] => {
                affected.pop();
            }
            ["$", "cd", name] => {
                affected.push(name);
            }
            [size, _name] => {
                let size: u32 = size.parse().unwrap();
                for idx in 0..affected.len() {
                    let path = PathBuf::from_iter(&affected[0..=idx]);
                    *sizes.entry(path).or_insert(0) += size;
                }
            }
            _ => panic!("unexpected line"),
        };
    }

    Some(sizes.into_values().filter(|size| *size < 100_000).sum())
}

pub fn part_two(input: &str) -> Option<u32> {
    let mut sizes = HashMap::new();
    let mut affected = Vec::new();

    for line in input.lines() {
        if line.starts_with("$ ls") || line.starts_with("dir") {
            continue;
        }

        let parts: Vec<_> = line.split_whitespace().collect();
        match parts[..] {
            ["$", "cd", ".."] => {
                affected.pop();
            }
            ["$", "cd", name] => {
                affected.push(name);
            }
            [size, _name] => {
                let size: u32 = size.parse().unwrap();
                for idx in 0..affected.len() {
                    let path = PathBuf::from_iter(&affected[0..=idx]);
                    *sizes.entry(path).or_insert(0) += size;
                }
            }
            _ => panic!("unexpected line"),
        };
    }

    let disk = 70_000_000;
    let needed = 30_000_000;
    let root = sizes.get(&PathBuf::from("/")).unwrap();
    let available = disk - root;

    sizes
        .into_values()
        .filter(|size| *size >= needed - available)
        .min()
}

fn main() {
    let input = &advent_of_code::read_file("inputs", 7);
    advent_of_code::solve!(1, part_one, input);
    advent_of_code::solve!(2, part_two, input);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let input = advent_of_code::read_file("examples", 7);
        assert_eq!(part_one(&input), Some(95437));
    }

    #[test]
    fn test_part_two() {
        let input = advent_of_code::read_file("examples", 7);
        assert_eq!(part_two(&input), Some(24933642));
    }
}
