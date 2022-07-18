use crate::frontend::keywords;

macro_rules! string {
    ($kw:expr) => {
        std::str::from_utf8($kw).unwrap()
    };
}

/// Represents a continuous range [start, end)
pub struct Range {
    /// Inclusive start index
    start: usize,
    
    /// Exclusive end index
    end: usize,
}
impl Range {
    fn new(start: usize, end: usize) -> Self {
        Self {
            start,
            end
        }
    }
}

#[derive(Debug)]
pub enum LexerError {
    UnexpectedInput(String),
    UnexpectedEOF(String),
}

pub enum Token {
    /// Argument: Offset and size of identifier
    ContainerOpen(Range),
    ContainerClose,
    /// Arguments: range of identifier, range of value
    OptionDef(Range, Range),
    VariableStart,
    VariableEnd,
    VariableOptional,
    VariableRepeatStart,
    VariableRepeatEnd,
    NumbersetStart,
    NumbersetEnd,
    /// Arguments: lower limit, upper limit
    IntegerRange(Range, Range),
    /// Argument: integer string
    Integer(Range),
    /// Argument: literal contents
    Character(Range),
    /// Argument: type name
    VariableType(Range),
    /// Argument: string literal contents
    String(Range),
    VariableValueStart,
    VariableValueEnd,
    BlockOpen,
    BlockClose,
}

struct Scanner<'a> {
    content: &'a [u8],
    cursor: usize,
}
impl<'a> Scanner<'a> {
    fn new(content: &'a [u8]) -> Self {
        Self {
            content,
            cursor: 0,
        }
    }
    
    fn peek(&self, buf: &[u8]) -> bool {
        if self.cursor.saturating_add(buf.len()) <= self.content.len() {
            if &self.content[self.cursor..self.cursor + buf.len()] == buf {
                true
            } else {
                false
            }
        } else {
            false
        }
    }
    
    fn forward(&mut self, len: usize) {
        if self.cursor < self.content.len() {
            self.cursor += len;
        }
    }
    
    fn skip<F>(&mut self, func: &mut F) -> usize
    where
        F: FnMut(u8) -> bool,
    {
        let mut skipped = 0;
        
        while self.cursor + skipped < self.content.len() {
            if func(self.content[self.cursor + skipped]) {
                skipped += 1;
            } else {
                break;
            }
        }
        
        self.forward(skipped);
        skipped
    }
    
    fn expect(&mut self, buf: &[u8]) -> Result<(), LexerError> {
        if self.peek(buf) {
            self.forward(buf.len());
            Ok(())
        } else {
            Err(LexerError::UnexpectedInput(
                format!("Expected '{}'", string!(buf))
            ))
        }
    }
    
    fn done(&self) -> bool {
        self.cursor >= self.content.len()
    }
    
    /// Returns line number and column for a given cursor position
    pub fn line_info(&self, cursor: usize) -> (usize, usize) {
        let mut line = 1;
        let mut col = 1;
        let cursor = std::cmp::min(cursor, self.content.len());
        
        //TODO: os-sensitive line ending detection
        
        for i in 0..cursor {
            if self.content[i] == b'\n' {
                line += 1;
                col = 0;
            }
            
            col += 1;
        }
        
        (line, col)
    }
    
    fn substring(&self, range: &Range) -> Option<&[u8]> {
        if range.start < self.content.len() && range.end <= self.content.len() {
            Some(&self.content[range.start .. range.end])
        } else {
            None
        }
    }
}

#[inline]
fn is_whitespace(c: u8) -> bool {
    c == b' ' || c == b'\r' || c == b'\n' || c == b'\t'
}

/// An identifier is [0-9a-zA-Z_]+
#[inline]
fn is_identifier(c: u8) -> bool {
    (c >= 0x30 && c < 0x3a) || (c >= 0x41 && c < 0x5B) || (c >= 0x61 && c < 0x7B) || c == b'_'
}

/// Charset for an option value: all chars except whitespaces and ';'
#[inline]
fn is_option_value(c: u8) -> bool {
    (c > 0x20 && c < 0x3B) || (c > 0x3B && c < 0x7F)
}

/// charset for signed integers + prefix characters 0x 0o 0b
#[inline]
fn is_integer(c: u8) -> bool {
    (c >= 0x30 && c < 0x3A) || c == b'-' || c == b'x' || c == b'o' || c == b'b'
}

/// charset for chars: everything printable, escape sequences are allowed!
#[inline]
fn is_char(c: u8) -> bool {
    c >= 0x20 && c < 0x7F
}

pub struct Lexer<'a> {
    scanner: Scanner<'a>,
}
impl<'a> Lexer<'a> {
    pub fn new(content: &'a [u8]) -> Self {
        Self {
            scanner: Scanner::new(content),
        }
    }
    
    pub fn describe_token(&self, token: &Token) {
        match token {
            Token::OptionDef(key, value) => {
                println!("Option({} = {})", string!(self.scanner.substring(key).unwrap()), string!(self.scanner.substring(value).unwrap()));
            },
            Token::ContainerOpen(name) => {
                println!("ContainerOpen({})", string!(&self.scanner.substring(name).unwrap()));
            },
            Token::ContainerClose => {
                println!("ContainerClose");
            },
            Token::VariableStart => {
                println!("VariableStart");
            },
            Token::VariableEnd => {
                println!("VariableEnd");
            },
            Token::VariableOptional => {
                println!("VariableOptional");
            },
            Token::VariableRepeatStart => {
                println!("VariableRepeatStart");
            },
            Token::VariableRepeatEnd => {
                println!("VariableRepeatEnd");
            },
            Token::NumbersetStart => {
                println!("NumbersetStart");
            },
            Token::NumbersetEnd => {
                println!("NumbersetEnd");
            },
            Token::Character(literal) => {
                println!("Character({})", string!(self.scanner.substring(literal).unwrap()));
            },
            Token::Integer(literal) => {
                println!("Integer({})", string!(self.scanner.substring(literal).unwrap()));
            },
            Token::IntegerRange(lower, upper) => {
                println!("IntegerRange({} .. {})", string!(self.scanner.substring(lower).unwrap()), string!(self.scanner.substring(upper).unwrap()));
            },
            Token::VariableType(name) => {
                println!("VariableType({})", string!(self.scanner.substring(name).unwrap()));
            },
            Token::String(literal) => {
                println!("String({})", string!(self.scanner.substring(literal).unwrap()));
            },
            Token::VariableValueStart => {
                println!("VariableValueStart");
            },
            Token::VariableValueEnd => {
                println!("VariableValueEnd");
            },
            Token::BlockOpen => {
                println!("BlockOpen");
            },
            Token::BlockClose => {
                println!("BlockClose");
            },
        }
    }
    
    pub fn describe_error(&self, error: LexerError) {
        // use scanner and other data to add some context
        // information to an error message
        let (line, col) = self.scanner.line_info(self.scanner.cursor);
        println!("{:?} at line {} col {}", error, line, col);
    }
    
    pub fn lex(&mut self) -> Result<Vec<Token>, LexerError> {
        let mut tokens = Vec::<Token>::new();
        
        while !self.scanner.done() {
            // Is it a container ?
            if self.scanner.peek(keywords::CONTAINER) {
                self.parse_container(&mut tokens)?;
            }
            // Is it an option ?
            else if self.scanner.peek(keywords::OPTION) {
                self.parse_option(&mut tokens)?;
            }
            // Is it a comment ?
            else if self.scanner.peek(keywords::COMMENT_OPEN) {
                self.parse_comment()?;
            }
            // then it must be whitespace
            else if self.scanner.skip(&mut is_whitespace) == 0 {
                return Err(LexerError::UnexpectedInput(
                    format!("Expected '{}' or '{}' or whitespace or comment", string!(keywords::CONTAINER), string!(keywords::OPTION))
                ));
            }
        }
        
        if tokens.is_empty() {
            Err(LexerError::UnexpectedEOF(
                format!("Expected '{}' or '{}'", string!(keywords::CONTAINER), string!(keywords::OPTION))
            ))
        } else {
            Ok(tokens)
        }
    }
    
    fn parse_comment(&mut self) -> Result<(), LexerError> {
        self.scanner.expect(keywords::COMMENT_OPEN)?;
        
        while !self.scanner.done() {
            // Is it a nested comment ?
            if self.scanner.peek(keywords::COMMENT_OPEN) {
                self.parse_comment()?;
            }
            // Is it a comment close ?
            else if self.scanner.peek(keywords::COMMENT_CLOSE) {
                self.scanner.forward(keywords::COMMENT_CLOSE.len());
                return Ok(());
            }
            // It is a normal char
            else {
                self.scanner.forward(1);
            }
        }
        
        Err(LexerError::UnexpectedEOF(
            format!("Expected end of comment '{}'", string!(keywords::COMMENT_CLOSE))
        ))
    }
    
    fn parse_option(&mut self, tokens: &mut Vec<Token>) -> Result<(), LexerError> {
        let identifier_start;
        let identifier_end;
        let value_start;
        let value_end;
        let option_start = self.scanner.cursor;
        let option_end;
        
        self.scanner.expect(keywords::OPTION)?;
        
        // At least one whitespace required after keyword
        if self.scanner.skip(&mut is_whitespace) == 0 {
            return Err(LexerError::UnexpectedInput(
                format!("Expected whitespace after '{}'", string!(keywords::OPTION))
            ));
        }
        
        // After whitespaces an identifier follows
        identifier_start = self.scanner.cursor;
        identifier_end = match self.scanner.skip(&mut is_identifier) {
            0 => {
                return Err(LexerError::UnexpectedInput(
                    format!("Expected an identifier after '{}'", string!(keywords::OPTION))
                ));
            },
            len @ _ => identifier_start + len,
        };
        
        // After the identifier whitespaces may follow
        self.scanner.skip(&mut is_whitespace);
        
        // ... until we hit an equals sign
        self.scanner.expect(keywords::ASSIGNMENT)?;
        
        // after an equals sign whitespaces may follow again
        self.scanner.skip(&mut is_whitespace);
        
        // ... until we hit the value of an option
        value_start = self.scanner.cursor;
        value_end = match self.scanner.skip(&mut is_option_value) {
            0 => {
                return Err(LexerError::UnexpectedInput(
                    format!("Expected value after '{}'", string!(keywords::ASSIGNMENT))
                ));
            },
            len @ _ => value_start + len,
        };
        
        // after the value whitespaces may follow
        self.scanner.skip(&mut is_whitespace);
        
        // and an option definition ends with ';'
        self.scanner.expect(keywords::TERMINATE_STATEMENT)?;
        
        option_end = self.scanner.cursor;
        
        if option_end > option_start && value_end > value_start && identifier_end > identifier_start {
            tokens.push(
                Token::OptionDef(
                    Range::new(identifier_start, identifier_end),
                    Range::new(value_start, value_end),
                )
            );
        }
        
        Ok(())
    }
    
    fn parse_container(&mut self, tokens: &mut Vec<Token>) -> Result<(), LexerError> {
        let name_start;
        let name_end;
        
        self.scanner.expect(keywords::CONTAINER)?;
        
        // after the keyword whitespaces may follow
        if self.scanner.skip(&mut is_whitespace) < 1 {
            return Err(LexerError::UnexpectedInput(
                format!("Expected whitespace after '{}'", string!(keywords::CONTAINER))
            ));
        }
        
        // after the whitespaces a container name follows
        name_start = self.scanner.cursor;
        name_end = match self.scanner.skip(&mut is_identifier) {
            0 => {
                return Err(LexerError::UnexpectedInput(
                    format!("Expected name after '{}'", string!(keywords::CONTAINER))
                ));
            },
            len @ _ => name_start + len,
        };
        
        tokens.push(
            Token::ContainerOpen(Range::new(name_start, name_end))
        );
        
        // after the name whitespaces may follow
        self.scanner.skip(&mut is_whitespace);
        
        // after the container name a block must be opened
        self.parse_block(tokens)?;
        
        tokens.push(Token::ContainerClose);
        
        Ok(())
    }
    
    fn parse_block(&mut self, tokens: &mut Vec<Token>) -> Result<(), LexerError> {
        self.scanner.expect(keywords::BLOCK_OPEN)?;
        
        tokens.push(Token::BlockOpen);
        
        self.parse_variable_listing(tokens)?;
        
        // Before a block delimiter whitespaces may precede
        self.scanner.skip(&mut is_whitespace);
        
        // A block delimiter must close the container
        self.scanner.expect(keywords::BLOCK_CLOSE)?;
        
        tokens.push(Token::BlockClose);
        
        Ok(())
    }
    
    fn parse_variable_listing(&mut self, tokens: &mut Vec<Token>) -> Result<(), LexerError> {
        while !self.scanner.done() {
            self.scanner.skip(&mut is_whitespace);
            
            // Is it the end of a block, then return
            if self.scanner.peek(keywords::BLOCK_CLOSE) {
                return Ok(());
            }
            // Is it a local option ?
            else if self.scanner.peek(keywords::OPTION) && !self.scanner.peek(keywords::VAROPT_OPTIONAL) {
                self.parse_option(tokens)?;
            }
            // Is it a comment ?
            else if self.scanner.peek(keywords::COMMENT_OPEN) {
                self.parse_comment()?;
            }
            // Otherwise it must be a variable definition
            else {
                self.parse_variable_definition(tokens)?;
            }
        }
        
        Err(LexerError::UnexpectedEOF(
            "Variable listing was not closed".to_string()
        ))
    }
    
    fn parse_variable_definition(&mut self, tokens: &mut Vec<Token>) -> Result<(), LexerError> {
        tokens.push(Token::VariableStart);
        
        // Variables may start with options
        while !self.scanner.done() {
            // variable optional ?
            if self.scanner.peek(keywords::VAROPT_OPTIONAL) {
                tokens.push(Token::VariableOptional);
                self.scanner.forward(keywords::VAROPT_OPTIONAL.len());
                
                // white space must follow the keyword
                if self.scanner.skip(&mut is_whitespace) == 0 {
                    return Err(LexerError::UnexpectedInput(
                        format!("Expected whitespace after '{}'", string!(keywords::VAROPT_REPEATS))
                    ));
                }
            } 
            // variable repeatable ?
            else if self.scanner.peek(keywords::VAROPT_REPEATS) {
                self.scanner.forward(keywords::VAROPT_REPEATS.len());
                
                // white space must follow the keyword
                if self.scanner.skip(&mut is_whitespace) == 0 {
                    return Err(LexerError::UnexpectedInput(
                        format!("Expected whitespace after '{}'", string!(keywords::VAROPT_REPEATS))
                    ));
                }
                
                tokens.push(Token::VariableRepeatStart);
                
                self.parse_numberset(tokens)?;
                
                tokens.push(Token::VariableRepeatEnd);
                
                // white space must follow after numberset
                if self.scanner.skip(&mut is_whitespace) == 0 {
                    return Err(LexerError::UnexpectedInput(
                        "Expected whitespace after numberset".to_string()
                    ));
                }
            }
            // No known option, assume its a variable name
            else {
                break;
            }
        }
        
        // after variable options comes the variable name, but we ignore
        // the concrete value of the name. It is just a help for the developer
        if self.scanner.skip(&mut is_identifier) == 0 {
            return Err(LexerError::UnexpectedInput(
                "Expected variable name".to_string()
            ));
        }
        
        // white space may follow after name
        self.scanner.skip(&mut is_whitespace);
        
        // A type separator is expected
        self.scanner.expect(keywords::VAR_TYPE_SEP)?;
        
        // Optionally whitespaces may follow the type separator
        self.scanner.skip(&mut is_whitespace);
        
        // Then a typename must follow
        let type_start = self.scanner.cursor;
        let type_end = match self.scanner.skip(&mut is_identifier) {
            0 => {
                return Err(LexerError::UnexpectedInput(
                    format!("Expected type name after '{}'", string!(keywords::VAR_TYPE_SEP))
                ));
            },
            len @ _ => type_start + len,
        };
        
        tokens.push(Token::VariableType(Range::new(type_start, type_end)));
        
        // Optionally whitespaces may follow the type
        self.scanner.skip(&mut is_whitespace);
        
        // After the type we may have an assignment with '='
        if self.scanner.peek(keywords::ASSIGNMENT) {
            self.scanner.forward(keywords::ASSIGNMENT.len());
            
            tokens.push(Token::VariableValueStart);
            
            self.scanner.skip(&mut is_whitespace);
            
            // After an equals sign we either expect a string literal or a numberset
            if self.scanner.peek(keywords::STRING_DELIM) {
                self.parse_string_literal(tokens)?;
            } else {
                self.parse_numberset(tokens)?;
            }
            
            tokens.push(Token::VariableValueEnd);
            
            self.scanner.skip(&mut is_whitespace);
        }
        // or we may have a new block
        else if self.scanner.peek(keywords::BLOCK_OPEN) {
            self.parse_block(tokens)?;
            
            // after a block whitespaces may follow
            self.scanner.skip(&mut is_whitespace);
        }
        
        self.scanner.expect(keywords::TERMINATE_STATEMENT)?;
        
        tokens.push(Token::VariableEnd);
        
        Ok(())
    }
    
    fn parse_numberset(&mut self, tokens: &mut Vec<Token>) -> Result<(), LexerError> {
        tokens.push(Token::NumbersetStart);
        
        while !self.scanner.done() {
            // Do we have a simple char ?
            if self.scanner.peek(b"'") {
                self.scanner.forward(1);
                let mut seen_backslash = false;
                
                let char_start = self.scanner.cursor;
                let char_end = match self.scanner.skip(&mut |c| {
                    if c == b'\'' {
                        let ret = seen_backslash;
                        seen_backslash = false;
                        ret
                    } else if seen_backslash {
                        seen_backslash = false;
                        true
                    } else if c == b'\\' {
                        seen_backslash = true;
                        true
                    } else {
                        is_char(c)
                    }
                }) {
                    1 => char_start + 1,
                    2 => char_start + 2,
                    _ => {
                        return Err(LexerError::UnexpectedInput(
                            format!("Expected char constant after '")
                        ));
                    },
                };
                
                self.scanner.forward(1);
                
                tokens.push(Token::Character(Range::new(char_start, char_end)));
            }
            // Otherwise we must have a number
            else {
                let number_start = self.scanner.cursor;
                let number_end = match self.scanner.skip(&mut is_integer) {
                    0 => {
                        return Err(LexerError::UnexpectedInput(
                            "Expected numberset".to_string()
                        ));
                    },
                    len @ _ => number_start + len,
                };
                
                // Is this a number range ?
                if self.scanner.peek(keywords::RANGE_OP) {
                    self.scanner.forward(keywords::RANGE_OP.len());
                    
                    let limit_start = self.scanner.cursor;
                    let limit_end = match self.scanner.skip(&mut is_integer) {
                        0 => {
                            return Err(LexerError::UnexpectedInput(
                                format!("Expected number after '{}'", string!(keywords::RANGE_OP))
                            ));
                        },
                        len @ _ => limit_start + len,
                    };
                    
                    tokens.push(Token::IntegerRange(
                        Range::new(number_start, number_end),
                        Range::new(limit_start, limit_end),
                    ));
                }
                // This is a single number
                else {
                    tokens.push(Token::Integer(Range::new(number_start, number_end)));
                }
            }
            
            // If we have a ',', parse again
            if self.scanner.peek(keywords::NUMBERSET_DELIM) {
                self.scanner.forward(keywords::NUMBERSET_DELIM.len());
            } else {
                break;
            }
        }
        
        tokens.push(Token::NumbersetEnd);
        
        Ok(())
    }
    
    fn parse_string_literal(&mut self, tokens: &mut Vec<Token>) -> Result<(), LexerError> {
        self.scanner.expect(keywords::STRING_DELIM)?;
        
        let mut seen_backslash = false;
        let string_start = self.scanner.cursor;
        let string_end = string_start + self.scanner.skip(&mut |c| {
            if c == keywords::STRING_DELIM[0] {
                let ret = seen_backslash;
                seen_backslash = false;
                ret
            } else if seen_backslash {
                seen_backslash = false;
                true
            } else if c == b'\\' {
                seen_backslash = true;
                true
            } else {
                is_char(c)
            }
        });
        
        tokens.push(Token::String(Range::new(string_start, string_end)));
        
        self.scanner.expect(keywords::STRING_DELIM)?;
        
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::Lexer;
    
    #[test]
    #[should_panic]
    fn unclosed_comment() {
        let input = "/*";
        Lexer::new(input.as_bytes()).lex().unwrap();
    }
    
    #[test]
    fn empty_comment() {
        let input = "/**/struct x{}";
        Lexer::new(input.as_bytes()).lex().unwrap();
    }
    
    #[test]
    fn valid_nested_comment() {
        let input = "/*/**/*/struct x{}";
        Lexer::new(input.as_bytes()).lex().unwrap();
    }
    
    #[test]
    #[should_panic]
    fn unclosed_nested_comment() {
        let input = "/*/**/struct x{}";
        Lexer::new(input.as_bytes()).lex().unwrap();
    }
    
    #[test]
    #[should_panic]
    fn double_closed_comment() {
        let input = "/**/*/struct x{}";
        Lexer::new(input.as_bytes()).lex().unwrap();
    }
    
    #[test]
    fn comment_at_block_start() {
        let input = "struct x{/**/}";
        Lexer::new(input.as_bytes()).lex().unwrap();
    }
    
    #[test]
    fn comment_at_block_end() {
        let input = "struct x{x:y;/**/}";
        Lexer::new(input.as_bytes()).lex().unwrap();
    }
    
    #[test]
    fn comment_inbetween_vars() {
        let input = "struct x{x:y;/**/x:y;}";
        Lexer::new(input.as_bytes()).lex().unwrap();
    }
    
    #[test]
    #[should_panic]
    fn option_no_assignment() {
        let input = "option key;";
        Lexer::new(input.as_bytes()).lex().unwrap();
    }
    
    #[test]
    #[should_panic]
    fn option_no_value() {
        let input = "option key=;";
        Lexer::new(input.as_bytes()).lex().unwrap();
    }
    
    #[test]
    fn option_valid() {
        let input = "option key=value;";
        Lexer::new(input.as_bytes()).lex().unwrap();
    }
    
    #[test]
    #[should_panic]
    fn option_reject_key() {
        let input = "option x!y=value;";
        Lexer::new(input.as_bytes()).lex().unwrap();
    }
    
    #[test]
    fn valid_number_variable() {
        let input = "struct x{x:u16=3;}";
        Lexer::new(input.as_bytes()).lex().unwrap();
    }
    
    #[test]
    fn valid_numberset() {
        let input = "struct x{x:u16=-8,-3,0..1,101;}";
        Lexer::new(input.as_bytes()).lex().unwrap();
    }
    
    #[test]
    fn valid_nonsense_numberset() {
        let input = "struct x{x:u16=-,-,0..0,'\\X';}";
        Lexer::new(input.as_bytes()).lex().unwrap();
    }
    
    #[test]
    fn valid_numberset_chars() {
        let input = "struct x{x:u16='A','SS','D','FF';}";
        Lexer::new(input.as_bytes()).lex().unwrap();
    }
    
    #[test]
    #[should_panic]
    fn invalid_numberset_chars() {
        let input = "struct x{x:u16='AAAA','SSSS','DDDD','FFFF';}";
        Lexer::new(input.as_bytes()).lex().unwrap();
    }
    
    #[test]
    fn empty_string_literal() {
        let input = "struct x{x:string=\"\";}";
        Lexer::new(input.as_bytes()).lex().unwrap();
    }
    
    #[test]
    fn valid_string_literal() {
        let input = "struct x{x:string=\"\\\" hello world! \\xCC\\x0d\";}";
        Lexer::new(input.as_bytes()).lex().unwrap();
    }
    
    #[test]
    fn duplicate_variable_flags() {
        let input = "struct x{optional optional optional repeats 3 repeats 4 x:x;}";
        Lexer::new(input.as_bytes()).lex().unwrap();
    }
    
    #[test]
    fn multiple_structs() {
        let input = "struct x{}struct x{}";
        Lexer::new(input.as_bytes()).lex().unwrap();
    }
    
    #[test]
    fn test_nested_blocks() {
        let input = "struct x{x:y{x:y{x:y;};};}";
        Lexer::new(input.as_bytes()).lex().unwrap();
    }
    
    #[test]
    fn test_number_formats() {
        let input = "struct x{x:x=0x01..0o77,0b10101;}";
        Lexer::new(input.as_bytes()).lex().unwrap();
    }
    
    #[test]
    fn valid_grammar() {
        let input = "
option endianness = little;

struct ELFIdent {
    magic: string = \"\\x7FELF\";
    class: u8 = 1,2;
    data: u8 = 1,2;
    version: u8 = 1;
    osabi: u8 = 0x00..0x04,0x06..0x12;
    abiversion: u8 = 0;
    repeats 7 pad: u8;
}

struct Root {
    ident: ELFIdent;
}        
";
        Lexer::new(input.as_bytes()).lex().unwrap();
    }
}
