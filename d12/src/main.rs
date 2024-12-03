use std::collections::VecDeque;
use std::error::Error;
use std::io;

use itertools::Itertools;

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
struct Point {
    x: usize,
    y: usize,
}

#[derive(Debug, PartialEq, Eq)]
struct Map {
    heights: Vec<Vec<u8>>,
    start: Point,
    end: Point,
}

impl Map {
    fn parse(content: &str) -> Self {
        let mut start: Option<_> = None;
        let mut end: Option<_> = None;

        let heights = content
            .lines()
            .enumerate()
            .map(|(y, l)| {
                l.char_indices()
                    .map(|(x, c)| {
                        let c = match c {
                            'S' => {
                                start = Some(Point { x, y });
                                'a'
                            }
                            'E' => {
                                end = Some(Point { x, y });
                                'z'
                            }
                            _ => c,
                        };

                        assert!(c.is_ascii_lowercase(), "Character out of range: [a-z]");
                        c as u8 - b'a'
                    })
                    .collect()
            })
            .collect();

        Self {
            heights,
            start: start.expect("No start point found"),
            end: end.expect("No endpoint found"),
        }
    }

    fn solve(&self) -> Option<u32> {
        self.solve_with_start(self.start)
    }

    fn solve_with_start(&self, start_point: Point) -> Option<u32> {
        let mut visited = vec![vec![false; self.heights[0].len()]; self.heights.len()];
        let mut queue: VecDeque<(_, u32)> = [(start_point, 0)].into();

        while let Some((tile, distance)) = queue.pop_front() {
            if tile == self.end {
                return Some(distance);
            }

            for offset in [(1, 0), (0, 1), (-1, 0), (0, -1)] {
                let (new_x, new_y) = (
                    (isize::try_from(tile.x).unwrap() + offset.0) as usize,
                    (isize::try_from(tile.y).unwrap() + offset.1) as usize,
                );

                // We don't care if a negative isize has wrapped around when cast to an usize.
                // Get handles that for us, and if get returns None, we don't index any further.
                let Some(&new_tile) = self.heights.get(new_y).and_then(|r| r.get(new_x)) else {
                    continue;
                };

                if (new_tile <= self.heights[tile.y][tile.x] + 1) && !visited[new_y][new_x] {
                    visited[new_y][new_x] = true;
                    queue.push_back((Point { x: new_x, y: new_y }, distance + 1));
                }
            }
        }

        None
    }

    fn find_best(&self) -> Option<u32> {
        let starting_points = (0..self.heights.len())
            .cartesian_product(0..self.heights[0].len())
            .filter(|&(y, x)| self.heights[y][x] == 0)
            .map(|(y, x)| Point { x, y })
            .collect::<Vec<_>>();

        starting_points
            .iter()
            .filter_map(|&p| self.solve_with_start(p))
            .min()
    }
}

fn main() -> Result<(), Box<dyn Error>> {
    let content = io::read_to_string(io::stdin())?;
    let map = Map::parse(&content);
    let score = map.solve().expect("No result found");
    let best = map.find_best().expect("No result found");

    println!("Score (part 1): {score}");
    println!("Optimal score (part 2): {best}");

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse() {
        assert_eq!(
            Map::parse(
                "\
Sabqponm
abcryxxl
accszExk
acctuvwj
abdefghi"
            ),
            Map {
                // For padding
                #[allow(clippy::zero_prefixed_literal)]
                heights: vec![
                    vec![0, 0, 1, 16, 15, 14, 13, 12],
                    vec![0, 1, 2, 17, 24, 23, 23, 11],
                    vec![0, 2, 2, 18, 25, 25, 23, 10],
                    vec![0, 2, 2, 19, 20, 21, 22, 09],
                    vec![0, 1, 3, 04, 05, 06, 07, 08]
                ],
                start: Point { x: 0, y: 0 },
                end: Point { x: 5, y: 2 }
            }
        );
    }

    #[test]
    fn test_solve() {
        assert_eq!(
            Map {
                // For padding
                #[allow(clippy::zero_prefixed_literal)]
                heights: vec![
                    vec![0, 0, 1, 16, 15, 14, 13, 12],
                    vec![0, 1, 2, 17, 24, 23, 23, 11],
                    vec![0, 2, 2, 18, 25, 25, 23, 10],
                    vec![0, 2, 2, 19, 20, 21, 22, 09],
                    vec![0, 1, 3, 04, 05, 06, 07, 08]
                ],
                start: Point { x: 0, y: 0 },
                end: Point { x: 5, y: 2 }
            }
            .solve(),
            Some(31)
        );
    }
}
