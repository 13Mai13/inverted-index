use pyo3::prelude::*;
use pyo3::exceptions::PyRuntimeError;

// Import our Rust implementation
mod index;
use index::inverted::InvertedIndex;

/// PyInvertedIndex is the Python-facing wrapper around our Rust implementation
/// The #[pyclass] attribute makes this class available to Python
#[pyclass]
struct PyInvertedIndex {
    // Hold our Rust implementation
    index: InvertedIndex,
}

/// Implement Python-accessible methods
#[pymethods]
impl PyInvertedIndex {
    /// Constructor - creates a new index
    /// This will be called when Python code does: PyInvertedIndex()
    #[new]
    fn new() -> Self {
        PyInvertedIndex {
            index: InvertedIndex::new(),
        }
    }

    /// Add a document to the index
    /// Returns the document ID
    /// In Python: index.add_document("some text")
    fn add_document(&self, content: &str) -> PyResult<usize> {
        self.index
            .add_document(content)
            .map_err(|e| PyRuntimeError::new_err(e))  // Convert Rust error to Python exception
    }

    /// List all documents in the index
    /// Returns list of (id, content) tuples
    /// In Python: index.list_documents()
    fn list_documents(&self) -> PyResult<Vec<(usize, String)>> {
        self.index
            .list_documents()
            .map_err(|e| PyRuntimeError::new_err(e))
    }

    /// Search for documents containing a word
    /// Returns list of (id, content) tuples for matching documents
    /// In Python: index.search("word")
    fn search(&self, word: &str) -> PyResult<Vec<(usize, String)>> {
        self.index
            .search(word)
            .map_err(|e| PyRuntimeError::new_err(e))
    }

    /// Clear all documents from the index
    /// In Python: index.clear()
    fn clear(&self) -> PyResult<()> {
        self.index
            .clear()
            .map_err(|e| PyRuntimeError::new_err(e))
    }
}

/// Module initialization
/// This function is called when Python imports our module
#[pymodule]
fn inverted_index(_py: Python, m: &PyModule) -> PyResult<()> {
    m.add_class::<PyInvertedIndex>()?;  // Make our class available
    Ok(())
}