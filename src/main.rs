use termcolor;
use termcolor::WriteColor;
use std::io::Write;

mod frontend;

fn get_decimal_length(mut lineno: usize) -> usize {
    let mut len = 0;
    
    while lineno > 0 {
        lineno /= 10;
        len += 1;
    }
    
    len
}

fn print_line_context(stream: &mut termcolor::StandardStream, view: &frontend::SourceView, lineno: usize, col: usize, len: usize) -> Result<(), std::io::Error> {
    let line_length = std::cmp::max(
        get_decimal_length(lineno - 1),
        std::cmp::max(
            get_decimal_length(lineno),
            get_decimal_length(lineno + 1),
        )
    );
    let mut dimmed = termcolor::ColorSpec::new();
    dimmed.set_bg(None);
    dimmed.set_dimmed(true);
    let mut slim_red = termcolor::ColorSpec::new();
    slim_red.set_bg(None);
    slim_red.set_bold(false);
    slim_red.set_fg(Some(termcolor::Color::Red));
    
    if lineno > 1 {
        if let Some(line) = view.get_line(lineno - 1) {
            write!(stream, " {:width$} | ", lineno - 1, width = line_length)?;
            stream.set_color(&dimmed)?;
            writeln!(stream, "{}", line)?;
            stream.reset()?;
        }
    }
    
    if let Some(line) = view.get_line(lineno) {
        writeln!(stream, " {:width$} | {}", lineno, line, width = line_length)?;
        
        // Mark the columns affected
        write!(stream, " {0:width$} | {0:skip_cols$}", "",  width = line_length, skip_cols = col - 1)?;
        stream.set_color(&slim_red)?;
        writeln!(stream, "{0:^<input_len$}", "", input_len = len)?;
        stream.reset()?;
    } else {
        writeln!(stream, "<could not get line info>")?;
    }
    
    if let Some(line) = view.get_line(lineno + 1) {
        write!(stream, " {:width$} | ", lineno + 1, width = line_length)?;
        stream.set_color(&dimmed)?;
        writeln!(stream, "{}", line)?;
        stream.reset()?;
    }
    
    Ok(())
}

fn print_lexing_error(view: &frontend::SourceView, error: frontend::LexerError) -> Result<(), std::io::Error> {
    let mut bold_red = termcolor::ColorSpec::new();
    bold_red.set_bg(None);
    bold_red.set_bold(true);
    bold_red.set_fg(Some(termcolor::Color::Red));
    let mut slim_red = termcolor::ColorSpec::new();
    slim_red.set_bg(None);
    slim_red.set_bold(false);
    slim_red.set_fg(Some(termcolor::Color::Red));
    let mut stream = termcolor::StandardStream::stderr(termcolor::ColorChoice::Auto);
    
    stream.set_color(&slim_red)?;
    stream.write_all(b" ----=== ")?;
    stream.set_color(&bold_red)?;
    stream.write_all(b"Parsing Error")?;
    stream.set_color(&slim_red)?;
    stream.write_all(b" ===----")?;
    stream.reset()?;
    writeln!(&mut stream, "")?;
    
    match error {
        frontend::LexerError::EOF(message) => {
            writeln!(&mut stream, "Hit an unexpected EOF: {}", message)?;
        },
        frontend::LexerError::ExpectedIdentifier(pos) => {
            let (line, col) = view.lineinfo(pos);
            writeln!(&mut stream, "In line {} column {}: Expected an identifier", line, col)?;
            print_line_context(&mut stream, view, line, col, 1)?;
        },
        frontend::LexerError::MissingWhitespace(pos) => {
            let (line, col) = view.lineinfo(pos);
            writeln!(&mut stream, "In line {} column {}: A whitespace is missing", line, col)?;
            print_line_context(&mut stream, view, line, col, 1)?;
        },
        frontend::LexerError::ExpectedChar(pos, c) => {
            let (line, col) = view.lineinfo(pos);
            writeln!(&mut stream, "In line {} column {}: Expected the character '{}'", line, col, c)?;
            print_line_context(&mut stream, view, line, col, 1)?;
        },
        frontend::LexerError::ExpectedKeyword(pos, keyword) =>  {
            let (line, col) = view.lineinfo(pos);
            writeln!(&mut stream, "In line {} column {}: Expected the keyword '{}'", line, col, keyword)?;
            print_line_context(&mut stream, view, line, col, 1)?;
        },
        frontend::LexerError::InvalidNumber(pos) => {
            let (line, col) = view.lineinfo(pos);
            writeln!(&mut stream, "In line {} column {}: Invalid number", line, col)?;
            print_line_context(&mut stream, view, line, col, 1)?;
        },
        frontend::LexerError::InvalidCharLiteral(pos) => {
            let (line, col) = view.lineinfo(pos);
            writeln!(&mut stream, "In line {} column {}: Invalid character constant", line, col)?;
            print_line_context(&mut stream, view, line, col, 1)?;
        },
    }
    
    writeln!(&mut stream, "")?;
    
    Ok(())
}

fn main() {
    let view = frontend::SourceView::from_file("all.chm");
    let mut lexer = frontend::Lexer::new(&view);
    
    let tokens = match lexer.lex() {
        Ok(tokens) => tokens,
        Err(error) => {
            if let Err(_) = print_lexing_error(&view, error) {
                println!("An error occured but I couldn't produce any error message");
            }
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
