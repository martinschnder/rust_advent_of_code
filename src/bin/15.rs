use std::collections::HashSet;

type Input = Vec<Sensor>;

#[derive(Debug, Clone, Hash, Eq, PartialEq)]
pub struct Point {
    x: isize,
    y: isize,
}

impl Point {
    pub fn manhattan_distance(&self, other: &Point) -> isize {
        (self.x - other.x).abs() + (self.y - other.y).abs()
    }
}

// impl PartialEq for Point {
//     fn eq(&self, other: &Self) -> bool {
//         self.x == other.x && self.y == other.y
//     }
// }

pub struct Sensor {
    at: Point,
    closest_beacon: Point,
}

pub fn parse_point(s: &str) -> Option<Point> {
    let (_, point_str) = s.split_once("at")?;
    let (x_str, y_str) = point_str.split_once(", ")?;
    let (_, x) = x_str.split_once('=')?;
    let (_, y) = y_str.split_once('=')?;
    Some(Point {
        x: x.parse().ok()?,
        y: y.parse().ok()?,
    })
}

pub fn parse(input: &str) -> Input {
    input
        .lines()
        .filter_map(|line| {
            let (sensor_str, beacon_str) = line.split_once(':')?;
            Some(Sensor {
                at: parse_point(sensor_str)?,
                closest_beacon: parse_point(beacon_str)?,
            })
        })
        .collect()
}

pub fn get_lines(sensor: &Sensor) -> Vec<isize> {
    let dist = sensor.at.manhattan_distance(&sensor.closest_beacon);
    vec![
        sensor.at.y - sensor.at.x - dist,
        sensor.at.y - sensor.at.x + dist,
        sensor.at.y + sensor.at.x + dist,
        sensor.at.y + sensor.at.x - dist,
    ]
}

pub fn part_one(input: &str) -> Option<usize> {
    let input = parse(input);
    let target_row = if cfg!(test) { 10 } else { 2000000 };
    let mut beacons = HashSet::new();
    input.iter().for_each(|sensor| {
        let max_distance = sensor.at.manhattan_distance(&sensor.closest_beacon);
        let max_y = sensor.at.y + max_distance;
        let min_y = sensor.at.y - max_distance;

        if min_y <= target_row && target_row <= max_y {
            let min_x = sensor.at.x - ((sensor.at.y - target_row).abs() - max_distance).abs();
            let max_x = sensor.at.x + ((sensor.at.y - target_row).abs() - max_distance).abs();
            for x in min_x..max_x {
                beacons.insert((x, target_row));
            }
        }
    });
    Some(beacons.len())
}

pub fn is_admissible(p: Point) -> bool {
    0 <= p.x && p.x <= 4000000 && 0 <= p.y && p.y <= 4000000
}

pub fn part_two(input: &str) -> Option<isize> {
    let input = parse(input);
    let mut intersections = vec![];
    let lines: Vec<Vec<isize>> = input.iter().map(get_lines).collect();
    lines.iter().for_each(|line1| {
        lines.iter().for_each(|line2| {
            for ord1 in line1.iter().take(2) {
                for ord2 in line2.iter().take(4).skip(2) {
                    let b1 = ord1;
                    let b2 = ord2;

                    if (b1 + b2) % 2 != 0 {
                        continue;
                    }

                    let x = (b2 - b1) / 2;
                    let y = (b2 + b1) / 2;
                    if is_admissible(Point { x, y }) && !intersections.contains(&Point { x, y }) {
                        intersections.push(Point { x, y });
                    }
                }
            }
        });
    });

    let ndist2: Vec<usize> = intersections
        .iter()
        .map(|p1| {
            intersections
                .iter()
                .filter(|p2| p1 != *p2 && p1.manhattan_distance(p2) == 2)
                .count()
        })
        .collect();

    let candidates: Vec<Point> = intersections
        .iter()
        .enumerate()
        .filter(|(i, _)| ndist2[*i] >= 3)
        .map(|(_, p)| p.clone())
        .collect();

    for candidate in &candidates {
        if candidates.contains(&Point {
            x: candidate.x + 2,
            y: candidate.y,
        }) && candidates.contains(&Point {
            x: candidate.x + 1,
            y: candidate.y + 1,
        }) && candidates.contains(&Point {
            x: candidate.x + 1,
            y: candidate.y - 1,
        }) {
            return Some((candidate.x + 1) * 4000000 + candidate.y);
        }
    }
    None
}

fn main() {
    let input = &advent_of_code::read_file("inputs", 15);
    advent_of_code::solve!(1, part_one, input);
    advent_of_code::solve!(2, part_two, input);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let input = advent_of_code::read_file("examples", 15);
        assert_eq!(part_one(&input), Some(26));
    }

    #[test]
    fn test_part_two() {
        let input = advent_of_code::read_file("examples", 15);
        assert_eq!(part_two(&input), Some(40000017));
    }
}
