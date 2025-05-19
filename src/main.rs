pub mod prep;
pub mod lex;
pub mod constant;
use prep::Preprocessor;
use lex::Lexer;

fn main() {
    let preprocessor = Preprocessor::new("test/pas_error3.pas");
    let mut lexer = Lexer::new(preprocessor);
    lexer.analyse();
    lexer.save();
}