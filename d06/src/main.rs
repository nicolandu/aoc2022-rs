use std::collections::HashSet;
use std::error::Error;
use std::io;

fn find_first_unique(content: &str, window_size: usize) -> Option<usize> {
    Some(
        content
        .chars()
        .collect::<Vec<_>>()
        .windows(window_size)
        .enumerate()
        .find(|elem| {
            let (_, elem) = elem;
            let mut set = HashSet::new();
            elem.iter().all(|x| set.insert(x))
        })?
        .0
        // Add window size to index, such that the first window returns WINDOW_SIZE
        + window_size,
    )
}

fn main() -> Result<(), Box<dyn Error>> {
    let content = io::read_to_string(io::stdin())?;
    println!(
        "Part 1: {}, Part 2: {}",
        find_first_unique(&content, 4).expect("No match found"),
        find_first_unique(&content, 14).expect("No match found"),
    );

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn empty() {
        assert_eq!(find_first_unique("", 4), None);
    }

    #[test]
    fn too_short() {
        assert_eq!(find_first_unique("abc", 4), None);
    }

    #[test]
    fn good() {
        assert_eq!(find_first_unique("frtffuiunanimaldecompagnie", 6), Some(16));
    }

    #[test]
    fn no_match() {
        assert_eq!(find_first_unique("abcdcbabcdcba", 5), None);
    }
}
