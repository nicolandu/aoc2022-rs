use std::collections::VecDeque;
use std::error::Error;
use std::io;

#[derive(Clone)]
struct Stacks {
    data: Vec<VecDeque<char>>,
}

impl Stacks {
    /// Parses an initial state and returns the stacks for further transformation.
    /// Front of stack is the bottom, the back is the top.
    /// Panics if invalid initial state.
    fn init(state: &str) -> Stacks {
        let num_stacks = state
            .lines()
            .next()
            .expect("Expected a line")
            .chars()
            .count();
        assert!(
            (num_stacks + 1) % 4 == 0,
            "Error: wrong text width in initial state"
        );
        let num_stacks = (num_stacks + 1) / 4;

        let mut data = Vec::new();
        data.resize(num_stacks, VecDeque::new());

        for line in state.lines() {
            for (c, stack) in line.chars().skip(1).step_by(4).zip(data.iter_mut()) {
                // Skip numbers, only keep letters
                match c {
                    'A'..='Z' => stack.push_front(c),
                    _ => (),
                }
            }
        }

        Stacks { data }
    }

    /// Applies a series of moves to the stacks.
    /// Panics if moves are invalid.
    /// move_at_once : whether all crates are moved simultaneously, or one at a time.
    fn transform(&mut self, moves: &str, move_at_once: bool) -> &mut Self {
        for line in moves.lines() {
            let indices: Vec<_> = line
                .replace(|c| !char::is_numeric(c), " ")
                .split_whitespace()
                .map(|s| s.parse::<usize>().expect("Not an integer"))
                .collect();
            assert_eq!(indices.len(), 3, "Not three integers in actions line");

            let cnt = indices[0];

            // Make 0-indexed
            let src = indices[1] - 1;
            let dst = indices[2] - 1;

            let start_index = self.data[src].len() - cnt;

            if move_at_once {
                let pop_vals = self.data[src].drain(start_index..).collect::<Vec<_>>();
                self.data[dst].extend(pop_vals);
            } else {
                let pop_vals = self.data[src]
                    .drain(start_index..)
                    .rev()
                    .collect::<Vec<_>>();
                self.data[dst].extend(pop_vals);
            }
        }

        self
    }

    fn display(&self) -> String {
        self.data.iter().fold(String::new(), |mut string, stack| {
            string.push(*stack.back().unwrap_or(&' '));
            string
        })
    }
}

fn main() -> Result<(), Box<dyn Error>> {
    let content = io::read_to_string(io::stdin())?;
    let (initial, moves) = content
        .split_once("\n\n")
        .expect("Didn't match \\n\\n in lines");

    let mut stacks = Stacks::init(initial);
    println!(
        "Part 1: {}",
        stacks.clone().transform(moves, false).display()
    );
    println!("Part 1: {}", stacks.transform(moves, true).display());

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_stacks() {
        let mut stacks1 = Stacks::init(
            "    [D]    
[N] [C]    
[Z] [M] [P]
 1   2   3 ",
        );

        assert_eq!(
            stacks1.data,
            [
                VecDeque::from(['Z', 'N']),
                VecDeque::from(['M', 'C', 'D']),
                VecDeque::from(['P'])
            ]
        );

        let mut stacks2 = stacks1.clone();

        stacks1.transform(
            "move 1 from 2 to 1
move 3 from 1 to 3
move 2 from 2 to 1
move 1 from 1 to 2",
            false,
        );

        assert_eq!(
            stacks1.data,
            [
                VecDeque::from(['C']),
                VecDeque::from(['M']),
                VecDeque::from(['P', 'D', 'N', 'Z'])
            ]
        );

        stacks2.transform(
            "move 1 from 2 to 1
move 3 from 1 to 3
move 2 from 2 to 1
move 1 from 1 to 2",
            true,
        );

        assert_eq!(
            stacks2.data,
            [
                VecDeque::from(['M']),
                VecDeque::from(['C']),
                VecDeque::from(['P', 'Z', 'N', 'D'])
            ]
        );
    }
}
