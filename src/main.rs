pub mod prep;
pub mod lex;
use prep::Preprocessor;
use lex::Lexer;

fn main() {
    let preprocessor = Preprocessor::new("test/pas_test.pas");
    let mut lexer = Lexer::new(preprocessor.content);
    lexer.analyse();
    println!("打印token流");
    for tk in lexer.stream {
        println!("{:?}", tk);
    }
}