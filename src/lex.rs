use crate::prep::Preprocessor;
use std::io::{Write};
use std::fs;

pub struct Lexer {
    pub name: String, // 源程序名
    pub source: String, // 源程序字符串
    pub cha: Option<char>, // 最新读入的字符
    pub pos: usize, // cha所在位置
    pub token: String, // 已读入的字符串 
    pub stream: Vec<Token>, // 已读入的Token流
    pub max_len: usize, // 标识符的最大长度
}

impl Lexer {
    pub fn new(p: Preprocessor) -> Self {
        let mut l = Lexer {
            name: p.name,
            source: p.content,
            cha: None,
            pos: usize::MAX,
            token: String::new(),
            stream: Vec::new(),
            max_len: 16,
        };
        l.getchar();
        l
    }
    fn get_label(&self, tk: &Token) ->i32 {
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
    fn get_symbol(&self, tk: &Token) -> String {
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

    // 从源程序读入下一个字符
    pub fn getchar(&mut self) -> Option<char> {
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
        self.cha
    }

    // 判断应该跳过的空白符
    pub fn is_white(&mut self) -> bool{
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

    // 跳过空白字符
    pub fn getnbc(&mut self) {
        while self.is_white() {
            self.getchar();
        }
    }
    
    // 将cha加入token末尾
    pub fn concat(&mut self) {
        match self.cha {
            Some(c) => {
                self.token.push(c);
            }
            _ => {
                println!("该符号无法拼接token")
            }
        }
    }

    // 判断是否为字母或下划线
    pub fn is_lu(&self) -> bool {
        match self.cha {
            Some(c) => c.is_alphabetic() || c == '_',
            None => false,
        }
    }

    // 判断是否为数字,字母或下划线
    pub fn is_dlu(&self) -> bool {
        match self.cha {
            Some(c) => c.is_alphanumeric() || c == '_',
            None => false,
        }
    }

    // 判断是数字
    pub fn is_d(&self) -> bool {
        match self.cha {
            Some(c) => c.is_digit(10),
            None => false,
        }
    }

    // peek可能的标识符和关键字
    pub fn peek_indentifier(&mut self) {
        while !self.is_white() {
            match self.cha {
                Some(_) => {
                    if self.is_dlu() {
                        self.concat();
                        self.getchar();
                    } else {
                        // Self::error(ErrorMessage::InvalidIdentifier);
                        break;
                    }
                }
                None => break,
            }
        }
        println!("读到空白符停止并生成可能的token");
        if self.token.len() > self.max_len {
            Self::error(ErrorMessage::OverflowIdentifier)
        }
    }

    // peek可能的数字串
    pub fn peek_number(&mut self) {
        while !self.is_white() {
            let Some(_) = self.cha else { break; };
            if self.is_d() {
                self.concat();
                self.getchar();
            } else {
                Self::error(ErrorMessage::InvalidNumber);
                break;
            }
        }
    }

    // 对token查关键字表
    fn reverse(&self) -> Token {
        let ident = &self.token;
        match ident.as_str() {
            "integer" => Token::Integer,
            "function" => Token::Function,
            "if" => Token::If,
            "else" => Token::Else,
            "then" => Token::Then,
            "read" => Token::Read,
            "write" => Token::Write,
            "begin" => Token::Begin,
            "end" => Token::End,
            _ => Token::Identifier(ident.to_string()),
        }
    }

    // 查常量表
    fn literal(&self) -> Token {
        let num = &self.token;
        match num.parse::<i64>() {
            Ok(n) => Token::IntegerLiteral(n),
            Err(_) => Token::Illegal(self.cha.unwrap_or('\0')),
        }
    }

    // 处理错误
    fn error(errmsg: ErrorMessage) {
        match errmsg {
            // ErrorMessage::InvalidIdentifier => {
            //     println!("LINE: 非法标识符！")
            // }
            ErrorMessage::InvalidNumber => {
                println!("LINE: 非法数字！")
            }
            ErrorMessage::OverflowIdentifier => {
                println!("LINE: 标识符长度溢出！")
            }
            ErrorMessage::FailMatchingSemicolon => {
                println!("LINE: 冒号匹配失败！")
            }
        }
    }

    // token解析
    fn current_token(&mut self) -> Token {
        self.getnbc();

        println!("开始匹配当前字符{:?}", self.cha);
        match self.cha {
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
                        Self::error(ErrorMessage::FailMatchingSemicolon);
                        Token::Illegal(self.cha.unwrap_or('\0'))
                    }
                }
            }
            Some(_) if self.is_lu() => {
                println!("预测为标识符");
                self.peek_indentifier();
                return self.reverse()
            }
            Some(_) if self.is_d() => {
                println!("预测为数字串");
                self.peek_number();
                return self.literal()
            }
            None => {return Token::Eof},
            Some(c) =>{return Token::Illegal(c)},
        }
    }

    // 分词
    pub fn analyse(&mut self) {
        loop {
            let tk = self.current_token();
            println!("检测到新的token,其含义为: {} {}", self.get_meaning(&tk), self.token);
            println!("记录至token流后重置当前token");
            self.stream.push(tk.clone());
            self.token.clear();
            println!("--------------------------");
            if tk == Token::Eof {
                break;
            }
            println!("当前指向字符{:?}", self.cha);
        }
    }

    // 输出为文件
    pub fn save(&mut self){
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
}

#[derive(PartialEq)]
#[derive(Debug)]
#[derive(Clone)]
pub enum Token {
    // 标识符
    Identifier(String),
    
    // 字面量
    IntegerLiteral(i64),

    // 算术运算符
    Minus,
    Multiply,
    Assign,

    // 关系运算符
    Equal,
    NotEqual,
    Less,
    LessEqual,
    Greater,
    GreaterEqual,

    // 分界符
    Begin,
    End,
    LeftParenthesis,
    RightParenthesis,
    Semicolon,

    // 行末提示符
    Eol,

    // 文件末提示符
    Eof,

    // 关键字
    Integer,
    Function,
    If,
    Then,
    Else,
    Read,
    Write,

    // 非法字符
    Illegal(char),
}

#[derive(Debug)]
enum ErrorMessage {
    // 非法标识符
    // InvalidIdentifier,
    
    // 非法数字串
    InvalidNumber,

    // 标识符长度溢出
    OverflowIdentifier,

    // 冒号不匹配
    FailMatchingSemicolon,
}