use std::fs;
use std::io::{self, Write,BufRead,BufReader};
use std::collections::HashMap;
fn main() -> io::Result<()> {
    let file = fs::File::open("output.log")?;
    let reader = BufReader::new(file);
    let mut  total_lines = 0;
    let mut level_count = HashMap::new();

    let log_levels = vec!["ERROR","INFO","DEBUG"];

    for line in reader.lines() {
        let line = line?;
        total_lines += 1;

        for level in &log_levels {
            if line.contains(level) {
                *level_count
                    .entry(level)
                    .and_modify(|val| *val += 1)
                    .or_insert(0) += 1;
            }
        }
    }
    println!("Total lines: {}", total_lines);
    for (level, count) in &level_count {
        println!("{}: {}", level, count);
    }

    if let Some((key,val)) = level_count.iter().max_by_key(|entry| entry.1) {
        println!("Most frequent log level: {} with {} occurrences", key, val);
    } else {
        println!("No log levels found.");
    }
    Ok(())
}
