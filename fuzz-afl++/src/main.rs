use afl;
use chameleon::{Lexer, SourceView, Parser, GrammarGraph, GrammarStats};

fn main() {
    afl::fuzz!(|data: &[u8]| {
        if let Ok(s) = std::str::from_utf8(data) {
            let view = SourceView::new(s);
            if let Ok(tokens) = Lexer::new(&view).lex() {
                let mut parser = Parser::new(&view, &tokens);
                
                if let Ok(grammar) = parser.parse() {
                    let graph = GrammarGraph::from_grammar(&grammar);
                    let _ = graph.unreachable_containers();
                    
                    if graph.cycle().is_none() {
                        let stats = GrammarStats::from_grammar(&grammar);
                        let _ = stats.num_paths();
                        let _ = stats.max_input_size();
                        let _ = stats.min_input_size();
                    }
                }
            }
        }
    });
}
