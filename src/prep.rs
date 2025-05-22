use std::fs::File;
use std::io::{Read};

pub struct Preprocessor {
    pub path: String, // 源程序名
    pub content: String, // 源程序字符流
}

impl Preprocessor {
    pub fn new(name: &str) -> Self{
        let mut p = Preprocessor{
            path: name.to_string() + ".pas",
            content: String::new(),
        };
        let mut input = File::open(&p.path).unwrap();
        input.read_to_string(&mut p.content).unwrap();
        println!("{}", p.content);
        p
    }
}

