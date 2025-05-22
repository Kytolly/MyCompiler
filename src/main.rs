pub mod prep;
pub mod lex;
pub mod env;
pub mod parse;

use prep::Preprocessor;
use lex::Lexer;
use parse::Parser;
use env::Env;

fn main() {
    let preprocessor = Preprocessor::new("test/0.pas");
    println!("---------------------------------");

    let mut lexer = Lexer::new(preprocessor);
    lexer.analyse();
    lexer.save();
    println!("---------------------------------");

    let mut env = Env::new();
    let s = lexer.get_stream();
    
    let mut parser = Parser::new(s);
    match parser.analyse(&mut env) {
        Ok(()) => println!("compilered!"),
        _ => println!("syntax error!")
    }
}