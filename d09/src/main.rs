use std::collections::HashSet;
use std::error::Error;
use std::io;
use std::ops;

use Move::*;

#[derive(Copy, Clone, Debug, PartialEq, Eq)]
enum Move {
    Up,
    Down,
    Left,
    Right,
}

impl Move {
    fn to_tile_movement(self) -> Tile {
        match self {
            Up => Tile { x: 0, y: 1 },
            Down => Tile { x: 0, y: -1 },
            Left => Tile { x: -1, y: 0 },
            Right => Tile { x: 1, y: 0 },
        }
    }
}

#[derive(Copy, Clone, Debug, PartialEq, Eq, Hash)]
struct Tile {
    x: isize,
    y: isize,
}
impl Tile {
    fn follow(&mut self, lead: &Tile) {
        let d = *lead - *self;

        if (d.x.abs() + d.y.abs() > 1) && (d.x.abs() > 1 || d.y.abs() > 1) {
            self.x += d.x.signum();
            self.y += d.y.signum()
        }
    }
}

impl ops::Add for Tile {
    type Output = Self;
    fn add(self, rhs: Self) -> Self {
        Tile {
            x: self.x + rhs.x,
            y: self.y + rhs.y,
        }
    }
}

impl ops::Sub for Tile {
    type Output = Self;
    fn sub(self, rhs: Self) -> Self {
        Tile {
            x: self.x - rhs.x,
            y: self.y - rhs.y,
        }
    }
}

impl ops::AddAssign for Tile {
    fn add_assign(&mut self, rhs: Self) {
        self.x += rhs.x;
        self.y += rhs.y;
    }
}

fn parse_moves(content: &str) -> Vec<Move> {
    let mut moves = Vec::new();
    content.lines().for_each(|l| {
        // Direction, space, count
        assert!(l.len() >= 3);
        let d = match l.chars().next().unwrap() {
            'U' => Up,
            'D' => Down,
            'L' => Left,
            'R' => Right,
            _ => unreachable!("Wrong direction letter"),
        };

        let cnt = l[2..].parse::<usize>().expect("Int parsing error");

        moves.extend((0..cnt).map(|_| d));
    });

    moves
}

fn visited_tiles(moves: &Vec<Move>, n: usize) -> usize {
    let mut knots = vec![Tile { x: 0, y: 0 }; n];
    let mut tail_visited = HashSet::from([*knots.last().unwrap()]);

    for &mv in moves {
        let mv = mv.to_tile_movement();
        knots[0] += mv;
        for i in 1..n {
            let prev = knots[i - 1];
            knots[i].follow(&prev);
        }
        let cur_visited = knots[n - 1];
        tail_visited.insert(cur_visited);
    }

    tail_visited.len()
}

fn main() -> Result<(), Box<dyn Error>> {
    let content = io::read_to_string(io::stdin())?;

    let moves = parse_moves(&content);
    let tiles_part1 = visited_tiles(&moves, 2);
    let tiles_part2 = visited_tiles(&moves, 10);

    println!("Part 1: {tiles_part1}, part 2: {tiles_part2}");

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_parse_moves() {
        assert_eq!(
            parse_moves(
                "\
U 1
D 1
L 3
R 2
U 10"
            ),
            [Up, Down, Left, Left, Left, Right, Right, Up, Up, Up, Up, Up, Up, Up, Up, Up, Up]
        );
    }

    #[test]
    fn test_visited_tiles_part1() {
        let moves = parse_moves(
            "\
R 4
U 4
L 3
D 1
R 4
D 1
L 5
R 2",
        );

        assert_eq!(visited_tiles(&moves, 2), 13);
    }

    #[test]
    fn test_visited_tiles_part2_a() {
        let moves = parse_moves(
            "\
R 4
U 4
L 3
D 1
R 4
D 1
L 5
R 2",
        );

        assert_eq!(visited_tiles(&moves, 10), 1);
    }

    #[test]
    fn test_visited_tiles_part2_b() {
        let moves_raw = "\
R 15
U 8
L 8
D 3
R 17
D 10
L 25
U 20";

        let moves = parse_moves(moves_raw);
        assert_eq!(visited_tiles(&moves, 10), calculate(moves_raw, 10));
    }

    // Other implementation, for reference. Adapted from https://github.com/dellink/advent-of-code/blob/main/2022/src/bin/09.rs
    fn calculate(input: &str, length: usize) -> usize {
        let mut rope = vec![(0i32, 0i32); length];
        let mut visited = HashSet::new();
        visited.insert((0, 0));

        for line in input.lines() {
            let instruction = line.split_once(' ').unwrap();

            let (dx, dy) = match instruction.0 {
                "U" => (0, 1),
                "D" => (0, -1),
                "R" => (1, 0),
                "L" => (-1, 0),
                _ => unreachable!(),
            };

            let steps = instruction.1.parse::<usize>().unwrap();

            for _ in 0..steps {
                rope[0] = (rope[0].0 + dx, rope[0].1 + dy);
                for i in 1..rope.len() {
                    let (dx, dy) = (rope[i - 1].0 - rope[i].0, rope[i - 1].1 - rope[i].1);
                    if dx.abs() > 1 || dy.abs() > 1 {
                        rope[i].0 += dx.signum();
                        rope[i].1 += dy.signum();
                    }
                }
                visited.insert(rope[length - 1]);
            }
        }

        visited.len()
    }
}
