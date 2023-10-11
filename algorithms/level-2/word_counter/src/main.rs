use std::collections::HashMap;
use std::fs::File;
use std::io::{BufRead, BufReader};

fn main() {
    let file_name = "sample_text.txt";

    let file = File::open(file_name).expect("error reading the file.");

    let mut word_freq: HashMap<String, usize> = HashMap::new();

    let reader = BufReader::new(file);
    for line in reader.lines() {
        if let Ok(line) = line {
            let words: Vec<&str> = line.split_whitespace().collect();
            
            for word in words {
                let cleaned_word = word.trim_matches(|c: char| !c.is_alphabetic()).to_lowercase();
                
                *word_freq.entry(cleaned_word).or_insert(0) += 1;
            }
        }
    }

    for (word, count) in &word_freq {
        println!("{}: {}", word, count);
    }
}

