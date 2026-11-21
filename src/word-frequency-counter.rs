use std::collections::HashMap;

fn main() {
    let sentence = "If you cannot do great things, do small things in a great way.";
    let word_count: HashMap<String, usize> = calculate_word_frequencies(sentence);
    for (word, count) in &word_count {
        println!("{word}: {count}");
    }
}

fn calculate_word_frequencies(sentence: &str) -> HashMap<String, usize> {
    let mut counts: HashMap<String, usize> = HashMap::new();
    for word in sentence.split_whitespace() {
        let key = word.to_lowercase();
        *counts.entry(key).or_insert(0) +=1;
    }
    counts
}