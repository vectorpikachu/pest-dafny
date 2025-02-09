use pest::Parser;
use pest_derive::Parser;
use std::{fs::File, io::Read};
use clap::{Command, Arg};
use std::io::Write;

#[derive(Parser)]
#[grammar = "dafny.pest"]
pub struct DafnyParser;

fn main() {
    let matches = Command::new("File Processor")
        .version("1.0")
        .author("VectorPikachu <HanghouLyu@outlook.com>")
        .about("Get the AST of Dafny file.")
        .arg(
            Arg::new("input")
                .help("Sets the input file to use")
                .required(true)
                .long("input")
                .short('i')
        )
        .arg(
            Arg::new("output")
                .help("Sets the output file to write to")
                .required(true)
                .long("output")
                .short('o')
        )
        .get_matches();
    let input: &String = matches.get_one("input").expect("No input file provided");
    let output: &String = matches.get_one("output").expect("No output file provided");
    let mut input_file = File::open(input).unwrap();
    let mut contents = String::new();
    input_file.read_to_string(&mut contents).unwrap();
    let result = DafnyParser::parse(Rule::Dafny, &contents);
    println!("{:#?}", result);

    let mut output_file = File::create(output).unwrap();
    if result.is_ok() {
        writeln!(output_file, "{:#?}", result.unwrap()).unwrap();
    } else {
        writeln!(output_file, "{:#?}", result.unwrap_err()).unwrap();
    }

    println!("Hello, world!");
}
