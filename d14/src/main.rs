use std::{collections::HashSet, io};

/*
  4     5  5
  9     0  0
  4     0  3
0 ......+...
1 ..........
2 ..........
3 ..........
4 ....#...##
5 ....#...#.   --> X
6 ..###...#.  |
7 ........#.  V
8 ........#.
9 #########.  Y
*/

#[derive(Debug)]
struct Map {
    all: HashSet<(u32, u32)>, // (x,y)
    lowest_rock: u32,
}

impl Map {
    fn tile_empty(&self, x: u32, y: u32, part2: bool) -> bool {
        if part2 && y >= self.lowest_rock + 2 {
            return false;
        };
        !self.all.contains(&(x, y))
    }
}

fn parse_map<T: AsRef<str>>(lines: impl Iterator<Item = T>) -> Map {
    let mut map = Map {
        all: HashSet::new(),
        lowest_rock: 0,
    };
    for line in lines.filter(|l| !l.as_ref().is_empty()) {
        let mut coords = line.as_ref().split(" -> ").map(|c| {
            let (x, y) = c.split_once(',').expect("A point should have 2 coords");
            (
                x.parse().expect("int conversion error"),
                y.parse().expect("int conversion error"),
            )
        });
        let mut cur: (u32, u32) = coords
            .next()
            .expect("Should have at least 1 coord on a line");
        for new in coords {
            if new.0 == cur.0 {
                // same x: vertical line
                for y in new.1.min(cur.1)..=new.1.max(cur.1) {
                    map.all.insert((new.0, y));
                }
            } else if new.1 == cur.1 {
                // same y: horizontal line
                for x in new.0.min(cur.0)..=new.0.max(cur.0) {
                    map.all.insert((x, new.1));
                }
            } else {
                panic!("No diagonals, please!")
            };

            cur = new;
        }
        map.lowest_rock = map.all.iter().map(|p| p.1).max().expect("no max");
    }
    map
}

fn count_sand1(mut map: Map) -> u32 {
    let mut cnt = 0;
    'outer: loop {
        /*
        for y in 0..=map.lowest_rock {
            for x in 420..=580 {
                print!("{}", if map.all.contains(&(x, y)) { "#" } else { "." });
            }
            println!();
        }
        println!();
        */
        if !map.tile_empty(500, 0, false) {
            break;
        }
        let mut x = 500u32;
        for y in 1..=map.lowest_rock {
            // straight down
            if map.tile_empty(x, y, false) {
                continue;
            };
            // left
            if map.tile_empty(x - 1, y, false) {
                x -= 1;
                continue;
            };
            // right
            if map.tile_empty(x + 1, y, false) {
                x += 1;
                continue;
            };
            map.all.insert((x, y - 1));
            cnt += 1;
            continue 'outer;
        }
        break;
    }

    cnt
}

fn count_sand2(mut map: Map) -> u32 {
    let mut cnt = 0;
    'outer: loop {
        /*
        for y in 0..=map.lowest_rock + 1 {
            for x in 420..=580 {
                print!("{}", if map.all.contains(&(x, y)) { "#" } else { "." });
            }
            println!();
        }
        print!("{}", "#".repeat((420u32..=580).count()));
        println!();
        println!();
        */
        if !map.tile_empty(500, 0, false) {
            break;
        }
        let mut x = 500u32;
        for y in 1..=map.lowest_rock + 2 {
            // straight down
            if map.tile_empty(x, y, true) {
                continue;
            };
            // left
            if map.tile_empty(x - 1, y, true) {
                x -= 1;
                continue;
            };
            // right
            if map.tile_empty(x + 1, y, true) {
                x += 1;
                continue;
            };
            map.all.insert((x, y - 1));
            cnt += 1;
            continue 'outer;
        }
        break;
    }

    cnt
}

fn main() {
    let lines: Vec<_> = io::stdin()
        .lines()
        .map_while(|r| r.ok())
        .filter(|l| !l.is_empty())
        .collect();
    println!("Part 1: {}", count_sand1(parse_map(lines.iter())));
    // We could continue onward from part 1 to save computation time, but I can't be bothered.
    println!("Part 2: {}", count_sand2(parse_map(lines.iter())));
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test1() {
        assert_eq!(
            count_sand1(parse_map(
                [
                    "498,4 -> 498,6 -> 496,6",
                    "503,4 -> 502,4 -> 502,9 -> 494,9"
                ]
                .into_iter()
            )),
            24
        );
    }

    #[test]
    fn test2() {
        assert_eq!(
            count_sand2(parse_map(
                [
                    "498,4 -> 498,6 -> 496,6",
                    "503,4 -> 502,4 -> 502,9 -> 494,9"
                ]
                .into_iter()
            )),
            93
        );
    }
}
