use std::io::{Write, stdout, BufWriter};
use std::fs::File;
use std::fmt::Arguments;

pub struct CodeFormatter {
    stream: BufWriter<Box<dyn Write>>,
    indentation: usize,
}
impl CodeFormatter {
    pub fn stdout() -> Self {
        Self {
            stream: BufWriter::new(Box::new(stdout())),
            indentation: 0,
        }
    }
    
    pub fn file(name: &str) -> Self {
        Self {
            stream: BufWriter::new(Box::new(File::create(name).expect("Could not create file"))),
            indentation: 0,
        }
    }
    
    pub fn block_open(&mut self) {
        self.indentation += 4;
    }
    
    pub fn block_close(&mut self) {
        self.indentation = self.indentation.saturating_sub(4);
    }
    
    pub fn put_indented(&mut self, args: Arguments<'_>) {
        write!(&mut self.stream, "{:width$}", "", width = self.indentation).expect("Could not write to outfile");
        self.stream.write_fmt(args).expect("Could not write to outfile");
    }
    
    pub fn put_raw(&mut self, args: Arguments<'_>) {
        self.stream.write_fmt(args).expect("Could not write to outfile");
    }
}

#[macro_export]
macro_rules! emit_raw {
    ($prod:expr, $($arg:tt)*) => {
        $prod.put_raw(format_args!($($arg)*))
    };
}

#[macro_export]
#[allow_internal_unstable(format_args_nl)]
macro_rules! emit_line {
    ($prod:expr, $($arg:tt)*) => {
        $prod.put_indented(format_args_nl!($($arg)*))
    };
}
