use inverted_index::InvertedIndex;
use std::io::{self, Write};

fn get_input(prompt: &str) -> io::Result<String> {
    print!("{}", prompt);
    io::stdout().flush()?;
    
    let mut input = String::new();
    io::stdin().read_line(&mut input)?;
    Ok(input.trim().to_string())
}

fn main() {
    let mut index = InvertedIndex::new();
    
    // Add some initial documents
    index.add_document(1, "The quick brown fox");
    index.add_document(2, "The brown dog");
    
    loop {
        match get_input("\nEnter a word to search (or 'quit' to exit): ") {
            Ok(input) => {
                if input == "quit" {
                    println!("Goodbye!");
                    break;
                }

                let results = index.search(&input);
                
                if results.is_empty() {
                    println!("No documents found containing '{}'", input);
                } else {
                    println!("Documents containing '{}': {:?}", input, results);
                }
            }
            Err(error) => {
                eprintln!("Error reading input: {}", error);
                break;
            }
        }
    }
}
