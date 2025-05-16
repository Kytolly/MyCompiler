pub mod prep;
pub mod lex;
use prep::Preprocessor;
use lex::Lexer;

fn main() {
    let preprocessor = Preprocessor::new("test/pas_test.pas");
    let mut lexer = Lexer::new(preprocessor);
    lexer.analyse();
    lexer.save();
}