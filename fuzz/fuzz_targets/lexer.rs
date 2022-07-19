#![no_main]
use libfuzzer_sys::fuzz_target;

use chameleon::{Lexer, SourceView};

fuzz_target!(|data: &[u8]| {
    if let Ok(s) = std::str::from_utf8(data) {
        let view = SourceView::new(s);
        let _ = Lexer::new(&view).lex();
    }
});
