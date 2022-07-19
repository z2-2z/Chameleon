use crate::frontend::{
    lexer::Token,
    grammar::*,
    range::SourceRange,
    keywords,
};

pub enum ParserError {
    UnknownOption(SourceRange),
    UnknownKeyword(SourceRange),
}

struct TokenScanner<'a> {
    content: &'a [u8],
    tokens: &'a [Token],
    cursor: usize,
}
impl<'a> TokenScanner<'a> {
    fn new(content: &'a [u8], tokens: &'a [Token]) -> Self {
        Self {
            content,
            tokens,
            cursor: 0,
        }
    }
    
    fn subslice<F>(&self, cond: &mut F) -> &'a [Token]
    where
        F: FnMut(&'a Token) -> bool,
    {
        let mut len = 0;
        
        while self.cursor.saturating_add(len) < self.tokens.len() {
            if !cond(&self.tokens[self.cursor + len]) {
                len += 1;
                break;
            }
            
            len += 1;
        }
        
        &self.tokens[self.cursor..self.cursor + len]
    }
    
    fn current(&self) -> Option<&'a Token> {
        if self.cursor < self.tokens.len() {
            Some(&self.tokens[self.cursor])
        } else {
            None
        }
    }
    
    fn forward(&mut self, len: usize) {
        if self.cursor < self.tokens.len() {
            self.cursor += len;
        }
    }
    
    fn get_source(&self, range: &SourceRange) -> &'a [u8] {
        // The lexer ensures that range is in bounds
        &self.content[range.start .. range.end]
    }
}

pub struct Parser<'a> {
    scanner: TokenScanner<'a>,
}
impl<'a> Parser<'a> {
    pub fn new(buf: &'a [u8], tokens: &'a [Token]) -> Self {
        Self {
            scanner: TokenScanner::new(buf, tokens),
        }
    }
    
    pub fn parse(&mut self) -> Result<Grammar, ParserError> {
        let mut grammar = Grammar::new();
        
        // Before any containers appear a user might define some global options
        self.parse_global_options(&mut grammar)?;
        
        // Now only containers may follow
        
        
        Ok(grammar)
    }
    
    fn parse_global_options(&mut self, grammar: &mut Grammar) -> Result<(), ParserError> {
        while let Some(token) = self.scanner.current() {
            match token {
                Token::OptionDef(key, value) => {
                    match self.scanner.get_source(key) {
                        keywords::OPTION_ENDIANNESS => {
                            let value = match self.scanner.get_source(value) {
                                b"little" => Endianness::Little,
                                b"big" => Endianness::Big,
                                b"native" => Endianness::Native,
                                _ => {
                                    return Err(ParserError::UnknownKeyword(value.clone()));
                                }
                            };
                            
                            grammar.options().set_endianness(value);
                        },
                        keywords::OPTION_SCHEDULING => {
                            let value = match self.scanner.get_source(value) {
                                b"round-robin" => Scheduling::RoundRobin,
                                b"random" => Scheduling::Random,
                                _ => {
                                    return Err(ParserError::UnknownKeyword(value.clone()));
                                }
                            };
                            
                            grammar.options().set_scheduling(value);
                        },
                        _ => {
                            return Err(ParserError::UnknownOption(key.clone()));
                        },
                    }
                },
                _ => {
                    break;
                }
            }
            
            self.scanner.forward(1);
        }
        
        Ok(())
    }
}
