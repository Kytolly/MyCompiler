use crate::env::{Token, ErrorMessage};

pub struct Parser {
    // 语法分析器
    pub stream: Vec<Token>, // 输入的token流
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

    fn debug(&self) {
        println!("-------------------------");
        println!("debug point");
        println!("pos:{:?}", self.pos);
        println!("line:{:?}", self.line);
        println!("token:{:?}", self.current_token());
    }
    fn current_token(&self) -> Token {
        if self.pos >= self.stream.len() {
            Token::Eof
        } else {
            self.stream[self.pos].clone()
        }
    }
    fn advance(&mut self) {
        if self.pos >= self.stream.len() {
            return;
        }
        
        self.pos += 1;
        let mut tk = self.current_token();
        
        // 处理换行符
        while tk == Token::Eol && self.pos < self.stream.len() {
            self.line += 1;
            self.pos += 1;
            tk = self.current_token();
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
        self.debug();
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
        println!("-------------------------");
    }
    fn skip_bad_line(&mut self) {
        let mut tk = self.current_token();
        while tk != Token::Eol && tk != Token::Eof {
            self.advance();
            tk = self.current_token();
        }
    }
    fn handle_error(&mut self, errmsg: ErrorMessage) -> Result<(), ErrorMessage>{
        let res = Err(errmsg.clone());
        self.error(errmsg);
        self.skip_bad_line();
        res
    }
    
    fn parse_node_program(&mut self) -> Result<(), ErrorMessage>{
        // <程序> → <分程序>
        self.parse_node_block()
    }
    fn parse_node_block(&mut self) -> Result<(), ErrorMessage>{
        // <分程序> → begin <说明语句表><执行语句表> end
        match self.match_token(Token::Begin) {
            true => self.advance(),
            false => return self.handle_error(ErrorMessage::SyntaxErrorExpectedABlock)
        }
        match self.parse_node_declaration_statement_table() {
            Ok(_) => (),
            Err(e) => return self.handle_error(e),
        }
        // match self.match_token(Token::Semicolon) {
        //     true => self.advance(),
        //     false => return self.handle_error(ErrorMessage::MissingSemicolon),
        // }
        match self.parse_node_execution_statement_table() {
            Ok(_) => (),
            Err(e) => return self.handle_error(e),
        }
        match self.match_token(Token::End) {
            true => {
                self.advance();
                Ok(())
            },
            false => return self.handle_error(ErrorMessage::MissingEnd)
        }
    }
    fn parse_node_declaration_statement_table(&mut self) -> Result<(), ErrorMessage>{
        // <说明语句表> → {<说明语句> ;}
        // FOLLOW(<说明语句表>) 包含 FIRST(<执行语句表>) 和 'end'
        loop {
            // 检查当前token是否可以开始一个说明语句 (integer)
            if !self.match_token(Token::Integer) {
                // 如果不能开始说明语句，检查是否在 FOLLOW 集里
                match self.current_token() {
                    Token::Read | Token::Write | Token::If | Token::Identifier(_) | Token::End | Token::Eof => {
                        // 如果在 FOLLOW 集里，说明说明语句表结束
                        return Ok(());
                    },
                    _ => { // 否则是语法错误
                        self.handle_error(ErrorMessage::SyntaxError);
                        return Err(ErrorMessage::SyntaxError);
                    }
                }
            }
            
            // 解析一个说明语句
            match self.parse_node_declaration_statement() {
                Ok(_) => (),
                Err(e) => return self.handle_error(e),
            }
            
            // 期望匹配分号
            match self.match_token(Token::Semicolon) {
                true => self.advance(),
                false => return self.handle_error(ErrorMessage::MissingSemicolon),
            }
        }
    }
    fn parse_node_declaration_statement(&mut self) -> Result<(), ErrorMessage>{
        // <说明语句> → integer <说明语句'>
        match self.match_token(Token::Integer) {
            true => self.advance(),
            false => return self.handle_error(ErrorMessage::InvalidTypeExpectedInterger)
        }
        self.parse_node_declaration_statement_prime()
    }
    fn parse_node_declaration_statement_prime(&mut self) -> Result<(), ErrorMessage>{
        // <说明语句'> → <变量> | function <标识符>（<参数>）<函数体> ;
        if self.match_token(Token::Function) {
            // 函数说明分支
            self.advance();
            match self.parse_node_identifier() {
                Ok(_) => (),
                Err(e) => return self.handle_error(e),
            }
            match self.match_token(Token::LeftParenthesis) {
                true => self.advance(),
                false => return self.handle_error(ErrorMessage::MissingLeftParenthesis)
            }
            match self.parse_node_parameter() {
                Ok(_) => (),
                Err(e) => return self.handle_error(e),
            }
            match self.match_token(Token::RightParenthesis) {
                true => self.advance(),
                false => return self.handle_error(ErrorMessage::MissingRightParenthesis)
            }
            // 在函数体后期望匹配分号
            match self.match_token(Token::Semicolon) {
                true => {
                    self.advance();
                    match self.parse_node_function_body() {
                        Ok(_) => Ok(()),
                        Err(e) => return self.handle_error(e),
                    }
                },
                false => { // 如果没有分号，是语法错误
                    return self.handle_error(ErrorMessage::MissingSemicolon);
                }
            }
        } else {
            // 变量说明分支
            match self.parse_node_variable() {
                Ok(_) => Ok(()),
                Err(e) => return self.handle_error(e),
            }
        }
    }
    fn parse_node_function_body(&mut self) -> Result<(), ErrorMessage>{
        // <函数体> → begin <说明语句表><执行语句表> end
        match self.match_token(Token::Begin) {
            true => self.advance(),
            false => return self.handle_error(ErrorMessage::SyntaxErrorExpectedABlock)
        }
        match self.parse_node_declaration_statement_table() {
            Ok(_) => (),
            Err(e) => return self.handle_error(e),
        }
        // match self.match_token(Token::Semicolon) {
        //     true => self.advance(),
        //     false => return self.handle_error(ErrorMessage::MissingSemicolon)
        // }
        match self.parse_node_execution_statement_table() {
            Ok(_) => (),
            Err(e) => return self.handle_error(e),
        }
        match self.match_token(Token::End) {
            true => {
                self.advance();
                Ok(())
            },
            false => return self.handle_error(ErrorMessage::MissingEnd)
        }
    }
    fn parse_node_parameter(&mut self) -> Result<(), ErrorMessage>{
        // <参数> → <算术表达式>
        match self.parse_node_expression() {
            Ok(_) => Ok(()),
            Err(e) => return self.handle_error(e),
        }
    }
    fn parse_node_execution_statement_table(&mut self) -> Result<(), ErrorMessage>{
        // <执行语句表> → {<执行语句> ;}
        // FOLLOW(<执行语句表>) 包含 'end' 和 '$'
        loop {
            // 检查当前token是否可以开始一个执行语句
            match self.current_token() {
                Token::Read | Token::Write | Token::If | Token::Identifier(_) => {
                    // 可以开始执行语句，继续解析
                },
                Token::End | Token::Eof => {
                    // 否则，检查是否在 FOLLOW 集里 (end 或 EOF)
                    // 如果在，说明执行语句表结束
                    return Ok(());
                },
                _ => { // 否则是语法错误
                    self.handle_error(ErrorMessage::SyntaxError);
                    return Err(ErrorMessage::SyntaxError);
                }
            }
            
            // 解析一个执行语句
            match self.parse_node_execution_statement() {
                Ok(_) => (),
                Err(e) => return self.handle_error(e),
            }
            
            // 期望匹配分号
            match self.match_token(Token::Semicolon) {
                true => self.advance(),
                false => return self.handle_error(ErrorMessage::MissingSemicolon),
            }
        }
    }
    fn parse_node_execution_statement(&mut self) -> Result<(), ErrorMessage>{
        // <执行语句> → <读语句>│<写语句>│<赋值语句>│<条件语句>
        match self.current_token() {
            Token::Read => self.parse_node_read_statement(),
            Token::Write => self.parse_node_write_statement(),
            Token::If => self.parse_node_conditional_statement(),
            Token::Identifier(_) => self.parse_node_assignment_statement(),
            _ => self.handle_error(ErrorMessage::SyntaxError)
        }
    }
    fn parse_node_assignment_statement(&mut self) -> Result<(), ErrorMessage>{
        // <赋值语句> → <变量> := <算术表达式>
        match self.parse_node_variable() {
            Ok(_) => (),
            Err(e) => return self.handle_error(e),
        }
        match self.match_token(Token::Assign) {
            true => self.advance(),
            false => return self.handle_error(ErrorMessage::WrongAssignToken)
        }
        match self.parse_node_expression() {
            Ok(_) => Ok(()),
            Err(e) => return self.handle_error(e),
        }
    }
    fn parse_node_conditional_statement(&mut self) -> Result<(), ErrorMessage>{
        // <条件语句> → if<条件表达式>then<执行语句>else <执行语句>
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
            true => {
                self.advance();
                Ok(())
            },
            false => return self.handle_error(ErrorMessage::MissingRightParenthesis)
        }
    }
    fn parse_node_write_statement(&mut self) -> Result<(), ErrorMessage>{
        // <写语句> → write(<变量>)
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
            true => {
                self.advance();
                Ok(())
            },
            false => return self.handle_error(ErrorMessage::MissingRightParenthesis)
        }
    }
    fn parse_node_condition(&mut self) -> Result<(), ErrorMessage>{
        // <条件表达式> → <算术表达式><关系运算符><算术表达式>
        match self.parse_node_expression() {
            Ok(_) => (),
            Err(e) => return self.handle_error(e),
        }
        match self.parse_node_relational_operator() {
            Ok(_) => self.advance(),
            Err(e) => return self.handle_error(e),
        }
        match self.parse_node_expression() {
            Ok(_) => (),
            Err(e) => return self.handle_error(e),
        }
        Ok(())
    }
    fn parse_node_expression(&mut self) -> Result<(), ErrorMessage>{
        // <算术表达式> → <项> <算术表达式'>
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
        // <算术表达式'> → -<项> <算术表达式'> | ε
        if self.match_token(Token::Minus) {
            self.advance();
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
        // <项'> → *<因子> <项'> | ε
        if self.match_token(Token::Multiply) {
            self.advance();
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
        match self.current_token() {
            Token::LeftParenthesis => {
                self.advance();
                match self.parse_node_expression() {
                    Ok(_) => (),
                    Err(e) => return self.handle_error(e),
                }
                match self.match_token(Token::RightParenthesis) {
                    true => {
                        self.advance();
                        Ok(())
                    },
                    false => self.handle_error(ErrorMessage::MissingRightParenthesis)
                }
            },
            Token::IntegerLiteral(_) => self.parse_node_constant(),
            Token::Identifier(_) => {
                // 需要前看一个token来判断是变量还是函数调用
                if let Some(next_token) = self.stream.get(self.pos + 1) {
                    if *next_token == Token::LeftParenthesis {
                        self.parse_node_function_call()
                    } else {
                        self.parse_node_variable()
                    }
                } else {
                    self.parse_node_variable()
                }
            },
            _ => self.handle_error(ErrorMessage::SyntaxError)
        }
    }
    fn parse_node_function_call(&mut self) -> Result<(), ErrorMessage>{
        // <函数调用> → <标识符>(<参数>)
        match self.parse_node_identifier() {
            Ok(_) => (),
            Err(e) => return self.handle_error(e),
        }
        match self.match_token(Token::LeftParenthesis) {
            true => self.advance(),
            false => return self.handle_error(ErrorMessage::MissingLeftParenthesis)
        }
        match self.parse_node_parameter() {
            Ok(_) => (),
            Err(e) => return self.handle_error(e),
        }
        match self.match_token(Token::RightParenthesis) {
            true => self.advance(),
            false => return self.handle_error(ErrorMessage::MissingRightParenthesis)
        }
        Ok(())
    }
    fn parse_node_relational_operator(&mut self) -> Result<(), ErrorMessage>{
        // <关系运算符> → <│<=│>│>=│=│<>
        match self.current_token() {
            Token::Equal => Ok(()),
            Token::NotEqual => Ok(()),
            Token::Less => Ok(()),
            Token::LessEqual => Ok(()),
            Token::Greater => Ok(()),
            Token::GreaterEqual => Ok(()),
            _ => self.handle_error(ErrorMessage::SyntaxError)
        }
    }
    fn parse_node_variable(&mut self) -> Result<(), ErrorMessage>{
        // <变量> → <标识符>
        match self.parse_node_identifier() {
            Ok(_) => Ok(()),
            Err(e) => return self.handle_error(e),
        }
    }
    fn parse_node_constant(&mut self) -> Result<(), ErrorMessage>{
        // <常量> → <整数>
        match self.current_token() {
            Token::IntegerLiteral(_) => {
                self.advance();
                Ok(())
            },
            _ => self.handle_error(ErrorMessage::InvalidNumber)
        }
    }
    fn parse_node_identifier(&mut self) -> Result<(), ErrorMessage>{
        // <标识符>
        match self.current_token() {
            Token::Identifier(_) => {
                self.advance();
                Ok(())
            },
            _ => self.handle_error(ErrorMessage::NotFoundDeclarationInThisField)
        }
    }
}