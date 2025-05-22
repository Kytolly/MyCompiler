use crate::env::{Token, ErrorMessage, Env};
use std::fs;
use std::io::{Write};

pub struct Parser {
    // LL1语法分析器,基于递归下降办法
    pub stream: Vec<Token>, // 输入的token流
    pub pos: usize, //当前token所在位置
    pub line: usize, // 当前token所在行数
    mode: &'static str, // 错误的打印模式
    name: String,
}

impl Parser {
    pub fn new(s: Vec<Token>, mode: &'static str, name: String) -> Self {
        let p = Parser {
            stream: s,
            pos: 0,
            line: 1,
            mode: mode,
            name: name,
        };
        p
    }
    pub fn analyse(&mut self, env: &mut Env) -> Result<(), ErrorMessage> {
        self.parse_node_program(env)
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
        match self.mode {
            "console" => self.console_error(errmsg),
            "file" => self.file_error(&errmsg),
            _ => println!("invalid mode!"),
        };
    }
    fn file_error(&self, errmsg:&ErrorMessage) {
        let path = format!("{}.err", self.name);
        let mut file = fs::OpenOptions::new()
            .write(true)
            .append(true)
            .create(true)
            .open(&path)
            .expect("Failed to create error file");
        let err_msg = match errmsg {
            ErrorMessage::SyntaxError => format!("LINE{:?}: unknown token!\n", self.line),
            ErrorMessage::WrongReserveYouMeanFunction => format!("LINE{:?}: wrong reserve: you mean 'function'?\n", self.line),
            ErrorMessage::WrongReserveYouMeanRead => format!("LINE{:?}: wrong reserve: you mean 'read'?\n", self.line),
            ErrorMessage::WrongReserveYouMeanWrite => format!("LINE{:?}: wrong reserve: you mean 'write'?\n", self.line),
            ErrorMessage::WrongAssignToken => format!("LINE{:?}: wrong assign operator: you mean ':='?\n", self.line),
            ErrorMessage::InvalidTypeExpectedInterger => format!("LINE{:?}: invalid type: expected INTEGER\n", self.line),
            ErrorMessage::InvalidNumber => format!("LINE{:?}: Invalid number!\n", self.line),
            ErrorMessage::OverflowIdentifier => format!("LINE{:?}: Identifier length overflow!\n", self.line),
            ErrorMessage::FailMatchingSemicolon => format!("LINE{:?}: Semicolon matching failed!\n", self.line),
            ErrorMessage::MissingSemicolon => format!("LINE{:?}: missing a ';' at the end of the statement\n", self.line),
            ErrorMessage::MissingLeftParenthesis => format!("LINE{:?}: expected '(' following the function statement\n", self.line),
            ErrorMessage::MissingRightParenthesis => format!("LINE{:?}: expected ')' to cover the block\n", self.line),
            ErrorMessage::MissingIf => format!("LINE{:?}: expected 'if' \n", self.line),
            ErrorMessage::MissingThen => format!("LINE{:?}: expected 'then' \n", self.line),
            ErrorMessage::MissingElse => format!("LINE{:?}: expected 'else' \n", self.line),
            ErrorMessage::MissingMultiply => format!("LINE{:?}: expected '*' \n", self.line),
            ErrorMessage::SyntaxErrorExpectedABlock => format!("LINE{:?}: syntax error, expected a block\n", self.line),
            ErrorMessage::FailMatching => format!("LINE{:?}: Symbol matching error!\n", self.line),
            ErrorMessage::MissingEnd => format!("LINE{:?}: missing END: this block is not covered\n", self.line),
            ErrorMessage::ExpectedIdentifier => format!("LINE{:?}: Expected identifier in this field\n", self.line),
            ErrorMessage::FoundRepeatDeclarationInThisField => format!("LINE{:?}: this symbol's declaration repeated in this field\n", self.line),
        };
        file.write_all(err_msg.as_bytes()).expect("Failed to write error file");
    }
    fn console_error(&self, errmsg: ErrorMessage) {
        // 抛出错误
        // 这里还是简化实现
        // 应该扔到标准错误流中，写入文件
        // 不能和标准输出流混合
        // self.debug();
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
            ErrorMessage::ExpectedIdentifier => {
                println!("LINE{:?}: expected indentifier", self.line);
            }
            ErrorMessage::FoundRepeatDeclarationInThisField => {
                println!("LINE{:?}: the declaration of this indentifier repeated in this scope", self.line);
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
        self.debug();
        self.error(errmsg);
        self.skip_bad_line();
        res
    }
    
    fn parse_node_program(&mut self, env: &mut Env) -> Result<(), ErrorMessage>{
        // <程序> → <分程序>
        self.parse_node_block(env)
    }
    fn parse_node_block(&mut self, env: &mut Env) -> Result<(), ErrorMessage>{
        // <分程序> → begin <说明语句表><执行语句表> end
        match self.match_token(Token::Begin) {
            true => self.advance(),
            false => return self.handle_error(ErrorMessage::SyntaxErrorExpectedABlock)
        }
        env.enter_scope();
        match self.parse_node_declaration_statement_table(env) {
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
            true => self.advance(),
            false => return self.handle_error(ErrorMessage::MissingEnd)
        }
        env.exit_scope();
        Ok(())
    }
    fn parse_node_declaration_statement_table(&mut self, env:&mut Env) -> Result<(), ErrorMessage>{
        // <说明语句表> → {<说明语句> ;}
        loop {
            if !self.match_token(Token::Integer) {
                // 检查FOLLOW 集
                match self.current_token() {
                    Token::Read | Token::Write | Token::If | Token::Identifier(_) | Token::End | Token::Eof => {
                        return Ok(());
                    },
                    _ => return self.handle_error(ErrorMessage::SyntaxError),
                }
            }
            
            // 匹配说明语句
            match self.parse_node_declaration_statement(env) {
                Ok(_) => (),
                Err(e) => return self.handle_error(e),
            }
            
            // 匹配分号
            match self.match_token(Token::Semicolon) {
                true => self.advance(),
                false => return self.handle_error(ErrorMessage::MissingSemicolon),
            }
        }
    }
    fn parse_node_declaration_statement(&mut self, env: &mut Env) -> Result<(), ErrorMessage>{
        // <说明语句> → integer <说明语句'>
        match self.match_token(Token::Integer) {
            true => self.advance(),
            false => return self.handle_error(ErrorMessage::InvalidTypeExpectedInterger)
        }
        self.parse_node_declaration_statement_prime(env)
    }
    fn parse_node_declaration_statement_prime(&mut self, env: &mut Env) -> Result<(), ErrorMessage>{
        // <说明语句'> → <变量> | function <标识符>（<参数>）<函数体> ;
        if self.match_token(Token::Function) {
            // 函数说明分支
            self.advance();

            // 获取函数标识符名称
            let pname = match self.parse_node_identifier() {
                Ok(pname) => pname,
                Err(e) => return self.handle_error(e),
            };
            // 检查是否重复声明，若没有则添加声明
            if env.check_repeat(pname.clone()){
                return self.handle_error(ErrorMessage::FoundRepeatDeclarationInThisField);
            } else{
                env.add_procedure(pname.clone());
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
            match self.match_token(Token::Semicolon) {
                true => {
                    self.advance();
                    match self.parse_node_function_body(env) {
                        Ok(_) => Ok(()),
                        Err(e) => return self.handle_error(e),
                    }
                },
                false => self.handle_error(ErrorMessage::MissingSemicolon),
            }
        } else {
            // 变量说明分支
            // 获取变量名字
            let vname = match self.parse_node_variable() {
                Ok(vname) => vname,
                Err(e) => return self.handle_error(e),
            };
            // 检查是否重复声明，若没有则添加声明
            if env.check_repeat(vname.clone()){
                return self.handle_error(ErrorMessage::FoundRepeatDeclarationInThisField);
            } else{
                env.add_variable(vname.clone(), "F".to_string(),0);
                Ok(())
            }
        }
    }
    fn parse_node_function_body(&mut self, env:&mut Env) -> Result<(), ErrorMessage>{
        // <函数体> → begin <说明语句表><执行语句表> end
        match self.match_token(Token::Begin) {
            true => self.advance(),
            false => return self.handle_error(ErrorMessage::SyntaxErrorExpectedABlock)
        }
        match self.parse_node_declaration_statement_table(env) {
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
                _ => return self.handle_error(ErrorMessage::SyntaxError),
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
        // <因子> → <标识符> <因子后缀> | <常数> | (<算术表达式>)
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
                match self.parse_node_identifier() {
                    Ok(_) => (),
                    Err(e) => return self.handle_error(e),
                }
                match self.parse_node_factor_suffix() {
                    Ok(_) => (),
                    Err(e) => return self.handle_error(e),
                }
                Ok(())
            },
            _ => self.handle_error(ErrorMessage::SyntaxError)
        }
    }
    fn parse_node_factor_suffix(&mut self) -> Result<(), ErrorMessage>{
        // <因子后缀> → (<参数>)| ε
        match self.match_token(Token::LeftParenthesis) {
            true => self.advance(),
            false => return Ok(()),
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
    fn parse_node_variable(&mut self) -> Result<String, ErrorMessage>{
        // <变量> → <标识符>
        match self.parse_node_identifier() {
            Ok(name) => Ok(name),
            Err(e) => return Err(e),
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
    fn parse_node_identifier(&mut self) -> Result<String, ErrorMessage>{
        // <标识符>
        match self.current_token() {
            Token::Identifier(name) => {
                self.advance();
                Ok(name.clone())
            },
            _ => Err(ErrorMessage::ExpectedIdentifier),
        }
    }
}