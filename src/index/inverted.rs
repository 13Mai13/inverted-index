use std::collections::HashMap;
use std::sync::Mutex; // Official documentation here: https://doc.rust-lang.org/std/sync/struct.Mutex.html

#[derive(Debug)]
pub struct InvertedIndex { // Maps words to document IDs: {"word" -> [1, 2, 3]}
    pub index: Mutex<HashMap<String, Vec<usize>>>, // Stores the actual documents: {1 -> "document content"}
    pub documents: Mutex<HashMap<usize, String>>,
    pub next_doc_id: Mutex<usize>, // Keeps track of the next available document ID
}

impl InvertedIndex {
    /// Creates a new, empty index
    pub fn new() -> Self {
        InvertedIndex {
            index: Mutex::new(HashMap::new()),
            documents: Mutex::new(HashMap::new()),
            next_doc_id: Mutex::new(1), // Start document IDs at 1
        }
    }


    /// Adds a new document to the index and returns its ID
    /// The document is split into words, and each word is mapped to this document's ID
    pub fn add_document(&self, content: &str) -> Result<usize, String> {
        // Get and increment the next available document ID
        let mut next_doc_id = self.next_doc_id.lock().map_err(|e| e.to_string())?;
        let doc_id = *next_doc_id;
        *next_doc_id += 1;

        // Store the full document content
        let mut documents = self.documents.lock().map_err(|e| e.to_string())?;
        documents.insert(doc_id, content.to_string());

        // Process each word in the document
        let mut index = self.index.lock().map_err(|e| e.to_string())?;
        for word in content.split_whitespace() {
            // Convert to lowercase for case-insensitive search
            let word = word.to_lowercase();
            // Add this document ID to the list of documents containing this word
            index.entry(word)
                .or_insert_with(Vec::new)
                .push(doc_id);
        }

        Ok(doc_id)
    }

    /// Returns a list of all documents in the index
    /// Returns pairs of (document_id, content)
    pub fn list_documents(&self) -> Result<Vec<(usize, String)>, String> {
        let documents = self.documents.lock().map_err(|e| e.to_string())?;
        // Convert the HashMap into a Vec of tuples
        let docs: Vec<(usize, String)> = documents
            .iter()
            .map(|(&id, content)| (id, content.clone()))
            .collect();
        Ok(docs)
    }

    /// Searches for documents containing the given word
    /// Returns pairs of (document_id, content) for matching documents
    pub fn search(&self, word: &str) -> Result<Vec<(usize, String)>, String> {
        // Lock both index and documents for consistent read
        let index = self.index.lock().map_err(|e| e.to_string())?;
        let documents = self.documents.lock().map_err(|e| e.to_string())?;
        
        // Find all document IDs containing this word
        let doc_ids = index
            .get(&word.to_lowercase())
            .cloned()
            .unwrap_or_default();

        // Get the full content for each matching document
        let results: Vec<(usize, String)> = doc_ids
            .into_iter()
            .filter_map(|id| {
                documents
                    .get(&id)
                    .map(|content| (id, content.clone()))
            })
            .collect();

        Ok(results)
    }

    /// Clears all data from the index
    pub fn clear(&self) -> Result<(), String> {
        // Lock all data structures
        let mut index = self.index.lock().map_err(|e| e.to_string())?;
        let mut documents = self.documents.lock().map_err(|e| e.to_string())?;
        let mut next_doc_id = self.next_doc_id.lock().map_err(|e| e.to_string())?;

        // Clear everything
        index.clear();
        documents.clear();
        *next_doc_id = 1;  // Reset document ID counter

        Ok(())
    }
}
