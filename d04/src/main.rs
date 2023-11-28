use std::error::Error;
use std::io;

fn decode_line(elem: &str) -> ((u32, u32), (u32, u32)) {
    let segments: Vec<_> = elem
        .split(['-', ','])
        .map(|elem| {
            elem.to_string()
                .parse::<u32>()
                .unwrap_or_else(|_| panic!("Couldn't parse integer: {elem}"))
        })
        .collect();
    ((segments[0], segments[1]), (segments[2], segments[3]))
}

fn fully_contained(elem: &str) -> bool {
    let (first, second) = decode_line(elem);
    (first.0 >= second.0 && first.1 <= second.1) || (second.0 >= first.0 && second.1 <= first.1)
}

fn overlap(elem: &str) -> bool {
    let (first, second) = decode_line(elem);
    !(first.1 < second.0 || second.1 < first.0)
}

fn main() -> Result<(), Box<dyn Error>> {
    let content = io::read_to_string(io::stdin())?;

    let contained = content.lines().filter(|elem| fully_contained(elem)).count();
    let overlapping = content.lines().filter(|elem| overlap(elem)).count();

    println!("Fully contained: {contained}");
    println!("Partially overlapping: {overlapping}");

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_fully_contained() {
        // Same range
        assert!(fully_contained("1-10,1-10"));
        // First smaller
        assert!(fully_contained("10-101,0-101"));
        assert!(fully_contained("10-13,10-16"));
        // Second smaller
        assert!(fully_contained("4-8,4-6"));
        assert!(fully_contained("30-190,31-190"));
    }

    #[test]
    fn test_not_fully_contained() {
        assert!(!fully_contained("2-5,1-4"));
        assert!(!fully_contained("89-314,100-400"));
    }

    #[test]
    fn test_overlap() {
        // Same range
        assert!(overlap("1-10,1-10"));
        // First smaller, fully contained
        assert!(overlap("10-101,0-101"));
        assert!(overlap("10-13,10-16"));
        // Second smaller, fully_contained
        assert!(overlap("4-8,4-6"));
        assert!(overlap("30-190,31-190"));
        // Partial overlap
        assert!(overlap("2-5,1-4"));
        assert!(overlap("89-314,100-400"));
    }

    #[test]
    fn no_overlap() {
        assert!(!overlap("12-12,13-14"));
        assert!(!overlap("12-15,16-199"));
    }
}
