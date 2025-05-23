use crate::prep::Preprocessor;
use crate::env::{Token, ErrorMessage};
// use std::fmt::Error;
use std::io::{Write};
use std::fs;
use std::collections::HashMap;

pub struct Lexer {
    name: &'static str, // 源程序名
    source: String, // 源程序字符串
    max_len: usize, // 标识符的最大长度
    mode: &'static str, // 错误的打印模式

    // 处于简化考虑，将符号表分为三个部分
    reserve_table: HashMap<&'static str, Token>, // 关键字表
    word_table: HashMap<String, Token>, // 标识符表
    literal_table: HashMap<i64, Token>, // 常量表，存储整型数字常量

    // 处于简化考虑，没有设计双缓冲区
    cha: Option<char>, // 最新读入的字符
    pos: usize, // cha指针位置
    peek: Option<char>, // 设置缓冲区大小为1
    nxt: usize, // 缓冲区的指针位置

    token: String, // 已读入的字符串 
    line: usize, // 已读入的行数

    
    stream: Vec<Token>, // 已读入的Token流
}

impl Lexer {
    pub fn new(p: Preprocessor, name: &'static str, mode: &'static str) -> Self {
        let mut l = Lexer {
            name: name,
            source: p.content,
            max_len: 16,
            mode: mode,

            reserve_table: HashMap::new(),
            word_table: HashMap::new(),
            literal_table: HashMap::new(),

            cha: None,
            pos: usize::MAX,
            peek: None,
            nxt: 0,

            token: String::new(),
            stream: Vec::new(),
            line: 1,
        };
        l.init_reserve();
        l
    }
    pub fn get_stream(&self) -> Vec<Token> {
        self.stream.clone()
    }
    pub fn analyse(&mut self) {
        // 分词
        self.getchar();
        self.getnbc();
        
        loop {
            self.getnbc();
            let tk = self.current_token();
            println!("[new token]:{} {}", self.get_meaning(&tk), self.token);
            // println!("记录至token流后重置当前token");
            self.stream.push(tk.clone());
            self.token.clear();
            self.peek = None;
            // println!("--------------------------");
            if tk == Token::Eof {
                break;
            }
            // println!("当前指向字符{:?}", self.cha);
        }
    }
    pub fn save(&mut self){
        // 输出为文件
        let path = format!("{}.dyd", self.name);
        let mut file = fs::File::create(&path).expect("创建文件失败");
        for tk in &self.stream {
            let symbol = self.get_symbol(tk);
            let id = if self.get_label(tk) < 10 {
                format!("0{}", self.get_label(tk))
            } else {
                format!("{}", self.get_label(tk))
            };
            let line = format!("{:>16} {}\n", symbol, id);
            file.write_all(line.as_bytes()).expect("写入文件失败");
        }
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
        let mut file = fs::File::create(&path).expect("创建错误文件失败");
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
        file.write_all(err_msg.as_bytes()).expect("写入错误文件失败");
    }
    fn console_error(&self, errmsg: ErrorMessage) {
        // 抛出错误
        // 这里还是简化实现
        // 应该扔到标准错误流中，写入文件
        // 不能和标准输出流混合
        match errmsg {
            ErrorMessage::SyntaxError => {
                println!("LINE{:?}: 语法错误!", self.line);
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
                println!("LINE{:?}: 非法的类型,expected integer!", self.line);
            }
            ErrorMessage::InvalidNumber => {
                println!("LINE{:?}: 非法数字!", self.line);
            }
            ErrorMessage::OverflowIdentifier => {
                println!("LINE{:?}: 标识符长度溢出!", self.line);
            }
            ErrorMessage::FailMatchingSemicolon => {
                println!("LINE{:?}: 冒号匹配失败!", self.line);
            }
            ErrorMessage::MissingSemicolon => {
                println!("LINE{:?}: 句尾缺少分号!", self.line);
            }
            ErrorMessage::MissingLeftParenthesis => {
                println!("LINE{:?}: expected '(' following the function statement", self.line);
            }
            ErrorMessage::MissingRightParenthesis => {
                println!("LINE{:?}: expected ')' to cover the block", self.line);
            }
            ErrorMessage::MissingThen => {
                println!("LINE{:?}: expected 'then' ", self.line);
            }
            ErrorMessage::MissingIf => {
                println!("LINE{:?}: expected 'if' ", self.line);
            }
            ErrorMessage::MissingElse => {
                println!("LINE{:?}: expected 'else' ", self.line);
            }
            ErrorMessage::MissingMultiply => {
                println!("LINE{:?}: expected '*' ", self.line);
            }
            ErrorMessage::SyntaxErrorExpectedABlock => {
                println!("LINE{:?}: syntax error, expected a block!", self.line);
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
    }
    fn init_reserve(&mut self) {
        // 初始化关键字表
        self.reserve_table.insert("integer", Token::Integer);
        self.reserve_table.insert("function", Token::Function);
        self.reserve_table.insert("if", Token::If);
        self.reserve_table.insert("else", Token::Else);
        self.reserve_table.insert("then", Token::Then);
        self.reserve_table.insert("read", Token::Read);
        self.reserve_table.insert("write", Token::Write);
        self.reserve_table.insert("begin", Token::Begin);
        self.reserve_table.insert("end", Token::End);
    }
    fn get_symbol(&self, tk: &Token) -> String {
        // 查查token对应的字符串
        match &tk {
            Token::Begin => "begin".to_string(),
            Token::End => "end".to_string(),
            Token::Integer => "integer".to_string(),
            Token::If => "if".to_string(),
            Token::Then => "then".to_string(),
            Token::Else => "else".to_string(),
            Token::Function => "function".to_string(),
            Token::Read => "read".to_string(),
            Token::Write => "write".to_string(),
            Token::Identifier(s) => s.clone(),
            Token::IntegerLiteral(n) => n.to_string(),
            Token::Equal => "=".to_string(),
            Token::NotEqual => "<>".to_string(),
            Token::LessEqual => "<=".to_string(),
            Token::Less => "<".to_string(),
            Token::GreaterEqual => ">=".to_string(),
            Token::Greater => ">".to_string(),
            Token::Minus => "-".to_string(),
            Token::Multiply => "*".to_string(),
            Token::Assign => ":=".to_string(),
            Token::LeftParenthesis => "(".to_string(),
            Token::RightParenthesis => ")".to_string(),
            Token::Semicolon => ";".to_string(),
            Token::Eol => "\\EOL".to_string(),
            Token::Eof => "\\EOF".to_string(),
            Token::Illegal(c) => c.to_string(),
        }
    }
    fn get_label(&self, tk: &Token) ->i32 {
        // 查Token对应的标号，以供调试使用
        match &tk {
            Token::Begin => 1,
            Token::End => 2,
            Token::Integer => 3,
            Token::If => 4,
            Token::Then => 5,
            Token::Else => 6,
            Token::Function => 7,
            Token::Read => 8,
            Token::Write => 9,
            Token::Identifier(_) => 10,
            Token::IntegerLiteral(_) => 11,
            Token::Equal => 12,
            Token::NotEqual => 13,
            Token::LessEqual => 14,
            Token::Less => 15,
            Token::GreaterEqual => 16,
            Token::Greater => 17,
            Token::Minus => 18,
            Token::Multiply => 19,
            Token::Assign => 20,
            Token::LeftParenthesis => 21,
            Token::RightParenthesis => 22,
            Token::Semicolon =>23,
            Token::Eol => 24,
            Token::Eof => 25,
            _ => 0,
        }
    }
    fn get_meaning(&self, tk: &Token) -> &'static str {
        // 查token对应的中文含义，以供调试使用
        match &tk {
            Token::Integer => "整数类型声明",
            Token::Function => "函数声明",
            Token::If => "条件语句开始",
            Token::Else => "条件语句分支",
            Token::Then => "条件语句分支",
            Token::Read => "读取输入",
            Token::Write => "输出结果",
            Token::Begin => "程序块开始",
            Token::End => "程序块结束",
            Token::LeftParenthesis => "左括号",
            Token::RightParenthesis => "右括号",
            Token::Semicolon => "语句结束符",
            Token::Equal => "等于运算符",
            Token::Minus => "减法运算符",
            Token::Multiply => "乘法运算符",
            Token::LessEqual => "小于等于运算符",
            Token::GreaterEqual => "大于等于运算符",
            Token::NotEqual => "不等于运算符",
            Token::Assign => "赋值运算符",
            Token::Eol => "换行符",
            Token::Eof => "文件终止符",
            Token::Identifier(_) => "标识符",
            Token::IntegerLiteral(_) => "数字串",
            _ => "未知token"
        }
    }
    fn get_peek(&mut self) {
        if self.pos < self.source.len() - 1{
            self.nxt = self.pos + 1;
        }else {
            self.nxt = usize::MAX;
        }

        if self.nxt < self.source.len() {
            self.peek = self.source.chars().nth(self.nxt);
        }else {
            self.peek = None;
        }
        // println!("获得peek符为{:?}", self.peek);
    }
    fn reserve(&self) -> Option<Token> {
        // 对token查关键字表,检索到应返回关键字token，没检索到返回None
        let ident: &String = &self.token;
        let tk_str: &str = ident.as_str();
        match self.reserve_table.get(&tk_str) {
            Some(tk) => Some(tk.clone()),
            _ => None,
        }
    }
    fn word(&mut self) -> Token{
        // 查标识符表
        match self.word_table.get(&self.token){
            Some(_) => Token::Identifier(self.token.clone()),
            _ => {
                let ident = &self.token;
                let tk= Token::Identifier(ident.clone());
                self.word_table.insert(ident.clone(), tk.clone());
                tk
            }
        }
    }
    fn literal(&mut self, num: i64) -> Token{
        // 查常量表
        match self.literal_table.get(&num){
            Some(_) => Token::IntegerLiteral(num),
            _ => {
                let tk= Token::IntegerLiteral(num);
                self.literal_table.insert(num, tk.clone());
                tk
            }
        }
    } 
    fn is_white(&mut self) -> bool{
        // 判断应该跳过的空白符
        match self.cha {
            Some(c) => {
                if c == '\n' {
                    return false
                }else if c.is_whitespace() {
                    return true
                }else {
                    return false
                }
            },
            _ => false,
        }
    }
    fn getnbc(&mut self) {
        // 跳过空白字符
        while self.is_white() {
            self.getchar();
        }
    }
    fn concat(&mut self) {
        // 将cha加入token末尾
        match self.cha {
            Some(c) => {
                self.token.push(c);
            }
            _ => {
                println!("该符号无法拼接token")
            }
        }
    }
    fn is_lu(&self) -> bool {
        // 判断是否为字母或下划线
        match self.cha {
            Some(c) => c.is_alphabetic() || c == '_',
            None => false,
        }
    }
    fn is_dlu(&self) -> bool {
        // 判断是否为数字,字母或下划线
        match self.cha {
            Some(c) => c.is_alphanumeric() || c == '_',
            None => false,
        }
    }
    fn is_d(&self) -> bool {
        // 判断是数字
        match self.cha {
            Some(c) => c.is_digit(10),
            None => false,
        }
    }
    fn peek_is_a(&self) -> bool {
        // 判断下一位能否跟在数字后面
        // 字母不能跟在数字后面
        match self.peek {
            Some(c) => c.is_alphabetic(),
            _ => false,
        }
    }
    fn getchar(&mut self) -> Option<char> {
        // 从源程序读入下一个字符
        if self.pos == usize::MAX {
            self.pos = 0;
        } else {
            self.pos += 1;
        }

        if self.pos >= self.source.len() {
            self.cha = None;
        } else {
            self.cha = self.source.chars().nth(self.pos);
        }

        // println!("{:?}", self.cha);
        if self.cha == Some('\n') {
            self.line += 1;
        }
        self.cha
    }
    fn retract(&mut self) {
        // 回退一个字符
        if self.pos > 0 {
            self.pos -= 1;
        } else {
            self.pos = usize::MAX;
        }

        if self.pos >= self.source.len() {
            self.cha = None;
        } else {
            self.cha = self.source.chars().nth(self.pos);
        }
        self.token.pop();
        // println!("回退到{:?}", self.cha)
    }
    fn skip_bad_line(&mut self) {
        // 处理错误，一直读到换行符
        while self.cha != Some('\n') {
            self.getchar();
        }
    }
    fn lex_digits_str(&mut self) -> Token {
        // 发现需要解析数字串
        let mut res: i64 = 0;
        let mut tk: Token = Token::Illegal('\0'); // Initialize with default value
        loop {
            let Some(dig) = self.cha else { break; };
            match dig.to_digit(10) {
                Some(val) => {
                    res = res * 10 + (val as i64);
                },
                _ => {
                    self.retract();
                    tk = self.literal(res);
                    self.get_peek();
                    if self.peek_is_a() {
                        self.error(ErrorMessage::InvalidNumber);
                        self.skip_bad_line();
                        return Token::Illegal(self.cha.unwrap_or('\0'));
                    }
                    break;
                }
            }
            self.concat();
            self.getchar();
        }
        self.getchar();
        tk
    }
    fn lex_indentifier(&mut self) ->Token {
        // 可能的标识符和关键字
        while !self.is_white() {
            match self.cha {
                Some(_) if self.is_dlu() => {
                    if self.token.len() >= self.max_len {
                        self.error(ErrorMessage::OverflowIdentifier);
                        self.skip_bad_line();
                        break;
                    }
                    self.concat();
                    self.getchar();
                }
                _ => break,
            }
        }
        
        match self.reserve(){
            Some(tk) => tk,
            _ => self.word(),
        }
    }
    fn current_token(&mut self) -> Token {
        // token解析,自动机的实现
        // 实际上这里也是简化实现，所有的fail指针
        self.getnbc();

        // println!("当前字符为{:?}", self.cha);
        match self.cha {
            // 寻找匹配当前输入的状态
            Some('\n') => {
                self.concat();
                self.getchar();
                Token::Eol
            }
            Some(';') => {
                self.concat();
                self.getchar();
                Token::Semicolon
            }
            Some('=') => {
                self.concat();
                self.getchar();
                return Token::Equal
            }
            Some('(') => {
                self.concat();
                self.getchar();
                return Token::LeftParenthesis
            }
            Some(')') => {
                self.concat();
                self.getchar();
                return Token::RightParenthesis
            }
            Some('-') => {
                self.concat();
                self.getchar();
                return Token::Minus
            }
            Some('*') => {
                self.concat();
                self.getchar();
                return Token::Multiply
            }
            Some('<') => {
                self.getchar();
                match self.cha {
                    Some('=') => {
                        self.getchar();
                        Token::LessEqual
                    }
                    Some('>') => {
                        self.getchar();
                        Token::NotEqual
                    }
                    _ => Token::Less
                }
            }
            Some('>') => {
                self.getchar();
                match self.cha {
                    Some('=') => {
                        Token::GreaterEqual
                    }
                    _ => Token::Greater
                }
            }
            Some(':') => {
                self.concat();
                self.getchar();
                match self.cha {
                    Some('=') => {
                        self.concat();
                        self.getchar();
                        Token::Assign
                    }
                    _ => {
                        self.error(ErrorMessage::FailMatchingSemicolon);
                        self.skip_bad_line();
                        Token::Illegal(self.cha.unwrap_or('\0'))
                    }
                }
            }
            Some(_) if self.is_lu() => {
                // println!("预测为关键字/标识符");
                self.get_peek();
                self.lex_indentifier()
            }
            Some(_) if self.is_d() => {
                // println!("预测为数字串");
                self.get_peek();
                self.lex_digits_str()
            }
            None => {return Token::Eof},
            Some(c) =>{
                self.getchar();
                Token::Illegal(c)
            },
        }
    }
    
}