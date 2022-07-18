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
    
    fn subslice<F>(&self, cond: &mut F) -> Option<&'a [Token]>
    where
        F: FnMut(&'a Token) -> bool,
    {
        let mut len = 0;
        
        while self.cursor.saturating_add(len) < self.content.len() {
            if !cond(&self.tokens[self.cursor + len]) {
                len += 1;
                break;
            }
            
            len += 1;
        }
        
        if len == 0 {
            None
        } else {
            Some(&self.tokens[self.cursor..self.cursor + len])
        }
    }
    
    fn get_source(&self, range: &SourceRange) -> &'a [u8] {
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
        if let Some(option_defs) = self.scanner.subslice(&mut |t| t.is_option_def()) {
            for option_def in option_defs {
                let grammar_option = self.parse_option_def(option_def)?;
                grammar.add_global_option(grammar_option);
            }
        }
        
        Ok(grammar)
    }
    
    fn parse_option_def(&mut self, option_def: &Token) -> Result<GrammarOption, ParserError> {
        match option_def {
            Token::OptionDef(key, value) => {
                match self.scanner.get_source(key) {
                    keywords::OPTION_ENDIANNESS => {
                        match self.scanner.get_source(value) {
                            b"little" => Ok(GrammarOption::Endianness(Endianness::Little)),
                            b"big" => Ok(GrammarOption::Endianness(Endianness::Big)),
                            b"native" => Ok(GrammarOption::Endianness(Endianness::Native)),
                            _ => Err(ParserError::UnknownKeyword(value.clone()))
                        }
                    },
                    keywords::OPTION_SCHEDULING => {
                        match self.scanner.get_source(value) {
                            b"round-robin" => Ok(GrammarOption::Scheduling(Scheduling::RoundRobin)),
                            b"random" => Ok(GrammarOption::Scheduling(Scheduling::Random)),
                            _ => Err(ParserError::UnknownKeyword(value.clone())),
                        }
                    },
                    _ => {
                        Err(ParserError::UnknownOption(key.clone()))
                    },
                }
            },
            _ => unreachable!(),
        }
    }
}
