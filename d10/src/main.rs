use std::error::Error;
use std::io;

#[derive(Debug, PartialEq, Eq)]
enum Op {
    Noop,
    Addx(i32),
}

struct Ops {
    ops: Vec<Op>,
}

impl Ops {
    fn parse(content: &str) -> Self {
        let mut ops = Vec::new();

        for ln in content.lines() {
            match ln {
                "noop" => ops.push(Op::Noop),

                // Ignore "addx " characters
                _ => ops.push(Op::Addx(ln[5..].parse::<i32>().unwrap())),
            }
        }

        Ops { ops }
    }

    fn total_signal_strengths(&self) -> i32 {
        // Program counter
        let mut pc = 0;
        // Register
        let mut x = 1;

        let mut sum = 0;

        for op in &self.ops {
            let cnt = match op {
                Op::Noop => 1,
                Op::Addx(_) => 2,
            };

            for _ in 0..cnt {
                pc += 1;
                match pc {
                    20 | 60 | 100 | 140 | 180 | 220 => sum += pc * x,
                    _ => (),
                }
            }

            if let Op::Addx(val) = op {
                x += val;
            }
        }

        sum
    }

    fn render(&self) -> String {
        // Program counter
        let mut pc = 0;
        // Pixel counter
        let mut px = 0;
        // Register
        let mut x = 1;

        let mut out = String::new();

        for op in &self.ops {
            let cnt = match op {
                Op::Noop => 1,
                Op::Addx(_) => 2,
            };

            for _ in 0..cnt {
                // Start row at pixel 0
                px += 1;

                // Format line
                if pc % 40 == 0 {
                    out.push('\n');
                    // reset pixel counter
                    px = 0;
                };

                pc += 1;

                match x == px - 1 || x == px || x == px + 1 {
                    true => out.push('#'),
                    false => out.push('.'),
                }
            }

            if let Op::Addx(val) = op {
                x += val;
            }
        }

        out
    }
}

fn main() -> Result<(), Box<dyn Error>> {
    let content = io::read_to_string(io::stdin())?;
    let ops = Ops::parse(&content);

    let total_strengths = ops.total_signal_strengths();
    let render = ops.render();

    println!("Total signal strengths (part 1): {total_strengths}\n{render}");

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_ops() {
        assert_eq!(
            Ops::parse(
                "\
noop
noop
addx 3
addx 304
addx -4
noop
addx 0
addx 13"
            )
            .ops,
            vec![
                Op::Noop,
                Op::Noop,
                Op::Addx(3),
                Op::Addx(304),
                Op::Addx(-4),
                Op::Noop,
                Op::Addx(0),
                Op::Addx(13)
            ]
        );
    }

    #[test]
    fn test_total_signal_strengths() {
        assert_eq!(Ops::parse(LONG_PROGRAM).total_signal_strengths(), 13140);
    }

    #[test]
    fn test_render() {
        assert_eq!(
            Ops::parse(LONG_PROGRAM).render(),
            "
##..##..##..##..##..##..##..##..##..##..
###...###...###...###...###...###...###.
####....####....####....####....####....
#####.....#####.....#####.....#####.....
######......######......######......####
#######.......#######.......#######....."
        );
    }

    const LONG_PROGRAM: &str = "\
addx 15
addx -11
addx 6
addx -3
addx 5
addx -1
addx -8
addx 13
addx 4
noop
addx -1
addx 5
addx -1
addx 5
addx -1
addx 5
addx -1
addx 5
addx -1
addx -35
addx 1
addx 24
addx -19
addx 1
addx 16
addx -11
noop
noop
addx 21
addx -15
noop
noop
addx -3
addx 9
addx 1
addx -3
addx 8
addx 1
addx 5
noop
noop
noop
noop
noop
addx -36
noop
addx 1
addx 7
noop
noop
noop
addx 2
addx 6
noop
noop
noop
noop
noop
addx 1
noop
noop
addx 7
addx 1
noop
addx -13
addx 13
addx 7
noop
addx 1
addx -33
noop
noop
noop
addx 2
noop
noop
noop
addx 8
noop
addx -1
addx 2
addx 1
noop
addx 17
addx -9
addx 1
addx 1
addx -3
addx 11
noop
noop
addx 1
noop
addx 1
noop
noop
addx -13
addx -19
addx 1
addx 3
addx 26
addx -30
addx 12
addx -1
addx 3
addx 1
noop
noop
noop
addx -9
addx 18
addx 1
addx 2
noop
noop
addx 9
noop
noop
noop
addx -1
addx 2
addx -37
addx 1
addx 3
noop
addx 15
addx -21
addx 22
addx -6
addx 1
noop
addx 2
addx 1
noop
addx -10
noop
noop
addx 20
addx 1
addx 2
addx 2
addx -6
addx -11
noop
noop
noop";
}
