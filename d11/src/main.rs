use anyhow::{bail, ensure, Result};
use std::{collections::VecDeque, io};

#[derive(Debug)]
enum ParseError {
    MissingData,
    InvalidData,
}

impl std::error::Error for ParseError {}
impl std::fmt::Display for ParseError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::MissingData => write!(f, "Parse error: Missing data"),
            Self::InvalidData => write!(f, "Parse error: Invalid data"),
        }
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
enum Operation {
    Add(u64),
    Multiply(u64),
    Square,
}

#[derive(Clone, Debug, PartialEq, Eq)]
struct Monkey {
    items: VecDeque<u64>,
    operation: Operation,
    div_cmp: u64,
    if_divisible: usize,
    else_divisible: usize,
}

#[derive(Clone, Debug, PartialEq, Eq)]
struct MonkeyGroup {
    monkeys: Vec<Monkey>,
}

impl MonkeyGroup {
    fn parse(content: &str) -> Result<Self> {
        const ITEMS_OFFSET: usize = "  Starting items: ".len();
        const OPERATION_OFFSET: usize = "  Operation: new = old ".len();
        const DIV_CMP_OFFSET: usize = "  Test: divisible by ".len();
        const IF_DIVISIBLE_OFFSET: usize = "    If true: throw to monkey ".len();
        const ELSE_DIVISIBLE_OFFSET: usize = "    If false: throw to monkey ".len();

        let ret: Result<Vec<_>> = content
            .replace("\r\n", "\n")
            .split("\n\n")
            .map(|par| {
                let mut par = par.lines();

                par.next();

                let items = par.next().ok_or(ParseError::MissingData)?;
                ensure!(items.len() > ITEMS_OFFSET);

                let items: VecDeque<_> = items[ITEMS_OFFSET..]
                    .split(", ")
                    .map(|elem| elem.parse().expect("Parse int error"))
                    .collect();

                let operation = par.next().ok_or(ParseError::MissingData)?;
                ensure!(operation.len() > OPERATION_OFFSET + 2);

                let operation = &operation[OPERATION_OFFSET..];

                let operation = match operation.chars().next().ok_or(ParseError::MissingData)? {
                    '+' => Operation::Add(operation[2..].parse()?),

                    // Length has already been checked
                    '*' => match operation.chars().nth(2).unwrap() {
                        'o' => Operation::Square,
                        _ => Operation::Multiply(operation[2..].parse()?),
                    },
                    _ => bail!(ParseError::InvalidData),
                };

                let div_cmp = par.next().ok_or(ParseError::MissingData)?;
                ensure!(div_cmp.len() > DIV_CMP_OFFSET);

                let div_cmp = div_cmp[DIV_CMP_OFFSET..].parse()?;

                let if_divisible = par.next().ok_or(ParseError::MissingData)?;
                ensure!(if_divisible.len() > IF_DIVISIBLE_OFFSET);

                let if_divisible = if_divisible[IF_DIVISIBLE_OFFSET..].parse()?;

                let else_divisible = par.next().ok_or(ParseError::MissingData)?;
                ensure!(else_divisible.len() > ELSE_DIVISIBLE_OFFSET);

                let else_divisible = else_divisible[ELSE_DIVISIBLE_OFFSET..].parse()?;

                Ok(Monkey {
                    items,
                    operation,
                    div_cmp,
                    if_divisible,
                    else_divisible,
                })
            })
            .collect();

        Ok(MonkeyGroup { monkeys: ret? })
    }

    fn score(mut self, num_rounds: usize, worry_reduction: impl Fn(u64) -> u64) -> usize {
        let num_monkeys = self.monkeys.len();
        assert!(num_monkeys >= 2, "Too few monkeys (>= 2 needed)");

        let mut inspections = vec![0; num_monkeys];

        for _ in 0..num_rounds {
            for (i, inspections) in inspections.iter_mut().enumerate() {
                // Clone monkey, then delete original items
                let mut monkey = self.monkeys[i].clone();
                self.monkeys[i].items.clear();

                *inspections += monkey.items.len();
                while let Some(item) = monkey.items.pop_front() {
                    let item = worry_reduction(match monkey.operation {
                        Operation::Add(x) => item + x,
                        Operation::Multiply(x) => item * x,
                        Operation::Square => item * item,
                    });

                    let throw_to = match (item % monkey.div_cmp) == 0 {
                        true => monkey.if_divisible,
                        false => monkey.else_divisible,
                    };

                    self.monkeys[throw_to].items.push_back(item);
                }
            }
        }

        // Sort by decreasing order
        inspections.sort_unstable_by(|a, b| a.cmp(b).reverse());
        inspections[0] * inspections[1]
    }
}

fn main() -> Result<()> {
    let content = io::read_to_string(io::stdin())?;
    let group = MonkeyGroup::parse(&content)?;

    let score_part1 = group.clone().score(20, |x| x / 3);

    // We use the product of all the moduli to avoid overflow when computing new worry.
    let modulus: u64 = group.monkeys.iter().map(|m| m.div_cmp).product();
    let score_part2 = group.score(10000, |x| x % modulus);

    println!("Level of monkey business: part 1={score_part1}, part 2={score_part2}");

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse() {
        assert_eq!(
            MonkeyGroup::parse(TEXT).unwrap().monkeys,
            vec![
                Monkey {
                    items: vec![79, 98].into(),
                    operation: Operation::Multiply(19),
                    div_cmp: 23,
                    if_divisible: 2,
                    else_divisible: 3
                },
                Monkey {
                    items: vec![54, 65, 75, 74].into(),
                    operation: Operation::Add(6),
                    div_cmp: 19,
                    if_divisible: 2,
                    else_divisible: 0
                },
                Monkey {
                    items: vec![79, 60, 97].into(),
                    operation: Operation::Square,
                    div_cmp: 13,
                    if_divisible: 1,
                    else_divisible: 3
                },
                Monkey {
                    items: vec![74].into(),
                    operation: Operation::Add(3),
                    div_cmp: 17,
                    if_divisible: 0,
                    else_divisible: 1
                },
            ]
        );
    }

    #[test]
    fn test_score_part1() {
        assert_eq!(
            MonkeyGroup::parse(TEXT).unwrap().score(20, |x| x / 3),
            10605
        );
    }

    #[test]
    fn test_score_part2() {
        let group = MonkeyGroup::parse(TEXT).unwrap();
        let modulus: u64 = group.monkeys.iter().map(|m| m.div_cmp).product();
        assert_eq!(group.score(10000, |x| x % modulus), 2713310158);
    }

    const TEXT: &str = "\
Monkey 0:
  Starting items: 79, 98
  Operation: new = old * 19
  Test: divisible by 23
    If true: throw to monkey 2
    If false: throw to monkey 3

Monkey 1:
  Starting items: 54, 65, 75, 74
  Operation: new = old + 6
  Test: divisible by 19
    If true: throw to monkey 2
    If false: throw to monkey 0

Monkey 2:
  Starting items: 79, 60, 97
  Operation: new = old * old
  Test: divisible by 13
    If true: throw to monkey 1
    If false: throw to monkey 3

Monkey 3:
  Starting items: 74
  Operation: new = old + 3
  Test: divisible by 17
    If true: throw to monkey 0
    If false: throw to monkey 1";
}
