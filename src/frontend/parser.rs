use crate::{
    grammar::{
        Grammar, HasOptions,
        Container, Endianness,
        Scheduling, Variable,
        VariableType, IntegerValue,
        VariableOptions, NumbersetType,
        NumbersetId,
    },
    frontend::{
        lexer::{Token, TokenId},
        source_view::{SourceRange, SourceView, NewRange},
        keywords,
    },
};
use std::ops::Range;

#[derive(Debug)]
pub enum ParserError {
    UnknownOptionValue(SourceRange),
    UnknownOptionName(SourceRange),
    DuplicateContainerName(SourceRange),
    EOF(String),
    UnexpectedToken(Option<usize>, String),
    InvalidKeyword(SourceRange, String),
}

struct TokenScanner<'a> {
    view: &'a SourceView,
    tokens: &'a [Token],
    cursor: usize,
}
impl<'a> TokenScanner<'a> {
    fn new(view: &'a SourceView, tokens: &'a [Token]) -> Self {
        Self {
            view,
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
    /*
    fn next(&mut self) -> Option<&'a Token> {
        if self.cursor < self.tokens.len() {
            let idx = self.cursor;
            self.cursor += 1;
            Some(&self.tokens[idx])
        } else {
    
    fn revert(&mut self) {
        if self.cursor > 0 {
            self.cursor -= 1;
        }
    }
            None
        }
    }
    */
    
    fn expect(&mut self, id: TokenId) -> Result<&'a Token, ParserError> {
        if self.cursor < self.tokens.len() {
            if self.tokens[self.cursor].id() != id {
                if let Some(pos) = self.tokens[self.cursor].pos() {
                    Err(ParserError::UnexpectedToken(
                        Some(pos),
                        format!("Expected {}", id.description())
                    ))
                } else {
                    Err(ParserError::UnexpectedToken(
                        None,
                        format!("Expected {} after {}", id.description(), self.tokens[self.cursor - 1].id().description())
                    ))
                }
            } else {
                let idx = self.cursor;
                self.cursor += 1;
                Ok(&self.tokens[idx])
            }
        } else {
            Err(ParserError::EOF(
                format!("Expected {}", id.description())
            ))
        }
    }
    
    //TODO: rename to peek
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
    
    fn get_source(&self, range: &SourceRange) -> &'a str {
        // The lexer ensures that range is in bounds
        self.view.range(&range)
    }
}

pub struct Parser<'a> {
    scanner: TokenScanner<'a>,
}
impl<'a> Parser<'a> {
    pub fn new(view: &'a SourceView, tokens: &'a [Token]) -> Self {
        Self {
            scanner: TokenScanner::new(view, tokens),
        }
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
        
        //TODO: root container
        
        Ok(grammar)
    }
    
    fn parse_options_list<T>(&mut self, dest: &mut T) -> Result<(), ParserError>
    where
        T: HasOptions
    {
        while let Some(token) = self.scanner.current() {
            match token {
                Token::OptionDef(_, key, value) => {
                    match self.scanner.get_source(key) {
                        keywords::OPTION_ENDIANNESS => {
                            let value = match self.scanner.get_source(value) {
                                "little" => Endianness::Little,
                                "big" => Endianness::Big,
                                "native" => Endianness::Native,
                                _ => {
                                    return Err(ParserError::UnknownOptionValue(value.clone()));
                                }
                            };
                            
                            dest.options_mut().set_endianness(value);
                        },
                        keywords::OPTION_SCHEDULING => {
                            let value = match self.scanner.get_source(value) {
                                "round-robin" => Scheduling::RoundRobin,
                                "random" => Scheduling::Random,
                                _ => {
                                    return Err(ParserError::UnknownOptionValue(value.clone()));
                                }
                            };
                            
                            dest.options_mut().set_scheduling(value);
                        },
                        _ => {
                            return Err(ParserError::UnknownOptionName(key.clone()));
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
        let name = match self.scanner.expect(TokenId::ContainerOpen)? {
            Token::ContainerOpen(_, name) => {
                /* check if name already exists */
                for container in grammar.containers() {
                    if let Some(other_name) = container.name() {
                        if self.scanner.get_source(&name) == self.scanner.get_source(other_name) {
                            return Err(ParserError::DuplicateContainerName(name.clone()));
                        }
                    }
                }
                
                name.clone()
            },
            _ => unreachable!(),
        };
        
        let id = grammar.reserve_container_id();
        let mut container = Container::new(id, Some(name));
        
        // After a container definition a block must be opened
        self.parse_block(grammar, &mut container)?;
        
        // After closing a block the container must end
        self.scanner.expect(TokenId::ContainerClose)?;
        
        Ok(container)
    }
    
    fn parse_block(&mut self, grammar: &mut Grammar, container: &mut Container) -> Result<(), ParserError> {
        self.scanner.expect(TokenId::BlockOpen)?;
        
        // Options may be overwritten in a block
        self.parse_options_list(container)?;
        
        // After options variables must follow
        while let Some(token) = self.scanner.current() {
            match token {
                Token::BlockClose => {
                    self.scanner.forward(1);
                    return Ok(());
                },
                
                Token::VariableStart(_) => {
                    let variable = self.parse_variable_definition(grammar)?;
                    container.add_variable(variable);
                },
                
                _ => {
                    // In order to get the best error message call expect()
                    self.scanner.expect(TokenId::VariableStart)?;
                    unreachable!();
                },
            }
        }
        
        Err(ParserError::EOF(
            "Block was not closed".to_string()
        ))
    }
    
    fn parse_variable_definition(&mut self, grammar: &mut Grammar) -> Result<Variable, ParserError> {
        self.scanner.expect(TokenId::VariableStart)?;
        
        // Parse variable options
        let mut had_optional = false;
        let mut had_repeats = false;
        let mut var_opts = VariableOptions::default();
        
        while let Some(token) = self.scanner.current() {
            match token {
                Token::VariableOptional(pos) => {
                    if had_optional {
                        return Err(ParserError::InvalidKeyword(
                            SourceRange::new(*pos, pos + keywords::VAROPT_OPTIONAL.len()),
                            "Multiple occurences of variable options not allowed".to_string(),
                        ));
                    }
                    
                    var_opts.set_optional();
                    had_optional = true;
                },
                Token::VariableRepeatStart(pos) => {
                    if had_repeats {
                        return Err(ParserError::InvalidKeyword(
                            SourceRange::new(*pos, pos + 1),
                            "Multiple occurences of variable options not allowed".to_string(),
                        ));
                    }
                    
                    self.scanner.forward(1);
                    let ranges = self.parse_numberset::<u32>(grammar, false)?;
                    todo!();
                    //var_opts.set_repeats(id);
                    had_repeats = true;
                },
                _ => {
                    break;
                },
            }
            
            self.scanner.forward(1);
        }
        
        let type_name = match self.scanner.expect(TokenId::VariableType)? {
            Token::VariableType(name) => {
                name.clone()
            },
            _ => unreachable!(),
        };
        
        let var_type = match self.scanner.current() {
            Some(Token::VariableValueStart(_)) => todo!(),
            Some(Token::BlockOpen(_)) => {
                // Check that the type name is 'oneof'
                if self.scanner.get_source(&type_name) != keywords::TYPE_ONEOF {
                    return Err(ParserError::InvalidKeyword(
                        type_name.clone(),
                        format!("Expected '{}'", keywords::TYPE_ONEOF),
                    ));
                }
                
                // Create container
                let id = grammar.reserve_container_id();
                let mut container = Container::new(id, None);
                self.parse_block(grammar, &mut container)?;
                
                // Create type
                grammar.add_container(container);
                VariableType::Oneof(id)
            },
            Some(Token::VariableEnd) => self.parse_variable_value_any(type_name)?,
            _ => unreachable!(),
        };
        
        self.scanner.expect(TokenId::VariableEnd)?;
        
        Ok(Variable::new(var_opts, var_type))
    }
    
    fn parse_variable_value_any(&mut self, type_name: SourceRange) -> Result<VariableType, ParserError> {
        match self.scanner.get_source(&type_name) {
            keywords::TYPE_U8 => Ok(VariableType::U8(IntegerValue::Any)),
            keywords::TYPE_I8 => Ok(VariableType::I8(IntegerValue::Any)),
            keywords::TYPE_U16 => Ok(VariableType::U16(IntegerValue::Any)),
            keywords::TYPE_I16 => Ok(VariableType::I16(IntegerValue::Any)),
            keywords::TYPE_U32 => Ok(VariableType::U32(IntegerValue::Any)),
            keywords::TYPE_I32 => Ok(VariableType::I32(IntegerValue::Any)),
            keywords::TYPE_U64 => Ok(VariableType::U64(IntegerValue::Any)),
            keywords::TYPE_I64 => Ok(VariableType::I64(IntegerValue::Any)),
            keywords::TYPE_ONEOF => Err(ParserError::InvalidKeyword(
                type_name.clone(),
                format!("Keyword '{}' is not allowed as a type name", keywords::TYPE_ONEOF)
            )),
            _ => Ok(VariableType::ResolveContainerRef(type_name)),
        }
    }
    
    fn parse_numberset<T>(&mut self, grammar: &mut Grammar, allow_chars: bool) -> Result<Vec<Range<T>>, ParserError> {
        self.scanner.expect(TokenId::NumbersetStart)?;
        
        let mut ranges = Vec::<Range<T>>::new();
        
        while let Some(token) = self.scanner.current() {
            match token {
                Token::NumbersetEnd => {
                    break;
                },
                Token::Integer(literal) => {
                    todo!();
                },
                _ => unreachable!(),
            }
            
            self.scanner.forward(1);
        }
        
        Ok(ranges)
    }
}
