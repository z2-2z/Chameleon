use crate::frontend::{
    lexer::Token,
    grammar::*,
    source_view::SourceRange,
    keywords,
};

#[derive(Debug)]
pub enum ParserError {
    UnknownOption(SourceRange),
    UnknownKeyword(SourceRange),
    UnexpectedToken(String),
    UnexpectedEOF,
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
    /*
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
    */
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
    
    fn done(&self) -> bool {
        self.cursor >= self.tokens.len()
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
    
    pub fn describe_error(&self, error: ParserError) {
        println!("{:?}", error);
    }
    
    pub fn parse(&mut self) -> Result<Grammar, ParserError> {
        let mut grammar = Grammar::new();
        
        // Before any containers appear a user might define some global options
        self.parse_options_list(&mut grammar)?;
        
        // Now only containers may follow
        while !self.scanner.done() {
            let container = self.parse_container(&mut grammar)?;
            grammar.add_container(container);
        }
        
        //TODO: Verifying: no name duplicates and root container
        
        Ok(grammar)
    }
    
    fn parse_options_list<T>(&mut self, dest: &mut T) -> Result<(), ParserError>
    where
        T: HasOptions
    {
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
                            
                            dest.options_mut().set_endianness(value);
                        },
                        keywords::OPTION_SCHEDULING => {
                            let value = match self.scanner.get_source(value) {
                                b"round-robin" => Scheduling::RoundRobin,
                                b"random" => Scheduling::Random,
                                _ => {
                                    return Err(ParserError::UnknownKeyword(value.clone()));
                                }
                            };
                            
                            dest.options_mut().set_scheduling(value);
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
    
    fn parse_container(&mut self, grammar: &mut Grammar) -> Result<Container, ParserError> {
        let id = grammar.reserve_container_id();
        let name = match self.scanner.current() {
            Some(Token::ContainerOpen(name)) => name.clone(),
            _ => {
                return Err(ParserError::UnexpectedToken(
                    "Expected container".to_string(),
                ));
            },
        };
        let mut container = Container::new(id, Some(name));
        
        self.scanner.forward(1);
        
        // After a container def a block must be opened
        if let Some(token) = self.scanner.current() {
            if *token != Token::BlockOpen {
                return Err(ParserError::UnexpectedToken(
                    "Expected opening of block".to_string()
                ));
            }
        }
        
        self.scanner.forward(1);
        
        // Options may be overwritten in a block
        self.parse_options_list(&mut container)?;
        
        // After options variables must follow
        while let Some(token) = self.scanner.current() {
            match token {
                Token::BlockClose => {
                    self.scanner.forward(1);
                    break;
                },
                
                _ => {
                    return Err(ParserError::UnexpectedToken(
                        "Expected ...".to_string(),
                    ));
                },
            }
            
            self.scanner.forward(1);
        }
        
        // After closing a block the container must end
        if let Some(token) = self.scanner.current() {
            if *token != Token::ContainerClose {
                return Err(ParserError::UnexpectedToken(
                    "Expected end of container".to_string()
                ));
            }
        }
        
        self.scanner.forward(1);
        
        Ok(container)
    }
}
