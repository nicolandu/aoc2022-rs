use std::cmp::Ordering;
use std::error::Error;
use std::io;

fn parse_map(text: &str) -> Vec<Vec<u32>> {
    let mut map: Vec<_> = Vec::new();

    for line_text in text.lines() {
        let mut line: Vec<u32> = Vec::new();
        for c in line_text.chars() {
            line.push(c.to_digit(10).expect("Couldn't parse as an integer"));
        }
        map.push(line);
    }

    map
}

fn count_visible(tiles: &Vec<Vec<u32>>) -> u32 {
    let imax = tiles.len();
    let jmax = tiles[0].len();
    let mut cnt = 0;

    for i in 0..imax {
        for j in 0..jmax {
            let elem = tiles[i][j];

            // Search left
            if tiles[i][..j].iter().all(|&x| x < elem) {
                cnt += 1;
                continue;
            }

            // Search right
            if tiles[i][j + 1..].iter().all(|&x| x < elem) {
                cnt += 1;
                continue;
            }

            // Search up
            if tiles[..i].iter().all(|x| x[j] < elem) {
                cnt += 1;
                continue;
            }

            // Search down
            if tiles[i + 1..].iter().all(|x| x[j] < elem) {
                cnt += 1;
                continue;
            }
        }
    }
    cnt
}

fn max_scenic_score(tiles: &Vec<Vec<u32>>) -> usize {
    let imax = tiles.len();
    let jmax = tiles[0].len();
    let mut best = 0;

    // Trees on the edge have a scenic score of zero, so we don't include them in the calculation.
    for i in 1..imax - 1 {
        for j in 1..jmax - 1 {
            let elem = tiles[i][j];

            let score = [
                tiles[i][..j].iter().rev().collect::<Vec<_>>(),
                tiles[i][j + 1..].iter().collect(),
                tiles[..i].iter().rev().map(|v| &v[j]).collect(),
                tiles[i + 1..].iter().map(|v| &v[j]).collect(),
            ]
            .iter()
            .map(|seq| {
                let mut cnt = 0;

                for &&e in seq {
                    match e.cmp(&elem) {
                        Ordering::Less => cnt += 1,
                        // If same height or taller
                        _ => {
                            cnt += 1;
                            break;
                        }
                    }
                }

                cnt
            })
            .product();
            best = best.max(score);
        }
    }
    best
}

fn main() -> Result<(), Box<dyn Error>> {
    let content = io::read_to_string(io::stdin())?;

    let map = parse_map(&content);
    let cnt = count_visible(&map);
    let score = max_scenic_score(&map);

    println!("Visible: {cnt}, max scenic score: {score}");

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_map() {
        assert_eq!(
            parse_map(
                "\
133456
934567
345678"
            ),
            [[1, 3, 3, 4, 5, 6], [9, 3, 4, 5, 6, 7], [3, 4, 5, 6, 7, 8]]
        );
    }

    #[test]
    fn test_count_visible() {
        assert_eq!(
            count_visible(&vec![
                vec![1, 3, 3, 4, 5, 6],
                vec![9, 3, 4, 4, 6, 7],
                vec![3, 4, 5, 9, 1, 8],
                vec![2, 4, 9, 7, 5, 1],
            ]),
            21
        );
    }

    #[test]
    fn test_max_scenic_score() {
        assert_eq!(
            max_scenic_score(&vec![
                vec![3, 0, 3, 7, 3],
                vec![2, 5, 5, 1, 2],
                vec![6, 5, 3, 3, 2],
                vec![3, 3, 5, 4, 9],
                vec![3, 5, 3, 9, 0],
            ]),
            8
        );
    }
}
