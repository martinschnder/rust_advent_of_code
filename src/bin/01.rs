use advent_of_code::solve;

fn part_one(input: &str) -> Option<i32> {
    let calories_count = input.split("\n\n").map(|elf| {
        elf.lines()
            .filter_map(|food| food.parse::<i32>().ok())
            .sum()
    });
    calories_count.max()
}

fn part_two(input: &str) -> Option<i32> {
    let mut calories_count: Vec<i32> = input
        .split("\n\n")
        .map(|elf| {
            elf.lines()
                .filter_map(|food| food.parse::<i32>().ok())
                .sum()
        })
        .collect();

    calories_count.sort_by(|a, b| b.cmp(a));
    Some(calories_count[0..3].iter().sum())
}

fn main() {
    let input = &advent_of_code::read_file("inputs", 1);
    solve!(1, part_one, input);
    solve!(2, part_two, input);
}
