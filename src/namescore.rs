use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

fn char_to_number(c: char) -> Option<u32> {
    match c {
        'A'..='Z' => Some(c as u32 - 'A' as u32 + 1),
        _ => None,
    }
}


pub fn main() -> io::Result<()> {
    let start = std::time::Instant::now();

    let path = Path::new("C:\\Users\\bdrkh\\Downloads\\names.txt");
    let file = File::open(&path)?;
    let reader = io::BufReader::new(file);

    let mut words: Vec<String> = Vec::new();
    for line_result in reader.lines() {
        let line = line_result?;
        // Remove quotes and split the line by commas
        words.extend(line.replace('\"', "").split(',')
                         .filter(|s| !s.is_empty())
                         .map(String::from));
    }

    words.sort_unstable(); // Sort the words

    let mut result = 0;
    for (index, word) in words.iter().enumerate() {
        let word_position = (index + 1) as u32;
        let sum: u32 = word.chars()
                           .filter_map(char_to_number)
                           .sum();
        let score = sum * word_position;
        result += score;

        // Special handling for "COLIN"
        if word == "COLIN" {
            println!("COLIN: {} ({} * {})", score, sum, word_position);
        }
    }

    println!("Total score: {}", result);
    let duration = start.elapsed();
    println!("Time to run it: {:?}", duration);
    Ok(())
}
