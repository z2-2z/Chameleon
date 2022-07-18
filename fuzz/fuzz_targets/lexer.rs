#![no_main]
use libfuzzer_sys::fuzz_target;

use chameleon::Lexer;

fuzz_target!(|data: &[u8]| {
    let _ = Lexer::new(data).lex();
});
