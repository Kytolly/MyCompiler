use crate::setting::Token;
use crate::lex::Lexer;
use std::collections::HashMap;

pub struct Parser {
    lexer: Lexer,
    current_token: Token,
    current_index: usize,
    symbol_table: HashMap<String, SymbolInfo>,
}

#[derive(Clone)]
struct SymbolInfo {
    token_type: Token,
    is_function: bool,
    params: Vec<String>,
}

impl Parser {
    pub fn new(mut lexer: Lexer) -> Self {
        let current_token = lexer.stream[0].clone();
        Parser {
            lexer,
            current_token,
            current_index: 0,
            symbol_table: HashMap::new(),
        }
    }

    fn advance(&mut self) {
        self.current_index += 1;
        if self.current_index < self.lexer.stream.len() {
            self.current_token = self.lexer.stream[self.current_index].clone();
        }
    }

    fn match_token(&mut self, expected: Token) -> bool {
        if self.current_token == expected {
            self.advance();
            true
        } else {
            false
        }
    }

    pub fn parse(&mut self) -> Result<(), String> {
        self.parse_program()
    }

    fn parse_program(&mut self) -> Result<(), String> {
        // program -> begin declarations statements end
        if !self.match_token(Token::Begin) {
            return Err("Expected 'begin'".to_string());
        }

        self.parse_declarations()?;
        self.parse_statements()?;

        if !self.match_token(Token::End) {
            return Err("Expected 'end'".to_string());
        }

        Ok(())
    }

    fn parse_declarations(&mut self) -> Result<(), String> {
        // declarations -> declaration*
        while self.current_token == Token::Integer {
            self.parse_declaration()?;
        }
        Ok(())
    }

    fn parse_declaration(&mut self) -> Result<(), String> {
        // declaration -> integer id; | integer function id(id);
        if !self.match_token(Token::Integer) {
            return Err("Expected 'integer'".to_string());
        }

        if self.current_token == Token::Function {
            self.parse_function_declaration()?;
        } else {
            self.parse_variable_declaration()?;
        }

        Ok(())
    }

    fn parse_function_declaration(&mut self) -> Result<(), String> {
        // integer function id(id);
        if !self.match_token(Token::Function) {
            return Err("Expected 'function'".to_string());
        }

        let func_name = match &self.current_token {
            Token::Identifier(name) => name.clone(),
            _ => return Err("Expected function name".to_string()),
        };
        self.advance();

        if !self.match_token(Token::LeftParenthesis) {
            return Err("Expected '('".to_string());
        }

        let param_name = match &self.current_token {
            Token::Identifier(name) => name.clone(),
            _ => return Err("Expected parameter name".to_string()),
        };
        self.advance();

        if !self.match_token(Token::RightParenthesis) {
            return Err("Expected ')'".to_string());
        }

        if !self.match_token(Token::Semicolon) {
            return Err("Expected ';'".to_string());
        }

        // Add function to symbol table
        self.symbol_table.insert(func_name, SymbolInfo {
            token_type: Token::Function,
            is_function: true,
            params: vec![param_name],
        });

        Ok(())
    }

    fn parse_variable_declaration(&mut self) -> Result<(), String> {
        // integer id;
        let var_name = match &self.current_token {
            Token::Identifier(name) => name.clone(),
            _ => return Err("Expected variable name".to_string()),
        };
        self.advance();

        if !self.match_token(Token::Semicolon) {
            return Err("Expected ';'".to_string());
        }

        // Add variable to symbol table
        self.symbol_table.insert(var_name, SymbolInfo {
            token_type: Token::Integer,
            is_function: false,
            params: vec![],
        });

        Ok(())
    }

    fn parse_statements(&mut self) -> Result<(), String> {
        // statements -> statement*
        while self.current_token != Token::End {
            self.parse_statement()?;
        }
        Ok(())
    }

    fn parse_statement(&mut self) -> Result<(), String> {
        match &self.current_token {
            Token::Identifier(_) => self.parse_assignment(),
            Token::If => self.parse_if_statement(),
            Token::Read => self.parse_read_statement(),
            Token::Write => self.parse_write_statement(),
            _ => Err("Invalid statement".to_string()),
        }
    }

    fn parse_assignment(&mut self) -> Result<(), String> {
        // id := expression;
        let var_name = match &self.current_token {
            Token::Identifier(name) => name.clone(),
            _ => return Err("Expected variable name".to_string()),
        };
        self.advance();

        if !self.match_token(Token::Assign) {
            return Err("Expected ':='".to_string());
        }

        self.parse_expression()?;

        if !self.match_token(Token::Semicolon) {
            return Err("Expected ';'".to_string());
        }

        Ok(())
    }

    fn parse_if_statement(&mut self) -> Result<(), String> {
        // if condition then statement else statement
        if !self.match_token(Token::If) {
            return Err("Expected 'if'".to_string());
        }

        self.parse_condition()?;

        if !self.match_token(Token::Then) {
            return Err("Expected 'then'".to_string());
        }

        self.parse_statement()?;

        if !self.match_token(Token::Else) {
            return Err("Expected 'else'".to_string());
        }

        self.parse_statement()?;

        Ok(())
    }

    fn parse_read_statement(&mut self) -> Result<(), String> {
        // read(id);
        if !self.match_token(Token::Read) {
            return Err("Expected 'read'".to_string());
        }

        if !self.match_token(Token::LeftParenthesis) {
            return Err("Expected '('".to_string());
        }

        match &self.current_token {
            Token::Identifier(_) => self.advance(),
            _ => return Err("Expected variable name".to_string()),
        }

        if !self.match_token(Token::RightParenthesis) {
            return Err("Expected ')'".to_string());
        }

        if !self.match_token(Token::Semicolon) {
            return Err("Expected ';'".to_string());
        }

        Ok(())
    }

    fn parse_write_statement(&mut self) -> Result<(), String> {
        // write(expression);
        if !self.match_token(Token::Write) {
            return Err("Expected 'write'".to_string());
        }

        if !self.match_token(Token::LeftParenthesis) {
            return Err("Expected '('".to_string());
        }

        self.parse_expression()?;

        if !self.match_token(Token::RightParenthesis) {
            return Err("Expected ')'".to_string());
        }

        if !self.match_token(Token::Semicolon) {
            return Err("Expected ';'".to_string());
        }

        Ok(())
    }

    fn parse_expression(&mut self) -> Result<(), String> {
        // expression -> term ((+ | -) term)*
        self.parse_term()?;

        while matches!(self.current_token, Token::Minus) {
            self.advance();
            self.parse_term()?;
        }

        Ok(())
    }

    fn parse_term(&mut self) -> Result<(), String> {
        // term -> factor ((* | /) factor)*
        self.parse_factor()?;

        while matches!(self.current_token, Token::Multiply) {
            self.advance();
            self.parse_factor()?;
        }

        Ok(())
    }

    fn parse_factor(&mut self) -> Result<(), String> {
        // factor -> id | number | (expression) | function_call
        match &self.current_token {
            Token::Identifier(_) => {
                let name = match &self.current_token {
                    Token::Identifier(name) => name.clone(),
                    _ => unreachable!(),
                };
                self.advance();

                if self.current_token == Token::LeftParenthesis {
                    self.parse_function_call(&name)?;
                }
                Ok(())
            }
            Token::IntegerLiteral(_) => {
                self.advance();
                Ok(())
            }
            Token::LeftParenthesis => {
                self.advance();
                self.parse_expression()?;
                if !self.match_token(Token::RightParenthesis) {
                    return Err("Expected ')'".to_string());
                }
                Ok(())
            }
            _ => Err("Invalid factor".to_string()),
        }
    }

    fn parse_function_call(&mut self, func_name: &str) -> Result<(), String> {
        // function_call -> id(expression)
        if !self.match_token(Token::LeftParenthesis) {
            return Err("Expected '('".to_string());
        }

        self.parse_expression()?;

        if !self.match_token(Token::RightParenthesis) {
            return Err("Expected ')'".to_string());
        }

        Ok(())
    }

    fn parse_condition(&mut self) -> Result<(), String> {
        // condition -> expression (== | <> | < | <= | > | >=) expression
        self.parse_expression()?;

        match self.current_token {
            Token::Equal | Token::NotEqual | Token::Less | Token::LessEqual |
            Token::Greater | Token::GreaterEqual => {
                self.advance();
                self.parse_expression()?;
                Ok(())
            }
            _ => Err("Invalid comparison operator".to_string()),
        }
    }
}
