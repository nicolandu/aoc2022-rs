use std::{error::Error, io, process};

fn main() -> Result<(), Box<dyn Error>> {
    let content = io::read_to_string(io::stdin())?;

    // Bleeping Windows
    let content = content.replace("\r\n", "\n");

    let mut reindeers = Vec::new();
    for block in content.split("\n\n") {
        let mut sum = 0;
        for line in block.lines() {
            if let Ok(x) = line.parse::<u32>() {
                sum += x;
            } else {
                eprintln!("Error parsing line:\n{line}");
                process::exit(1);
            }
        }
        reindeers.push(sum);
    }

    // Sort decreasing
    reindeers.sort_unstable_by(|a, b| a.partial_cmp(b).unwrap().reverse());

    if let Some(max) = reindeers.first() {
        println!("Max: {max}");
    } else {
        eprintln!("Couldn't find max value");
        process::exit(1);
    }

    println!("Greatest 3: {}", reindeers[0..3].iter().sum::<u32>());

    Ok(())
}
