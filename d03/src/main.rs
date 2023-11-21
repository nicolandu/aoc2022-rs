use std::collections::HashSet;
use std::error::Error;
use std::io;

/// Outputs the priority value corresponding to a letter.
/// Panics if value outside of \[a-zA-Z\].
/// a => 1, ... z => 26
/// A => 27, ... Z => 52
fn priority_value(value: char) -> u32 {
    match value {
        'a'..='z' => value as u32 - 'a' as u32 + 1,
        'A'..='Z' => value as u32 - 'A' as u32 + 1 + 26,
        _ => panic!("Character out of range: {value}"),
    }
}

fn get_priority_line(line: &str) -> u32 {
    let chars: Vec<_> = line.chars().collect();
    let mid = chars.len() / 2;

    let first_set: HashSet<_> = chars[..mid].iter().collect();

    let value = chars[mid..]
        .iter()
        .find(|c| first_set.contains(c))
        .expect("a line didn't match");

    priority_value(*value)
}

fn get_priority_group(group: &[&str]) -> u32 {
    let mut chars: Vec<HashSet<_>> = group.iter().map(|line| line.chars().collect()).collect();

    // Allows to borrow part of it mutably, and part of it immutably
    let (first, others) = chars.split_at_mut(1);

    first[0].retain(|elem| others[0].contains(elem) && others[1].contains(elem));
    priority_value(*first[0].iter().next().expect("Unable to find intersection"))
}

fn main() -> Result<(), Box<dyn Error>> {
    let content = io::read_to_string(io::stdin())?;

    let line_sum: u32 = content.lines().map(|line| get_priority_line(line)).sum();

    let group_sum: u32 = content
        .lines()
        .collect::<Vec<_>>()
        .chunks_exact(3)
        .map(|group| get_priority_group(group))
        .sum();

    println!("Line sum (part 1): {line_sum}");
    println!("Group sum (part 2): {group_sum}");

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_line_matches() {
        assert_eq!(get_priority_line("aa"), 1);
        assert_eq!(get_priority_line("abcZdZtr"), 52);
    }

    #[test]
    #[should_panic]
    fn test_line_no_match() {
        get_priority_line("ab");
    }

    #[test]
    #[should_panic]
    fn test_line_no_alpha() {
        get_priority_line(";;");
    }

    #[test]
    fn test_group_matches() {
        assert_eq!(get_priority_group(&["milder", "MILDERKeBABS", "kebabs"]), 5);
        assert_eq!(get_priority_group(&["BritishBroadcastingCorporation", "MILDERKEBABS", "hexBF"]), 28);
    }
}
