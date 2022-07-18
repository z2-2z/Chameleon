mod frontend;

fn main() {
    let buf = std::fs::read("test.chm").expect("testcase");
    let mut lexer = frontend::Lexer::new(&buf);
    
    let tokens = match lexer.lex() {
        Ok(tokens) => tokens,
        Err(error) => {
            lexer.describe_error(error);
            std::process::exit(1);
        },
    };
    
    for token in &tokens {
        lexer.describe_token(token);
    }
    
    let mut parser = frontend::Parser::new(&buf, &tokens);
    
    let grammar = match parser.parse() {
        Ok(grammar) => grammar,
        Err(error) => {
            todo!();
        },
    };
    
    
}
