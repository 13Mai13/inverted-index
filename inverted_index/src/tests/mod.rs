#[cfg(test)]
mod tests {
    use crate::index::inverted::InvertedIndex;

    #[test]
    fn test_empty_index() {
        let index = InvertedIndex::new();
        assert_eq!(index.search("any"), vec![]);
    }

    #[test]
    fn test_single_document() {
        let mut index = InvertedIndex::new();
        index.add_document(1, "The quick brown fox");
        
        assert_eq!(index.search("quick"), vec![1]);
        assert_eq!(index.search("fox"), vec![1]);
        assert_eq!(index.search("missing"), vec![]);
    }

    #[test]
    fn test_multiple_documents() {
        let mut index = InvertedIndex::new();
        index.add_document(1, "The quick brown fox");
        index.add_document(2, "The brown dog");
        
        assert_eq!(index.search("brown"), vec![1, 2]);
        assert_eq!(index.search("quick"), vec![1]);
        assert_eq!(index.search("dog"), vec![2]);
    }
}