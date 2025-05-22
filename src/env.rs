use std::collections::HashMap;

#[derive(Clone, PartialEq, Debug)]
pub enum Token {
    // 所有的记号
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

#[derive(Clone)]
pub enum ErrorMessage {
    // 所有的报错信息
    SyntaxError,// 语法错误
    WrongReserveYouMeanFunction, // wrong reserve: you mean 'function'?
    WrongReserveYouMeanRead, // wrong reserve: you mean 'read'?
    WrongReserveYouMeanWrite, // wrong reserve: you mean 'write'?
    WrongAssignToken, // wrong assign operator: you mean ':='?
    InvalidTypeExpectedInterger, // 非法的类型，expected integer
    InvalidNumber, // 非法数字串
    OverflowIdentifier,// 标识符长度溢出
    FailMatchingSemicolon, // 冒号不匹配
    MissingSemicolon, // 缺一个分号,
    MissingLeftParenthesis, // expected '(' following the function statement
    MissingRightParenthesis, // expected ')' to cover the block
    MissingIf, // expected 'if' 
    MissingThen, // expected 'then' 
    MissingElse, // expected 'else'
    MissingMultiply, // expected 'multiply'
    SyntaxErrorExpectedABlock, // expected a block
    FailMatching, // 符号匹配错误
    MissingEnd, // begin没有匹配的end
    NotFoundDeclarationInThisField, // 符号无声明
    FoundRepeatDeclarationInThisField, //符号重复声明
}

#[derive(Clone)]
pub struct VariableItem {
    // 变量表项
    vname: String, // 变量名
    vproc: String, // 所属过程
    vkind: i32, // 0-变量，1-形参
    vlev: i32, // 变量所在层次
    vadr: i32, // 相对于第一个变量在变量表中的位置
    vtype: Vec<i32>, // 变量类型 
}

#[derive(Clone)]
pub struct ProcedureItem {
    // 过程表项
    pname: String, // 过程名
    ptype: Vec<i32>, //过程类型
    plev: i32, // 过程所在层次
    fadr: i32, // 第一个变量在变量表里的位置
    ladr: i32, // 最后一个变量在变量表中的位置
}

#[derive(Clone)]
pub struct Env {
    // 符号表链,每个作用域都应该对应一个符号表
    parent: Option<Box<Env>>, // 父作用域
    variables: HashMap<String, VariableItem>, // 变量表
    procedures: HashMap<String, ProcedureItem>, // 过程表
    current_level: i32, // 当前作用域层级
}

impl Env {
    pub fn new() -> Self {
        Env {
            variables: HashMap::new(),
            procedures: HashMap::new(),
            current_level: 0,
            parent: None,
        }
    }
}