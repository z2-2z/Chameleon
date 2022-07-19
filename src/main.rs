mod frontend;

//TODO: generic function for any error in any stage using SourceView

fn main() {
    let view = frontend::SourceView::from_file("all.chm");
    let mut lexer = frontend::Lexer::new(&view);
    
    let tokens = match lexer.lex() {
        Ok(tokens) => tokens,
        Err(_error) => {
            //lexer.describe_error(error);
            std::process::exit(1);
        },
    };
    
    for token in &tokens {
        println!("{:?}", token);
    }
    /*
    let mut parser = frontend::Parser::new(&buf, &tokens);
    
    let grammar = match parser.parse() {
        Ok(grammar) => grammar,
        Err(error) => {
            parser.describe_error(error);
            std::process::exit(1);
        },
    };
    */
}
