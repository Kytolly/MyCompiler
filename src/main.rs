pub mod prep;
pub mod lex;
pub mod env;
pub mod parse;

use prep::Preprocessor;
use lex::Lexer;
use parse::Parser;
use env::Env;

fn main() {
    let path = "test/6";
    let mode = "file";
    let preprocessor = Preprocessor::new(path);
    println!("---------------------------------");

    let mut lexer = Lexer::new(preprocessor, path, mode);
    lexer.analyse();
    lexer.save();
    println!("---------------------------------");

    let mut env = Env::new();
    let s = lexer.get_stream();
    
    let mut parser = Parser::new(s, mode, path.to_string());
    match parser.analyse(&mut env) {
        Ok(()) => println!("compilered!"),
        _ => println!("syntax error!")
    }
}