use core::cmp::Ordering;
use itertools::Itertools;
use lazy_static::lazy_static;
use regex::Regex;
use std::io;

#[derive(Debug, PartialEq, Eq, Clone)]
enum Elem {
    List(Vec<Elem>),
    Num(u32),
}

impl Ord for Elem {
    fn cmp(&self, other: &Self) -> Ordering {
        match (self, other) {
            (Elem::Num(a), Elem::Num(b)) => a.cmp(b),
            (Elem::List(a), Elem::List(b)) => {
                let mut ait = a.iter();
                let mut bit = b.iter();
                'out: loop {
                    let a = ait.next();
                    let b = bit.next();
                    match (a, b) {
                        (None, Some(_)) => break 'out Ordering::Less,
                        (Some(_), None) => break 'out Ordering::Greater,
                        (None, None) => break 'out Ordering::Equal,

                        (Some(aval), Some(bval)) => match aval.cmp(bval) {
                            // continue searching
                            Ordering::Equal => (),
                            // done
                            o => break 'out o,
                        },
                    }
                }
            }
            (Elem::Num(_), Elem::List(_)) => Elem::List(vec![self.clone()]).cmp(other),
            (Elem::List(_), Elem::Num(_)) => self.cmp(&Elem::List(vec![other.clone()])),
        }
    }
}

impl PartialOrd for Elem {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

fn parse_tokens<T: AsRef<str>>(match_iter: &mut impl Iterator<Item = T>) -> Elem {
    let mut elems: Vec<Elem> = vec![];
    loop {
        let x = match_iter.next().expect("Unclosed braces");
        let x = x.as_ref();
        match x {
            "[" => elems.push(parse_tokens(match_iter)),
            "]" => return Elem::List(elems),
            _ => elems.push(Elem::Num(x.parse().unwrap())),
        }
    }
}

fn parse_packet<T: AsRef<str> + ?Sized>(packet: &T) -> Elem {
    lazy_static! {
        static ref re: Regex = Regex::new(r"(\[|\d+|\])").unwrap();
    }
    parse_tokens(
        &mut re
            .find_iter(packet.as_ref())
            .map(|m| m.as_str())
            .skip_while(|&e| e != "[")
            .skip(1), // as we're already going into parse, drop initial bracket
    )
}

fn run_1<T: AsRef<str>>(lines: impl Iterator<Item = T>) -> usize {
    lines
        .map(|p| parse_packet(&p))
        .tuples()
        .enumerate()
        .filter_map(|(i, (a, b))| if a < b { Some(i + 1) } else { None })
        .sum()
}

fn run_2<T: AsRef<str>>(lines: impl Iterator<Item = T>) -> usize {
    let mut all_packets: Vec<_> = lines
        .map(|p| parse_packet(&p))
        .chain([
            Elem::List(vec![Elem::List(vec![Elem::Num(2)])]),
            Elem::List(vec![Elem::List(vec![Elem::Num(6)])]),
        ])
        .collect();

    all_packets.sort();
    (all_packets
        .iter()
        .position(|e| *e == Elem::List(vec![Elem::List(vec![Elem::Num(2)])]))
        .unwrap()
        + 1)
        * (all_packets
            .iter()
            .position(|e| *e == Elem::List(vec![Elem::List(vec![Elem::Num(6)])]))
            .unwrap()
            + 1)
}

fn main() {
    let lines: Vec<_> = io::stdin()
        .lines()
        .map_while(|r| r.ok())
        .filter(|l| !l.is_empty())
        .collect();
    let sum: usize = run_1(lines.iter());
    println!("Part 1: {}", sum);
    let prod: usize = run_2(lines.iter());
    println!("Part 2: {}", prod);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse() {
        assert_eq!(
            parse_packet("[1,[5,6],[[]],[[3]]]"),
            Elem::List(vec![
                Elem::Num(1),
                Elem::List(vec![Elem::Num(5), Elem::Num(6)]),
                Elem::List(vec![Elem::List(vec![])]),
                Elem::List(vec![Elem::List(vec![Elem::Num(3)])])
            ])
        );
    }
    #[test]
    fn test_order1() {
        assert!(parse_packet("[1,1,3,1,1]") < parse_packet("[1,1,5,1,1]"));
    }
    #[test]
    fn test_order2() {
        assert!(parse_packet("[[1],[2,3,4]]") < parse_packet("[[1],4]"));
    }
    #[test]
    fn test_order3() {
        assert!(parse_packet("[9]") > parse_packet("[[8,7,6]]"));
    }
    #[test]
    fn test_order4() {
        assert!(parse_packet("[[4,4],4,4]") < parse_packet("[[4,4],4,4,4]"));
    }
    #[test]
    fn test_order5() {
        assert!(parse_packet("[7,7,7,7]") > parse_packet("[7,7,7]"));
    }
    #[test]
    fn test_order6() {
        assert!(parse_packet("[]") < parse_packet("[3]"));
    }
    #[test]
    fn test_order7() {
        assert!(parse_packet("[[[]]]") > parse_packet("[]"));
    }
    #[test]
    fn test_order8() {
        assert!(
            parse_packet("[1,[2,[3,[4,[5,6,7]]]],8,9]")
                > parse_packet("[1,[2,[3,[4,[5,6,0]]]],8,9]")
        );
    }
    #[test]
    fn test_run_1() {
        assert_eq!(
            run_1(
                "[1,1,3,1,1]
[1,1,5,1,1]

[[1],[2,3,4]]
[[1],4]

[9]
[[8,7,6]]

[[4,4],4,4]
[[4,4],4,4,4]

[7,7,7,7]
[7,7,7]

[]
[3]

[[[]]]
[[]]

[1,[2,[3,[4,[5,6,7]]]],8,9]
[1,[2,[3,[4,[5,6,0]]]],8,9]"
                    .lines()
                    .filter(|l| !l.is_empty())
            ),
            13
        );
    }
    #[test]
    fn test_run_2() {
        assert_eq!(
            run_2(
                "[1,1,3,1,1]
[1,1,5,1,1]

[[1],[2,3,4]]
[[1],4]

[9]
[[8,7,6]]

[[4,4],4,4]
[[4,4],4,4,4]

[7,7,7,7]
[7,7,7]

[]
[3]

[[[]]]
[[]]

[1,[2,[3,[4,[5,6,7]]]],8,9]
[1,[2,[3,[4,[5,6,0]]]],8,9]"
                    .lines()
                    .filter(|l| !l.is_empty())
            ),
            140
        );
    }
}
