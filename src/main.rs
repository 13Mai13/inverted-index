use std::io::{self, Write};
mod index;
use crate::index::inverted::InvertedIndex;

fn get_input(prompt: &str) -> io::Result<String> {
    print!("{}", prompt);
    io::stdout().flush()?;
    
    let mut input = String::new();
    io::stdin().read_line(&mut input)?;
    Ok(input.trim().to_string())
}

fn main() {
    let index = InvertedIndex::new();  // Removed mut
    
    // Add some initial documents
    match index.add_document("The quick brown fox") {
        Ok(id) => println!("Added document with ID: {}", id),
        Err(e) => println!("Error adding document: {}", e),
    }

    match index.add_document("The brown dog") {
        Ok(id) => println!("Added document with ID: {}", id),
        Err(e) => println!("Error adding document: {}", e),
    }
    
    loop {
        println!("\nAvailable commands:");
        println!("1. search <word>");
        println!("2. list");
        println!("3. clear");
        println!("4. quit");
        
        match get_input("Enter command: ") {
            Ok(input) => {
                if input == "quit" || input == "4" {
                    println!("Goodbye!");
                    break;
                }

                match input.as_str() {
                    "list" | "2" => {
                        match index.list_documents() {
                            Ok(docs) => {
                                println!("\nAll documents:");
                                for (id, content) in docs {
                                    println!("ID {}: {}", id, content);
                                }
                            }
                            Err(e) => println!("Error listing documents: {}", e),
                        }
                    }
                    "clear" | "3" => {
                        match index.clear() {
                            Ok(_) => println!("Index cleared successfully"),
                            Err(e) => println!("Error clearing index: {}", e),
                        }
                    }
                    _ => {
                        // Assume it's a search command
                        match index.search(&input) {
                            Ok(results) => {
                                if results.is_empty() {
                                    println!("No documents found containing '{}'", input);
                                } else {
                                    println!("Documents containing '{}': {:?}", input, results);
                                }
                            }
                            Err(e) => println!("Error searching: {}", e),
                        }
                    }
                }
            }
            Err(error) => {
                eprintln!("Error reading input: {}", error);
                break;
            }
        }
    }
}