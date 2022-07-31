use crate::grammar::Grammar;

/// Statistics about a grammar
pub struct GrammarStats {
    num_paths: usize,
    max_input_size: usize,
    min_input_size: usize,
}

impl GrammarStats {
    pub fn from_grammar(grammar: &Grammar) -> Self {
        let root = *grammar.root().unwrap();
        
        Self {
            num_paths: grammar.num_paths(root),
            max_input_size: grammar.container_size(root, true),
            min_input_size: grammar.container_size(root, false),
        }
    }
    
    pub fn num_paths(&self) -> Option<usize> {
        if self.num_paths == usize::MAX {
            None
        } else {
            Some(self.num_paths)
        }
    }
    
    pub fn max_input_size(&self) -> Option<usize> {
        if self.max_input_size == usize::MAX {
            None
        } else {
            Some(self.max_input_size)
        }
    }
    
    pub fn min_input_size(&self) -> Option<usize> {
        if self.min_input_size == usize::MAX {
            None
        } else {
            Some(self.min_input_size)
        }
    }
}
