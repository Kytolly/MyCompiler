use crate::prep::Preprocessor;
use crate::constant::{Token, ErrorMessage};
use std::fmt::Error;
use std::io::{Write};
use std::fs;
use std::collections::HashMap;

pub struct Lexer {
    pub name: String, // 源程序名
    pub source: String, // 源程序字符串
    pub max_len: usize, // 标识符的最大长度

    // 处于简化考虑，将符号表分为三个部分
    pub reserve_table: HashMap<&'static str, Token>, // 关键字表
    pub word_table: HashMap<String, Token>, // 标识符表
    pub literal_table: HashMap<i64, Token>, // 常量表，存储整型数字常量

    // 处于简化考虑，没有设计双缓冲区
    pub cha: Option<char>, // 最新读入的字符
    pub pos: usize, // cha指针位置
    pub peek: Option<char>, // 设置缓冲区大小为1
    pub nxt: usize, // 缓冲区的指针位置

    pub token: String, // 已读入的字符串 
    pub stream: Vec<Token>, // 已读入的Token流
    pub line: usize, // 已读入的行数
}

impl Lexer {
    pub fn new(p: Preprocessor) -> Self {
        let mut l = Lexer {
            name: p.name,
            source: p.content,
            max_len: 16,

            reserve_table: HashMap::new(),
            word_table: HashMap::new(),
            literal_table: HashMap::new(),

            cha: None,
            pos: usize::MAX,
            peek: None,
            nxt: 0,

            token: String::new(),
            stream: Vec::new(),
            line: 0,
        };
        l.init_reserve();
        l
    }
    pub fn analyse(&mut self) {
        // 分词
        self.getchar();
        self.getnbc();
        
        loop {
            self.getnbc();
            let tk = self.current_token();
            println!("检测到新的token,其含义为: {} {}", self.get_meaning(&tk), self.token);
            println!("记录至token流后重置当前token");
            self.stream.push(tk.clone());
            self.token.clear();
            self.peek = None;
            println!("--------------------------");
            if tk == Token::Eof {
                break;
            }
            println!("当前指向字符{:?}", self.cha);
        }
    }
    pub fn save(&mut self){
        // 输出为文件
        let path = format!("{}.dyd", self.name);
        let mut file = fs::File::create(&path).expect("创建文件失败");
        for tk in &self.stream {
            let id = if self.get_label(tk) < 10 {
                format!("0{}", self.get_label(tk))
            } else {
                format!("{}", self.get_label(tk))
            };
            let line = format!("{} {}\n", id, self.get_symbol(tk));
            file.write_all(line.as_bytes()).expect("写入文件失败");
        }
    }
    fn output_error(&self, errmsg:&ErrorMessage) {
        let path = format!("{}.err", self.name);
        let mut file = fs::File::create(&path).expect("创建错误文件失败");
        let err_msg = match errmsg {
            ErrorMessage::InvalidIdentifier => format!("LINE{:?}: 非法标识符！", self.line),
            ErrorMessage::InvalidNumber => format!("LINE{:?}: 非法数字！", self.line),
            ErrorMessage::OverflowIdentifier => format!("LINE{:?}: 标识符长度溢出！", self.line),
            ErrorMessage::FailMatchingSemicolon => format!("LINE{:?}: 冒号匹配失败！", self.line),
        };
        file.write_all(err_msg.as_bytes()).expect("写入错误文件失败");
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
        println!("获得peek符为{:?}", self.peek);
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

        println!("{:?}", self.cha);
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
        println!("回退到{:?}", self.cha)
    }
    fn handle_error(&mut self) {
        // 处理错误，一直读到换行符
        while self.cha != Some('\n') {
            self.getchar();
        }
    }
    fn error(&self, errmsg: ErrorMessage) {
        // 抛出错误
        // 这里还是简化实现
        // 应该扔到标准错误流中，写入文件
        // 不能和标准输出流混合
        match errmsg {
            ErrorMessage::InvalidIdentifier => {
                println!("LINE{:?}: 非法标识符！", self.line);
            }
            ErrorMessage::InvalidNumber => {
                println!("LINE{:?}: 非法数字！", self.line);
            }
            ErrorMessage::OverflowIdentifier => {
                println!("LINE{:?}: 标识符长度溢出！", self.line);
            }
            ErrorMessage::FailMatchingSemicolon => {
                println!("LINE{:?}: 冒号匹配失败！", self.line);
            }
        }
    }
    fn parse_digits_str(&mut self) -> Token {
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
                        self.handle_error();
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
    fn parse_indentifier(&mut self) ->Token {
        // 可能的标识符和关键字
        while !self.is_white() {
            match self.cha {
                Some(_) if self.is_dlu() => {
                    if self.token.len() >= self.max_len {
                        self.error(ErrorMessage::OverflowIdentifier);
                        self.handle_error();
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

        println!("当前字符为{:?}", self.cha);
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
                        self.handle_error();
                        Token::Illegal(self.cha.unwrap_or('\0'))
                    }
                }
            }
            Some(_) if self.is_lu() => {
                println!("预测为关键字/标识符");
                self.get_peek();
                self.parse_indentifier()
            }
            Some(_) if self.is_d() => {
                println!("预测为数字串");
                self.get_peek();
                self.parse_digits_str()
            }
            None => {return Token::Eof},
            Some(c) =>{
                self.getchar();
                Token::Illegal(c)
            },
        }
    }
}