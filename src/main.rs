use pest::Parser;
use pest_derive::Parser;
use std::{fs::File, io::Read};

#[derive(Parser)]
#[grammar = "dafny.pest"]
pub struct DafnyParser;


fn main() {
    
    let mut file = File::open("hello.dfy").unwrap();
    let mut contents = String::new();
    file.read_to_string(&mut contents).unwrap();
    let result = DafnyParser::parse(Rule::Dafny, &contents);
    println!("{:#?}", result);

    println!("Hello, world!");
}
