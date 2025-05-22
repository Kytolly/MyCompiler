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
    ExpectedIdentifier, // 符号无声明
    FoundRepeatDeclarationInThisField, //符号重复声明
}

#[derive(Clone)]
pub struct VariableItem {
    // 变量表项
    pub vname: String, // 变量名
    pub vproc: String, // 所属过程
    pub vkind: i32, // 0-变量，1-形参
    pub vlev: i32, // 变量所在层次
    // pub vadr: i32, // 相对于第一个变量在变量表中的位置
    pub vtype: Vec<i32>, // 变量类型 
}
impl VariableItem {
    pub fn new(vname: String, vproc: String, vkind: i32, vlev: i32)-> Self{
        let v = VariableItem {
            vname: vname,
            vproc: vproc,
            vkind: vkind,
            vlev: vlev,
            vtype: Vec::new(),
        };
        v
    }
}

#[derive(Clone)]
pub struct ProcedureItem {
    // 过程表项
    pub pname: String, // 过程名
    pub ptype: Vec<i32>, //过程类型
    pub plev: i32, // 过程所在层次
    // pub fadr: i32, // 第一个变量在变量表里的位置
    // pub ladr: i32, // 最后一个变量在变量表中的位置
}
impl ProcedureItem {
    pub fn new(pname: String, plev: i32) -> Self {
        let p = ProcedureItem {
            pname: pname, 
            ptype: Vec::new(),
            plev: plev
        };
        p
    }
}

#[derive(Clone)]
pub struct SymbolTable {
    // 符号表,每个作用域都应该对应一个符号表
    pub variables: HashMap<String, VariableItem>, // 变量表
    pub procedures: HashMap<String, ProcedureItem>, // 过程表
    pub level: i32, // 当前作用域层级
}
impl SymbolTable {
    pub fn new(level: i32) -> Self {
        let st = SymbolTable {
            variables: HashMap::new(),
            procedures: HashMap::new(),
            level: level,
        };
        st
    }
    pub fn get_level(&self) -> i32{
        self.level
    }
}

#[derive(Clone)]
pub struct Env {
    // 符号表栈，管理顶层符号表随作用域变化
    pub stack: Vec<SymbolTable>,
}
impl Env {
    pub fn new() -> Self {
        let e = Env {
            stack: Vec::new(),
        };
        e
    }
    pub fn enter_scope(&mut self){
        // 进入作用域，移入一个空符号表
        let t = SymbolTable::new(self.stack.len() as i32);
        self.stack.push(t);
    }
    pub fn exit_scope(&mut self){
        // 退出作用域，移出栈顶符号表
        self.stack.pop();
    }
    pub fn add_variable(&mut self, vname: String, vproc: String, vkind: i32){
        // 声明一个变量
        let t: &mut SymbolTable = self.stack.last_mut().unwrap();
        let item = VariableItem::new(vname.clone(), vproc, vkind, t.get_level());
        t.variables.insert(vname, item);
    }
    pub fn delete_cariable(&mut self, vname: String){
        // 析构一个变量
        let t: &mut SymbolTable = self.stack.last_mut().unwrap();
        t.variables.remove_entry(&vname);
    }
    pub fn add_procedure(&mut self, pname: String) {
        // 声明一个过程
        let t: &mut SymbolTable = self.stack.last_mut().unwrap();
        let item = ProcedureItem::new(pname.clone(), t.get_level());
        t.procedures.insert(pname, item);
    }
    pub fn delete_procedure(&mut self, pname: String) {
        // 析构一个过程
        let t: &mut SymbolTable = self.stack.last_mut().unwrap();
        t.procedures.remove_entry(&pname);
    }
    pub fn find_symbol(&self, name: String) -> bool{
        // 自顶向下查找一个符号
        for s in self.stack.iter().rev() {
            if s.variables.get(&name).is_some()||s.procedures.get(&name).is_some() {
                return true;
            }
        }
        false
    }
    pub fn check_repeat(&self, name: String) -> bool {
        // 检查当前作用域是否重复声明某符号
        let t = self.stack.last().unwrap(); 
        t.variables.contains_key(&name) || t.procedures.contains_key(&name)
    }
    pub fn save(&self){
        // 保存在.var文件
    }
}