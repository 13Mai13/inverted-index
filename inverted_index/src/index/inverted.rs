use std::collections::HashMap;

#[derive(Debug, Default)]
// derive attribute:  implements two traits for our struct:
// 1. Lets you print the struct for debugging using {:?} in println! -> println!("{:?}", index);
// 2. Provides a default way to create the struct -> InvertedIndex::default() 
pub struct InvertedIndex {
    index: HashMap<String, Vec<usize>>,
}

impl InvertedIndex {
    pub fn new() -> Self {
        Self {
            index: HashMap::new(),
        }
    }

    pub fn add_document(&mut self, doc_id: usize, content: &str) {
        for word in content.split_whitespace() {
            let word = word.to_lowercase();
            self.index            // Start with our HashMap
                .entry(word)      // Look up the word, creating an Entry
                .or_insert(Vec::new())  // If word isn't found, insert new empty Vec
                .push(doc_id);    // Add the doc_id to the Vec
        }
    }

    pub fn search(&self, word: &str) -> Vec<usize> {
        self.index
            .get(&word.to_lowercase()) // Look up the lowercase word
            .cloned()                  // Make a copy of the Vec if we found it
            .unwrap_or_default()       // Return empty Vec if nothing found
    }
}
