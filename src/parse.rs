use crate::setting::{Token, ErrorMessage};

pub struct Parser {
    // 语法分析器
    pub stream: Vec<Token>, // 输入的token流

    // pub lookahead: Token, // 当前的token
    pub pos: usize, //当前token所在位置
    pub line: usize, // 当前token所在行数

}

impl Parser {
    pub fn new(s: Vec<Token>) -> Self {
        let p = Parser {
            stream: s,
            pos: 0,
            line: 1,
        };
        p
    }
    pub fn analyse(&mut self) -> Result<(), ErrorMessage> {
        self.parse_node_program()
    }

    fn current_token(&self) -> Token {
        self.stream[self.pos].clone()
    }
    fn advance(&mut self) {
        self.pos += 1;
        let tk = self.current_token();
        if tk == Token::Eol {
            self.line += 1;
            self.pos += 1;
        }
    }
    fn match_token(&self, tk: Token) -> bool {
        self.current_token() == tk
    }
    fn error(&self, errmsg: ErrorMessage) {
        // 抛出错误
        // 这里还是简化实现
        // 应该扔到标准错误流中，写入文件
        // 不能和标准输出流混合
        match errmsg {
            ErrorMessage::SyntaxError => {
                println!("LINE{:?}: syntax error, exited", self.line);
            }
            ErrorMessage::WrongReserveYouMeanFunction => {
                println!("LINE{:?}: wrong reserve: you mean 'function'?", self.line);
            }
            ErrorMessage::WrongReserveYouMeanRead => {
                println!("LINE{:?}: wrong reserve: you mean 'read'?", self.line);
            }
            ErrorMessage::WrongReserveYouMeanWrite => {
                println!("LINE{:?}: wrong reserve: you mean 'write'?", self.line);
            }
            ErrorMessage::WrongAssignToken => {
                println!("LINE{:?}: wrong assign operator: you mean ':='?", self.line);
            }
            ErrorMessage::InvalidTypeExpectedInterger => {
                println!("LINE{:?}: invalid type here, expected integer", self.line);
            }
            ErrorMessage::InvalidNumber => {
                println!("LINE{:?}: invalid number", self.line);
            }
            ErrorMessage::OverflowIdentifier => {
                println!("LINE{:?}: the length of identifier overflows", self.line);
            }
            ErrorMessage::FailMatchingSemicolon => {
                println!("LINE{:?}: fail in matching semicolon", self.line);
            }
            ErrorMessage::MissingSemicolon => {
                println!("LINE{:?}: missing a ';' at the end of the statement", self.line);
            }
            ErrorMessage::MissingLeftParenthesis => {
                println!("LINE{:?}: missing a '(' following the function statement", self.line);
            }
            ErrorMessage::MissingRightParenthesis => {
                println!("LINE{:?}: missing a ')' to cover the block", self.line);
            }
            ErrorMessage::MissingMultiply => {
                println!("LINE{:?}: expected '*' ", self.line);
            }
            ErrorMessage::MissingIf => {
                println!("LINE{:?}: expected 'if' ", self.line);
            }
            ErrorMessage::MissingThen => {
                println!("LINE{:?}: expected 'then' ", self.line);
            }
            ErrorMessage::MissingElse => {
                println!("LINE{:?}: expected 'else' ", self.line);
            }
            ErrorMessage::SyntaxErrorExpectedABlock => {
                println!("LINE{:?}: syntax error, expected a block", self.line);
            }
            ErrorMessage::FailMatching => {
                println!("LINE{:?}: 符号匹配错误!", self.line);
            }
            ErrorMessage::MissingEnd => {
                println!("LINE{:?}: missing END: this block is not covered", self.line);
            }
            ErrorMessage::NotFoundDeclarationInThisField => {
                println!("LINE{:?}: 符号在该作用域内找不到声明!", self.line);
            }
            ErrorMessage::FoundRepeatDeclarationInThisField => {
                println!("LINE{:?}: 符号在该作用域内重复声明!", self.line);
            }
        }
    }
    fn skip_bad_line(&mut self) {
        // 跳过错误行
        let tk = self.current_token();
        while tk != Token::Eol {
            self.advance();
        }
    }
    fn handle_error(&mut self, errmsg: ErrorMessage) -> Result<(), ErrorMessage>{
        let res = Err(errmsg.clone());
        self.error(errmsg);
        self.skip_bad_line();
        res
    }
    fn parse_node_program(&mut self) -> Result<(), ErrorMessage>{
        // <程序> → begin <说明语句表>；<执行语句表> end
        match self.match_token(Token::Begin) {
            true => self.advance(),
            false => return self.handle_error(ErrorMessage::SyntaxErrorExpectedABlock)
        }
        match self.parse_node_declaration_statement_table() {
            Ok(_) => (),
            Err(e) => return self.handle_error(e),
        }
        match self.match_token(Token::Semicolon) {
            true => self.advance(),
            false => return self.handle_error(ErrorMessage::MissingSemicolon)
        }
        match self.parse_node_execution_statement_table() {
            Ok(_) => (),
            Err(e) => return self.handle_error(e),
        }
        match self.match_token(Token::End) {
            true => Ok(()),
            false => return self.handle_error(ErrorMessage::MissingEnd)
        }
    }
    fn parse_node_declaration_statement_table(&mut self) -> Result<(), ErrorMessage>{
        // <说明语句表> → <说明语句><说明语句表'> 
        match self.parse_node_declaration_statement() {
            Ok(_) => (),
            Err(e) => return self.handle_error(e),
        }
        match self.parse_node_declaration_statement_table_prime() {
            Ok(_) => Ok(()),
            Err(e) => return self.handle_error(e),
        }
    }
    fn parse_node_declaration_statement(&mut self) -> Result<(), ErrorMessage>{
        // <说明语句> → <变量说明>|<过程说明>
        match self.parse_node_variable_declaration() {
            Ok(_) => Ok(()),
            Err(e) => match self.parse_node_procedure_declaration() {
                Ok(_) => Ok(()),
                Err(_) => self.handle_error(e),
            }
        }
    }
    fn parse_node_declaration_statement_table_prime(&mut self) -> Result<(), ErrorMessage>{
        // <说明语句表'> → ；<说明语句> <说明说明表'> | ε
        match self.match_token(Token::Semicolon) {
            true => self.advance(),
            false => return Ok(()),
        }
        match self.parse_node_declaration_statement() {
            Ok(_) => (),
            Err(e) => return self.handle_error(e),
        }
        match self.parse_node_declaration_statement_table_prime() {
            Ok(_) => Ok(()),
            Err(e) => return self.handle_error(e),
        }
    }  
    fn parse_node_variable_declaration(&mut self) -> Result<(), ErrorMessage>{
        // <变量说明> → integer <标识符>
        match self.match_token(Token::Integer) {
            true => self.advance(),
            false => return self.handle_error(ErrorMessage::InvalidTypeExpectedInterger)
        }
        match self.parse_node_identifier() {
            Ok(_) => Ok(()),
            Err(e) => return self.handle_error(e),
        }
    }
    fn parse_node_procedure_declaration(&mut self) -> Result<(), ErrorMessage>{
        // <过程说明> → integer function <标识符>(<形参表>)；<过程体>
        match self.match_token(Token::Integer) {
            true => self.advance(),
            false => return self.handle_error(ErrorMessage::InvalidTypeExpectedInterger)
        }
        match self.match_token(Token::Function) {
            true => self.advance(),
            false => return self.handle_error(ErrorMessage::WrongReserveYouMeanFunction)
        }
        match self.parse_node_identifier() {
            Ok(_) => (),
            Err(e) => return self.handle_error(e),
        }
        match self.match_token(Token::LeftParenthesis) {
            true => self.advance(),
            false => return self.handle_error(ErrorMessage::MissingLeftParenthesis)
        }
        match self.parse_node_parameter_table() {
            Ok(_) => (),
            Err(e) => return self.handle_error(e),
        }
        match self.match_token(Token::RightParenthesis) {
            true => self.advance(),
            false => return self.handle_error(ErrorMessage::MissingLeftParenthesis)
        }
        match self.match_token(Token::Semicolon) {
            true => self.advance(),
            false => return self.handle_error(ErrorMessage::MissingSemicolon)
        }
        match self.parse_node_procedure_body() {
            Ok(_) => Ok(()),
            Err(e) => return self.handle_error(e),
        }
    }
    fn parse_node_parameter_table(&mut self) -> Result<(), ErrorMessage>{
        // <形参表> → <变量>
        match self.parse_node_variable() {
            Ok(_) => Ok(()),
            Err(e) => return self.handle_error(e),
        }
    }
    fn parse_node_procedure_body(&mut self) -> Result<(), ErrorMessage>{
        // <过程体> → begin <说明部分>；<执行部分> end
        match self.match_token(Token::Begin) {
            true => self.advance(),
            false => return self.handle_error(ErrorMessage::SyntaxErrorExpectedABlock)
        }
        match self.parse_node_declaration_statement_table() {
            Ok(_) => (),
            Err(e) => return self.handle_error(e),
        }
        match self.match_token(Token::Semicolon) {
            true => self.advance(),
            false => return self.handle_error(ErrorMessage::MissingSemicolon)
        }
        match self.parse_node_execution_statement_table() {
            Ok(_) => (),
            Err(e) => return self.handle_error(e),
        }
        match self.match_token(Token::End) {
            true => Ok(()),
            false => return self.handle_error(ErrorMessage::MissingEnd)
        }
    }
    fn parse_node_execution_statement_table(&mut self) -> Result<(), ErrorMessage>{
        // <执行语句表> → <执行语句> <执行语句表'>
        match self.parse_node_execution_statement() {
            Ok(_) => (),
            Err(e) => return self.handle_error(e),
        }
        match self.parse_node_execution_statement_table_prime() {
            Ok(_) => Ok(()),
            Err(e) => return self.handle_error(e),
        }
    }
    fn parse_node_execution_statement_table_prime(&mut self) -> Result<(), ErrorMessage>{
        // <执行语句表'> → ；<执行语句> <执行语句表'> | ε
        match self.match_token(Token::Semicolon) {
            true => self.advance(),
            false => return Ok(()),
        }
        match self.parse_node_execution_statement() {
            Ok(_) => (),
            Err(e) => return self.handle_error(e),
        }
        match self.parse_node_execution_statement_table_prime() {
            Ok(_) => Ok(()),
            Err(e) => return self.handle_error(e),
        }
    }
    fn parse_node_execution_statement(&mut self) -> Result<(), ErrorMessage>{
        // <执行语句> → <赋值语句> | <条件语句> | <读语句> | <写语句>
        let res1 = self.parse_node_assignment_statement();
        let res2 = self.parse_node_conditional_statement();
        let res3 = self.parse_node_read_statement();
        let res4 = self.parse_node_write_statement();
        
        match (res1, res2, res3, res4) {
            (Ok(_), _, _, _) | (_, Ok(_), _, _) | (_, _, Ok(_), _) | (_, _, _, Ok(_)) => Ok(()),
            _ => self.handle_error(ErrorMessage::SyntaxError)
        }
    }
    fn parse_node_assignment_statement(&mut self) -> Result<(), ErrorMessage>{
        // <赋值语句> → <变量> := <表达式>
        match self.parse_node_variable() {
            Ok(_) => (),
            Err(e) => return self.handle_error(e),
        }
        match self.match_token(Token::Assign) {
            true => Ok(()),
            false => return self.handle_error(ErrorMessage::WrongAssignToken)
        }
    }
    fn parse_node_conditional_statement(&mut self) -> Result<(), ErrorMessage>{
        // <条件语句> → if <条件> then <执行语句表> else <执行语句>
        match self.match_token(Token::If) {
            true => self.advance(),
            false => return self.handle_error(ErrorMessage::MissingIf)
        }
        match self.parse_node_condition() {
            Ok(_) => (),
            Err(e) => return self.handle_error(e),
        }
        match self.match_token(Token::Then) {
            true => self.advance(),
            false => return self.handle_error(ErrorMessage::MissingThen)
        }
        match self.parse_node_execution_statement() {
            Ok(_) => (),
            Err(e) => return self.handle_error(e),
        }
        match self.match_token(Token::Else) {
            true => self.advance(),
            false => return self.handle_error(ErrorMessage::MissingElse)
        }
        match self.parse_node_execution_statement() {
            Ok(_) => Ok(()),
            Err(e) => return self.handle_error(e),
        }
    }
    fn parse_node_read_statement(&mut self) -> Result<(), ErrorMessage>{
        // <读语句> → read(<变量>)
        match self.match_token(Token::Read) {
            true => self.advance(),
            false => return self.handle_error(ErrorMessage::WrongReserveYouMeanRead)
        }
        match self.match_token(Token::LeftParenthesis) {
            true => self.advance(),
            false => return self.handle_error(ErrorMessage::MissingLeftParenthesis)
        }
        match self.parse_node_variable() {
            Ok(_) => (),
            Err(e) => return self.handle_error(e),
        }
        match self.match_token(Token::RightParenthesis) {
            true => Ok(()),
            false => return self.handle_error(ErrorMessage::MissingLeftParenthesis)
        }
    }
    fn parse_node_write_statement(&mut self) -> Result<(), ErrorMessage>{
        // <写语句> → write(<表达式>)
        match self.match_token(Token::Write) {
            true => self.advance(),
            false => return self.handle_error(ErrorMessage::WrongReserveYouMeanWrite)
        }
        match self.match_token(Token::LeftParenthesis) {
            true => self.advance(),
            false => return self.handle_error(ErrorMessage::MissingLeftParenthesis)
        }
        match self.parse_node_variable() {
            Ok(_) => (),
            Err(e) => return self.handle_error(e),
        }
        match self.match_token(Token::RightParenthesis) {
            true => Ok(()),
            false => return self.handle_error(ErrorMessage::MissingLeftParenthesis)
        }
    }
    fn parse_node_condition(&mut self) -> Result<(), ErrorMessage>{
        // <条件> → <表达式> <关系运算符> <表达式>
        match self.parse_node_expression() {
            Ok(_) => (),
            Err(e) => return self.handle_error(e),
        }
        match self.parse_node_relational_operator() {
            Ok(_) => (),
            Err(e) => return self.handle_error(e),
        }
        match self.parse_node_expression() {
            Ok(_) => Ok(()),
            Err(e) => return self.handle_error(e),
        }
    }
    fn parse_node_expression(&mut self) -> Result<(), ErrorMessage>{
        // <表达式> → <项> <表达式'>
        match self.parse_node_term() {
            Ok(_) => (),
            Err(e) => return self.handle_error(e),
        }
        match self.parse_node_expression_prime() {
            Ok(_) => (),
            Err(e) => return self.handle_error(e),
        }
        Ok(())
    }
    fn parse_node_expression_prime(&mut self) -> Result<(), ErrorMessage>{
        // <表达式'> → - <项> <表达式'> | ε
        if self.match_token(Token::Minus) {
            match self.parse_node_term() {
                Ok(_) => (),
                Err(e) => return self.handle_error(e),
            }
            match self.parse_node_expression_prime() {
                Ok(_) => (),
                Err(e) => return self.handle_error(e),
            }
            Ok(())
        }else { 
            Ok(())
        }
    }
    fn parse_node_term(&mut self) -> Result<(), ErrorMessage>{
        // <项> → <因子> <项'>
        match self.parse_node_factor() {
            Ok(_) => (),
            Err(e) => return self.handle_error(e),
        }
        match self.parse_node_term_prime() {
            Ok(_) => (),
            Err(e) => return self.handle_error(e),
        }
        Ok(())
    }
    fn parse_node_term_prime(&mut self) -> Result<(), ErrorMessage>{
        // <项'> → * <因子> <项'>  | ε
        if self.match_token(Token::Multiply) {
            match self.parse_node_factor() {
                Ok(_) => (),
                Err(e) => return self.handle_error(e),
            }
            match self.parse_node_term_prime() {
                Ok(_) => (),
                Err(e) => return self.handle_error(e),
            }
            Ok(())
        }else {
            Ok(())
        }
    }
    fn parse_node_factor(&mut self) -> Result<(), ErrorMessage>{
        // <因子> → ( <表达式> ) | <常量> | <变量> | <函数调用>
        if self.match_token(Token::LeftParenthesis) {
            match self.parse_node_expression() {
                Ok(_) => (),
                Err(e) => return self.handle_error(e),
            }
            match self.match_token(Token::RightParenthesis) {
                true => self.advance(),
                false => return self.handle_error(ErrorMessage::MissingRightParenthesis)
            }
            Ok(())
        }else {
            let res1 = self.parse_node_constant();
            let res2 = self.parse_node_variable();
            let res3 = self.parse_node_function_call();
            match (res1, res2, res3) {
                (Ok(_), _, _) | (_, Ok(_), _) | (_, _, Ok(_)) => Ok(()),
                _ => self.handle_error(ErrorMessage::SyntaxError)
            }
        }
    }
    fn parse_node_function_call(&mut self) -> Result<(), ErrorMessage>{
        // <函数调用> → <标识符>(<形参表>)
        match self.parse_node_identifier() {
            Ok(_) => (),
            Err(e) => return self.handle_error(e),
        }
        match self.match_token(Token::LeftParenthesis) {
            true => self.advance(),
            false => return self.handle_error(ErrorMessage::MissingLeftParenthesis)
        }
        match self.parse_node_parameter_table() {
            Ok(_) => (),
            Err(e) => return self.handle_error(e),
        }
        match self.match_token(Token::RightParenthesis) {
            true => self.advance(),
            false => return self.handle_error(ErrorMessage::MissingLeftParenthesis)
        }
        Ok(())
    }
    fn parse_node_relational_operator(&mut self) -> Result<(), ErrorMessage>{
        // <关系运算符> → <= | < | > | >= | == | !=
        if self.match_token(Token::Equal)
        || self.match_token(Token::NotEqual)
        || self.match_token(Token::Less)
        || self.match_token(Token::LessEqual)
        || self.match_token(Token::Greater)
        || self.match_token(Token::GreaterEqual){
            Ok(())
        }else {
            self.handle_error(ErrorMessage::SyntaxError)
        }
    }
    fn parse_node_variable(&mut self) -> Result<(), ErrorMessage>{
        // <变量> → <标识符>
        Ok(())
    }
    fn parse_node_constant(&mut self) -> Result<(), ErrorMessage>{
        // <常量> 
        Ok(())
    }
    fn parse_node_identifier(&mut self) -> Result<(), ErrorMessage>{
        // <标识符>
        Ok(())
    }
}