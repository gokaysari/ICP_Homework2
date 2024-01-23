use std::io::{self, Write};

pub struct WordCounter {
    text: String,
}

impl WordCounter {
    pub fn new(text: &str) -> Self {
        WordCounter {
            text: text.to_string(),
        }
    }

    pub fn count_words(&self) -> usize {
        self.text.split_whitespace().count()
    }
}

fn main() {
    println!("Please enter a text:");
    io::stdout().flush().unwrap();

    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("Failed!");

    let word_counter = WordCounter::new(&input);
    let word_count = word_counter.count_words();

    println!("Number of words: {}", word_count);
}
