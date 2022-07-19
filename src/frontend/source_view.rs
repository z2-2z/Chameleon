use unicode_segmentation::UnicodeSegmentation;

/// Type alias that represents a slice of the SourceView
pub type SourceRange = std::ops::Range<usize>;

pub trait NewRange<Idx> {
    fn new(start: Idx, end: Idx) -> Self;
}

impl<Idx> NewRange<Idx> for std::ops::Range<Idx> {
    fn new(start: Idx, end: Idx) -> Self {
        Self {
            start,
            end
        }
    }
}

pub struct SourceView {
    content: String,
    length: usize,
}
impl SourceView {
    #[cfg(test)]
    pub fn new(content: &str) -> Self {
        Self {
            content: content.to_string(),
            length: UnicodeSegmentation::graphemes(content, true).count(),
        }
    }
    
    pub fn from_file(file: &str) -> Self {
        let content = std::fs::read(file).expect("Cannot read from file");
        let content = String::from_utf8(content).expect("Grammar is not valid UTF-8");
        let length = UnicodeSegmentation::graphemes(content.as_str(), true).count();
        
        Self {
            content,
            length,
        }
    }
    
    pub fn len(&self) -> usize {
        self.length
    }
    
    /// Return a substring of the file with `len` valid UTF-8 chars starting at character index `start`
    pub fn slice(&self, start: usize, len: usize) -> &str {
        if len == 0 {
            return "";
        }
        
        let mut iter = UnicodeSegmentation::grapheme_indices(self.content.as_str(), true).skip(start);
        
        let start_idx = if let Some((offset, _)) = iter.next() {
            offset
        } else {
            return "";
        };
        
        let end_idx = if let Some((offset, _)) = iter.skip(len - 1).next() {
            offset
        } else {
            self.content.len()
        };
        
        &self.content[start_idx..end_idx]
    }
    
    /*
    /// Convenience function that returns a slice from a SourceRange
    pub fn range(&self, range: &SourceRange) -> &str {
        self.slice(range.start, range.len())
    }
    */
    
    //TODO: lineinfo
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_ascii() {
        let view = SourceView::new("asdf\r\n");
        assert_eq!(view.len(), 5);
        
        /* request too many characters */
        assert_eq!(view.slice(0, 8), "asdf\r\n");
        
        /* request exact size */
        assert_eq!(view.slice(0, 5), "asdf\r\n");
        
        /* request substring */
        assert_eq!(view.slice(0, 1), "a");
        assert_eq!(view.slice(0, 2), "as");
        assert_eq!(view.slice(0, 3), "asd");
        assert_eq!(view.slice(0, 4), "asdf");
    }
    
    #[test]
    fn test_unicode() {
        let view = SourceView::new("a̐éö̲\r\n");
        assert_eq!(view.len(), 4);
        
        /* request starting at multi-codepoint char */
        assert_eq!(view.slice(2, 1), "ö̲");
        assert_eq!(view.slice(2, 2), "ö̲\r\n");
        
        /* request ending in multi-codepoint char */
        assert_eq!(view.slice(0, 3), "a̐éö̲");
        
        /* request exact size */
        assert_eq!(view.slice(0, 4), "a̐éö̲\r\n");
    }
}
