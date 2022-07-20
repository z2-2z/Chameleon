use unicode_segmentation::UnicodeSegmentation;

/// Type alias that represents a slice of the SourceView
pub type SourceRange = std::ops::Range<usize>;

/// Trait that provides a new() method for Range
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


#[inline]
fn is_linebreak(s: &str) -> bool {
    #[cfg(target_os = "windows")]
    return s == "\r\n";
    
    #[cfg(any(target_os = "macos", target_os = "ios"))]
    return s == "\r";
    
    #[cfg(not(any(target_os = "windows", target_os = "macos", target_os = "ios")))]
    return s == "\n";
}

/// Given a grammar that is specified in valid UTF-8, this struct
/// offers some convenience functions to get slices of graphemes
/// as &str from the underlying string.
/// This enables us to implement parsers that operate on UTF-8 encoded
/// chars instead of raw bytes.
pub struct SourceView {
    content: String,
    length: usize,
}
impl SourceView {
    #[cfg(any(test, fuzzing))]
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
    
    /// Convenience function that returns a slice from a SourceRange
    pub fn range(&self, range: &SourceRange) -> &str {
        self.slice(range.start, range.len())
    }
    
    /// Return line number and column for the grapheme at index `pos`
    pub fn lineinfo(&self, pos: usize) -> (usize, usize) {
        let mut lineno = 1;
        let mut last_col = 1;
        let mut col = 1;
        
        for grapheme in UnicodeSegmentation::graphemes(self.content.as_str(), true).take(pos + 1) {
            if is_linebreak(grapheme) {
                lineno += 1;
                last_col = col;
                col = 0;
            }
            
            col += 1;
        }
        
        if col == 1 && lineno > 1 {
            col = last_col;
            lineno -= 1;
        } else if col > 1 {
            col -= 1;
        }
        
        (lineno, col)
    }
    
    /// Return the contents of a line (without line ending) or None
    /// if the line number is invalid.
    pub fn get_line(&self, req_line: usize) -> Option<&str> {
        let mut line_start = 0;
        let mut line_end = 0;
        let mut lineno = 1;
        
        if req_line == 0 {
            return None;
        }
        
        let mut graphemes = UnicodeSegmentation::grapheme_indices(self.content.as_str(), true);
        
        // Find start of line
        if lineno < req_line {
            while let Some((_, grapheme)) = graphemes.next() {
                if is_linebreak(grapheme) {
                    lineno += 1;
                }
                
                if lineno >= req_line {
                    break;
                }
            }
            
            match graphemes.next() {
                Some((offset, s)) => {
                    line_start = offset;
                    
                    // If the requested line is empty, return an empty string
                    if is_linebreak(s) {
                        return Some("");
                    }
                },
                None => {
                    return Some("");
                },
            }
        }
        
        // Find end of line
        while let Some((offset, grapheme)) = graphemes.next() {
            if is_linebreak(grapheme) {
                line_end = offset;
                break;
            }
        }
        
        if line_end == 0 && graphemes.next().is_none() {
            line_end = self.content.len();
        }
        
        // Return line without trailing linebreak
        if line_start <= line_end {
            Some(&self.content[line_start .. line_end])
        } else {
            Some("")
        }
    }
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
    
    #[test]
    fn test_lineinfo() {
        /* out of bounds */
        let view = SourceView::new("");
        assert_eq!(view.lineinfo(0), (1, 1));
        
        let view = SourceView::new("\n");
        assert_eq!(view.lineinfo(1), (1, 1));
        
        /* mid byte */
        let view = SourceView::new("\n\n");
        assert_eq!(view.lineinfo(0), (1, 1));
        assert_eq!(view.lineinfo(1), (2, 1));
    
        let view = SourceView::new("\nasdf\n");
        assert_eq!(view.lineinfo(2), (2, 2));
        assert_eq!(view.lineinfo(5), (2, 5));
        
        /* start byte */
        let view = SourceView::new("asdf");
        assert_eq!(view.lineinfo(0), (1, 1));
    }
    
    #[test]
    fn test_getline() {
        let view = SourceView::new("");
        assert_eq!(view.get_line(1), Some(""));
        assert_eq!(view.get_line(2), Some(""));
        
        let view = SourceView::new("asdf\n");
        assert_eq!(view.get_line(1), Some("asdf"));
        assert_eq!(view.get_line(2), Some(""));
        
        let view = SourceView::new("asdf\n\nasdf");
        assert_eq!(view.get_line(2), Some(""));
        assert_eq!(view.get_line(3), Some("asdf"));
    }
}
