use itertools::Itertools;
use std::{
    cmp::Ordering,
    collections::{BTreeSet, BinaryHeap, HashMap, HashSet, VecDeque},
};

#[derive(Eq, PartialEq)]
pub struct Valve<'a> {
    flow: u32,
    neighbours: HashSet<&'a str>,
}

#[derive(Eq, PartialEq)]
pub struct Node<'a> {
    cost: u32,
    curr: &'a str,
}

#[derive(Debug, Hash, PartialEq, Eq, Clone)]
pub struct State<'a> {
    opened: BTreeSet<&'a str>,
    curr: &'a str,
    elapsed: u32,
    relieved: u32,
}

impl<'a> Ord for Node<'a> {
    fn cmp(&self, other: &Node) -> Ordering {
        other.cost.cmp(&self.cost)
    }
}

impl<'a> PartialOrd for Node<'a> {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

fn min_cost(from: &str, to: &str, map: &HashMap<&str, Valve>) -> u32 {
    let mut pq = BinaryHeap::new();
    let mut seen = HashSet::new();

    pq.push(Node {
        cost: 0,
        curr: from,
    });
    seen.insert(from);

    while let Some(Node { cost, curr }) = pq.pop() {
        if curr == to {
            return cost;
        }
        for neighbour in map[curr].neighbours.iter() {
            if seen.insert(neighbour) {
                pq.push(Node {
                    cost: cost + 1,
                    curr: neighbour,
                })
            }
        }
    }

    u32::MAX
}

pub fn parse(input: &str) -> HashMap<&str, Valve> {
    input
        .lines()
        .map(|line| {
            let (valve, neighbours) = line.split_once("; ").unwrap();
            let valve = valve.strip_prefix("Valve ").unwrap();
            let (name, flow) = valve.split_once(" has flow rate=").unwrap();
            let flow = flow.parse().unwrap();
            let neighbours = neighbours
                .strip_prefix("tunnels lead to valves ")
                .or_else(|| neighbours.strip_prefix("tunnel leads to valve "))
                .unwrap();
            let neighbours = neighbours.split_terminator(", ").collect();
            (name, Valve { flow, neighbours })
        })
        .collect()
}

pub fn min_distance<'a>(map: &'a HashMap<&str, Valve>) -> HashMap<(&'a str, &'a str), u32> {
    map.iter()
        .filter(|(_, valve)| valve.flow > 0)
        .map(|(name, _)| name)
        .tuple_combinations()
        .fold(HashMap::new(), |mut acc, (name1, name2)| {
            acc.entry(("AA", name1))
                .or_insert_with(|| min_cost("AA", name1, map));

            acc.entry(("AA", name2))
                .or_insert_with(|| min_cost("AA", name2, map));

            let dist = min_cost(name1, name2, map);

            acc.insert((name1, name2), dist);
            acc.insert((name2, name1), dist);

            acc
        })
}

pub fn wait_until_ending(
    max_time: u32,
    elapsed: u32,
    relieved: u32,
    opened: &BTreeSet<&str>,
    map: &HashMap<&str, Valve>,
) -> u32 {
    let time_left = max_time - elapsed;
    let relieved_per_min: u32 = opened.iter().map(|name| map[name].flow).sum();
    relieved + time_left * relieved_per_min
}
pub fn part_one(input: &str) -> Option<u32> {
    let map = parse(input);
    let dist_map = min_distance(&map);
    let flowing: HashSet<_> = map
        .iter()
        .filter(|(_, valve)| valve.flow > 0)
        .map(|(&name, _)| name)
        .collect();

    let mut max_relieved = 0;
    let mut q = VecDeque::new();
    let mut seen = HashSet::new();

    q.push_back(State {
        curr: "AA",
        opened: BTreeSet::new(),
        elapsed: 0,
        relieved: 0,
    });

    seen.insert((BTreeSet::new(), 0, 0));

    while let Some(State {
        curr,
        opened,
        elapsed,
        relieved,
    }) = q.pop_front()
    {
        if opened.len() == flowing.len() || elapsed >= 30 {
            let relieved_at_end = wait_until_ending(30, elapsed, relieved, &opened, &map);
            max_relieved = max_relieved.max(relieved_at_end);
            continue;
        }

        let unopened = flowing.iter().filter(|name| !opened.contains(*name));

        for dest in unopened {
            let cost = dist_map[&(curr, *dest)] + 1;
            let new_elapsed = elapsed + cost;
            if new_elapsed >= 30 {
                let relieved_at_end = wait_until_ending(30, elapsed, relieved, &opened, &map);
                max_relieved = max_relieved.max(relieved_at_end);
                continue;
            }

            let relieved_per_min: u32 = opened.iter().map(|name| map[name].flow).sum();
            let new_relieved = relieved + relieved_per_min * cost;

            let mut new_opened = opened.clone();
            new_opened.insert(dest);

            if seen.insert((new_opened.clone(), new_elapsed, new_relieved)) {
                q.push_back(State {
                    curr: dest,
                    opened: new_opened,
                    elapsed: new_elapsed,
                    relieved: new_relieved,
                })
            }
        }
    }
    Some(max_relieved)
}

pub fn part_two(input: &str) -> Option<u32> {
    let map = parse(input);
    let dist_map = min_distance(&map);
    let flowing: HashSet<_> = map
        .iter()
        .filter(|(_, valve)| valve.flow > 0)
        .map(|(&name, _)| name)
        .collect();

    // let mut max_relieved = 0;
    let mut q = VecDeque::new();
    let mut seen = HashSet::new();

    let mut max_relieves_states: HashMap<BTreeSet<&str>, u32> = HashMap::new();

    q.push_back(State {
        curr: "AA",
        opened: BTreeSet::new(),
        elapsed: 0,
        relieved: 0,
    });

    seen.insert((BTreeSet::new(), 0, 0));

    while let Some(State {
        curr,
        opened,
        elapsed,
        relieved,
    }) = q.pop_front()
    {
        let relieved_at_end = wait_until_ending(26, elapsed, relieved, &opened, &map);
        max_relieves_states
            .entry(opened.clone())
            .and_modify(|val| *val = relieved_at_end.max(*val))
            .or_insert(relieved_at_end);

        if opened.len() == flowing.len() || elapsed >= 26 {
            continue;
        }

        let unopened = flowing.iter().filter(|name| !opened.contains(*name));

        for dest in unopened {
            let cost = dist_map[&(curr, *dest)] + 1;
            let new_elapsed = elapsed + cost;
            if new_elapsed >= 26 {
                continue;
            }

            let relieved_per_min: u32 = opened.iter().map(|name| map[name].flow).sum();
            let new_relieved = relieved + relieved_per_min * cost;

            let mut new_opened = opened.clone();
            new_opened.insert(dest);

            if seen.insert((new_opened.clone(), new_elapsed, new_relieved)) {
                q.push_back(State {
                    curr: dest,
                    opened: new_opened,
                    elapsed: new_elapsed,
                    relieved: new_relieved,
                })
            }
        }
    }
    Some(
        max_relieves_states
            .iter()
            .tuple_combinations()
            .filter(|(human, elephant)| human.0.is_disjoint(elephant.0))
            .map(|(human, elephant)| human.1 + elephant.1)
            .max()
            .unwrap(),
    )
}

fn main() {
    let input = &advent_of_code::read_file("inputs", 16);
    advent_of_code::solve!(1, part_one, input);
    advent_of_code::solve!(2, part_two, input);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let input = advent_of_code::read_file("examples", 16);
        assert_eq!(part_one(&input), Some(1651));
    }

    #[test]
    fn test_part_two() {
        let input = advent_of_code::read_file("examples", 16);
        assert_eq!(part_two(&input), Some(1707));
    }
}
