use std::fs::File;
use std::io::{Read};

pub struct Preprocessor {
    pub name: String, // 源程序名
    pub content: String,
}

impl Preprocessor {
    pub fn new(path: &str) -> Self{
        let mut p = Preprocessor{
            name: String::from(path),
            content: String::new(),
        };
        let mut input = File::open(path).unwrap();
        input.read_to_string(&mut p.content).unwrap();
        println!("{}", p.content);
        p
    }
}

