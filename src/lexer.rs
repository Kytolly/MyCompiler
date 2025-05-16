use std::io;  
#[derive(Debug, PartialEq, Clone)]

enum Token {
    // Keywords
    IntKeyword,
    ReturnKeyword,
    IfKeyword,
    ElseKeyword,

    // Identifiers
    Identifier(String),

    // Literals
    IntegerLiteral(i64),

    // Operators
    Equal,       // ==
    Minus,       // -
    Star,        // *
    // Note: Single '=' (Assign) is omitted as it's not in the minimal factorial example's C subset.

    // Punctuation
    LParen,      // (
    RParen,      // )
    LBrace,      // {
    RBrace,      // }
    Semicolon,   // ;

    // Special
    EndOfFile,
    Illegal(char), // For unrecognized characters
}

struct Lexer<'a> {
    input: &'a str,
    position: usize,      // current position in input (points to current char)
    read_position: usize, // current reading position in input (after current char)
    ch: Option<char>,     // current char under examination
}

impl<'a> Lexer<'a> {
    fn new(input: &'a str) -> Self {
        let mut lexer = Lexer {
            input,
            position: 0,
            read_position: 0,
            ch: None,
        };
        lexer.read_char(); // Initialize ch, position, and read_position
        lexer
    }

    // Reads the next character and advances the position in the input.
    fn read_char(&mut self) {
        if self.read_position >= self.input.len() {
            self.ch = None; // End Of File
        } else {
            self.ch = self.input.chars().nth(self.read_position);
        }
        self.position = self.read_position;
        self.read_position += 1;
    }

    // Peeks at the next character without consuming it.
    fn peek_char(&self) -> Option<char> {
        if self.read_position >= self.input.len() {
            None
        } else {
            self.input.chars().nth(self.read_position)
        }
    }

    // Skips whitespace characters.
    fn skip_whitespace(&mut self) {
        while let Some(c) = self.ch {
            if c.is_whitespace() {
                self.read_char();
            } else {
                break;
            }
        }
    }

    // Reads an identifier (sequence of letters, numbers, and underscores, not starting with a number).
    fn read_identifier(&mut self) -> String {
        let start_pos = self.position;
        // Loop while the current character is a valid part of an identifier.
        // self.ch is Some(char) here.
        while self.ch.map_or(false, Self::is_identifier_char) {
            self.read_char();
        }
        // After the loop, self.position is at the character *after* the identifier.
        self.input[start_pos..self.position].to_string()
    }

    // Reads a sequence of digits as a number.
    fn read_number(&mut self) -> String {
        let start_pos = self.position;
        // Loop while the current character is a digit.
        // self.ch is Some(char) here.
        while self.ch.map_or(false, |c| c.is_digit(10)) {
            self.read_char();
        }
        // After the loop, self.position is at the character *after* the number.
        self.input[start_pos..self.position].to_string()
    }
    
    // Helper to check if a character can start an identifier.
    fn is_letter(ch: char) -> bool {
        ch.is_alphabetic() || ch == '_'
    }

    // Helper to check if a character can be part of an identifier (after the first char).
    fn is_identifier_char(ch: char) -> bool {
        ch.is_alphanumeric() || ch == '_'
    }

    // Returns the next token from the input.
    fn next_token(&mut self) -> Token {
        self.skip_whitespace();

        let token = match self.ch {
            Some('=') => {
                if self.peek_char() == Some('=') {
                    self.read_char(); // Consume the first '='
                    self.read_char(); // Consume the second '='
                    Token::Equal // ==
                } else {
                    // Single '=' is not a recognized token in our C subset.
                    let current_char = self.ch.unwrap(); // We know self.ch is Some('=')
                    self.read_char(); // Consume the '='
                    Token::Illegal(current_char)
                }
            }
            Some(';') => { self.read_char(); Token::Semicolon }
            Some('(') => { self.read_char(); Token::LParen }
            Some(')') => { self.read_char(); Token::RParen }
            Some('{') => { self.read_char(); Token::LBrace }
            Some('}') => { self.read_char(); Token::RBrace }
            Some('-') => { self.read_char(); Token::Minus }
            Some('*') => { self.read_char(); Token::Star }
            
            Some(c) if Self::is_letter(c) => {
                // This is an identifier or a keyword.
                // read_identifier consumes all characters of the identifier/keyword
                // and positions self.ch to the character *after* it.
                let ident = self.read_identifier();
                return match ident.as_str() { // Return directly, no further read_char needed here
                    "int" => Token::IntKeyword,
                    "return" => Token::ReturnKeyword,
                    "if" => Token::IfKeyword,
                    "else" => Token::ElseKeyword,
                    _ => Token::Identifier(ident),
                };
            }
            Some(c) if c.is_digit(10) => {
                // This is an integer literal.
                // read_number consumes all digits of the number
                // and positions self.ch to the character *after* it.
                let num_str = self.read_number();
                // Return directly, no further read_char needed here
                return match num_str.parse::<i64>() {
                    Ok(n) => Token::IntegerLiteral(n),
                    Err(_) => Token::Illegal(c), // If parsing fails (e.g. too large, though read_number should ensure digits)
                };
            }
            None => Token::EndOfFile, // No more characters
            Some(c) => {
                // Unrecognized character
                self.read_char(); // Consume the illegal character
                Token::Illegal(c)
            }
        };
        token
    }
}

#[allow(dead_code)] 
fn show_lexer_example() {
    let c_code = r#"
int factorial(int n) {
    if (n == 0) {
        return 1;
    } else {
        return n * factorial(n - 1);
    }
}
"#;
    println!("Tokenizing C code:\n{}", c_code);
    let mut lexer = Lexer::new(c_code);
    loop {
        let token = lexer.next_token();
        println!("{:?}", token);
        if token == Token::EndOfFile {
            break;
        }
    }
}


fn main() {
    show_lexer_example(); 
}