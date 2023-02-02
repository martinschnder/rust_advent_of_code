use std::{collections::HashMap, fmt::Display};

pub enum Jet {
    Left,
    Right,
}

#[derive(Default)]
pub struct Coord {
    x: usize,
    y: usize,
}

const WIDTH: usize = 7;
const PIECES: [&[Coord]; 5] = [
    // hozizontal line
    &[
        Coord { x: 0, y: 0 },
        Coord { x: 1, y: 0 },
        Coord { x: 2, y: 0 },
        Coord { x: 3, y: 0 },
    ],
    // plus
    &[
        Coord { x: 0, y: 1 },
        Coord { x: 1, y: 0 },
        Coord { x: 1, y: 1 },
        Coord { x: 1, y: 2 },
        Coord { x: 2, y: 1 },
    ],
    // J (or backwards L)
    &[
        Coord { x: 0, y: 0 },
        Coord { x: 1, y: 0 },
        Coord { x: 2, y: 0 },
        Coord { x: 2, y: 1 },
        Coord { x: 2, y: 2 },
    ],
    // vertical line
    &[
        Coord { x: 0, y: 0 },
        Coord { x: 0, y: 1 },
        Coord { x: 0, y: 2 },
        Coord { x: 0, y: 3 },
    ],
    // square
    &[
        Coord { x: 0, y: 0 },
        Coord { x: 1, y: 0 },
        Coord { x: 0, y: 1 },
        Coord { x: 1, y: 1 },
    ],
];

#[derive(Default)]
pub struct State {
    jet_count: usize,
    piece_count: usize,
    top: usize,
    map: Vec<[bool; WIDTH]>,
    curr: Coord,
    seen: HashMap<(usize, usize), (usize, usize, usize)>,
    added_by_repeat: usize,
}

impl State {
    pub fn is_valid(&mut self, new_curr: &Coord, piece: &[Coord]) -> bool {
        piece.iter().all(|offset| {
            let x = new_curr.x + offset.x;
            let y = new_curr.y + offset.y;
            while self.map.len() <= y {
                self.map.push([false; WIDTH]);
            }
            x < WIDTH && !self.map[y][x]
        })
    }

    pub fn simulate(&mut self, target: usize, jets: Vec<Jet>) -> Option<usize> {
        while self.piece_count < target {
            let piece = PIECES[self.piece_count % PIECES.len()];

            self.curr.x = 2;
            self.curr.y = self.top + 3;

            loop {
                let jet = &jets[self.jet_count % jets.len()];
                let new_curr = match jet {
                    Jet::Left => Coord {
                        x: self.curr.x.saturating_sub(1),
                        y: self.curr.y,
                    },
                    Jet::Right => Coord {
                        x: self.curr.x + 1,
                        y: self.curr.y,
                    },
                };
                if self.is_valid(&new_curr, piece) {
                    self.curr = new_curr;
                }
                self.jet_count += 1;

                let new_curr = Coord {
                    x: self.curr.x,
                    y: self.curr.y.saturating_sub(1),
                };

                if self.curr.y == 0 || !self.is_valid(&new_curr, piece) {
                    break;
                }
                self.curr = new_curr
            }

            for offset in piece {
                let x = self.curr.x + offset.x;
                let y = self.curr.y + offset.y;
                while self.map.len() <= y {
                    self.map.push([false; WIDTH]);
                }
                self.map[y][x] = true;
                self.top = self.top.max(y + 1);
            }

            if self.added_by_repeat == 0 {
                let key = (self.piece_count % PIECES.len(), self.jet_count % jets.len());

                if let Some((2, old_piece_count, old_top)) = self.seen.get(&key) {
                    let delta_top = self.top - old_top;
                    let delta_piece_count = self.piece_count - old_piece_count;
                    let repeats = (target - self.piece_count) / delta_piece_count;
                    self.piece_count += repeats * delta_piece_count;
                    self.added_by_repeat = delta_top * repeats;
                }

                self
                .seen
                .entry(key)
                    .and_modify(|(amnt, old_piece_count, old_top)| {
                        *amnt += 1;
                        *old_piece_count = self.piece_count;
                        *old_top = self.top;
                    })
                .or_insert((1, self.piece_count, self.top));
            }

            self.piece_count += 1;
        }
        Some(self.top + self.added_by_repeat)
    }
}

impl Display for State {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let piece = PIECES[self.piece_count % PIECES.len()];
        let mut print: Vec<Vec<_>> = self
            .map
            .iter()
            .map(|row| {
                row.iter()
                    .map(|rock| if *rock { '#' } else { '.' })
                    .collect()
            })
            .collect();
        let mut local_top = self.top + 1;
        for offset in piece {
            let x = self.curr.x + offset.x;
            let y = self.curr.y + offset.y;
            while print.len() <= y {
                print.push(vec!['.'; WIDTH]);
            }
            print[y][x] = '@';
            local_top = local_top.max(y + 1);
        }

        for row in (0..local_top).rev() {
            let mut row_str = String::from('|');
            for col in 0..WIDTH {
                row_str.push(print[row][col]);
            }
            row_str.push('|');
            row_str.push('\n');
            write!(f, "{}", row_str)?;
        }
        writeln!(f, "+{}+", "-".repeat(WIDTH))
    }
}

pub fn parse(input: &str) -> Vec<Jet> {
    input
        .trim()
        .chars()
        .map(|char| match char {
            '>' => Jet::Right,
            '<' => Jet::Left,
            _ => panic!("invalid input : {}", char),
        })
        .collect()
}

pub fn part_one(input: &str) -> Option<usize> {
    State::default().simulate(2022, parse(input))
}

pub fn part_two(input: &str) -> Option<usize> {
    State::default().simulate(1_000_000_000_000, parse(input))
}

fn main() {
    let input = &advent_of_code::read_file("inputs", 17);
    advent_of_code::solve!(1, part_one, input);
    advent_of_code::solve!(2, part_two, input);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let input = advent_of_code::read_file("examples", 17);
        assert_eq!(part_one(&input), Some(3068));
    }

    #[test]
    fn test_part_two() {
        let input = advent_of_code::read_file("examples", 17);
        assert_eq!(part_two(&input), Some(1514285714288));
    }
}
