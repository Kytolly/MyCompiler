pub mod prep;
pub mod lex;
pub mod env;
pub mod parse;

use prep::Preprocessor;
use lex::Lexer;
use parse::Parser;
// use env::Env;

fn main() {
    let preprocessor = Preprocessor::new("test/pas_test.pas");
    let mut lexer = Lexer::new(preprocessor);
    lexer.analyse();
    lexer.save();
    let s = lexer.get_stream();

    let mut parser = Parser::new(s);
    match parser.analyse() {
        Ok(()) => println!("compilered!"),
        _ => println!("syntax error!")
    }
}