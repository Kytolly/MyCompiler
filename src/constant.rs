#[derive(Clone, PartialEq)]
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
pub enum ErrorMessage {
    // 非法标识符
    InvalidIdentifier,
    
    // 非法数字串
    InvalidNumber,

    // 标识符长度溢出
    OverflowIdentifier,

    // 冒号不匹配
    FailMatchingSemicolon,
}

