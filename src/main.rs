use pest::{iterators::Pairs, Parser};
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
    let mut output_file = File::create(output).unwrap();
    if result.is_ok() {
        let pairs = result.unwrap();
        let mut buf = Vec::new();
        print_tight_ast(pairs, 0, &mut buf);
        writeln!(output_file, "{}", String::from_utf8(buf).unwrap()).unwrap();
    } else {
        writeln!(output_file, "{:#?}", result.unwrap_err()).unwrap();
    }
}

fn print_tight_ast(pairs: Pairs<'_, Rule>, level: usize, buf: &mut Vec<u8>) {
    if pairs.len() == 0 {
        return;
    }
    if pairs.len() == 1 {
        let rule = pairs.clone().last().unwrap().as_rule();
        match rule {
            Rule::Dafny => {
                print_ident(level, buf);
                print!("- {:?} ", rule);
                write!(buf, "- {:?}", rule).unwrap();
            }
            _ => {
                print!("> {:?} ", rule);
                write!(buf, "> {:?}", rule).unwrap();
            }
        }
        print_tight_ast(pairs.last().unwrap().into_inner(), level + 1, buf);
    } else {
        for pair in pairs {
            println!();
            writeln!(buf).unwrap();
            print_ident(level, buf);
            print!("- {:?} ", pair.as_rule());
            write!(buf, "- {:?}", pair.as_rule()).unwrap();
            print_tight_ast(pair.into_inner(), level + 1, buf);
        }
    }
}

fn print_ident(level: usize, buf: &mut Vec<u8>) {
    for _ in 0..level {
        print!(" ");
        write!(buf, " ").unwrap();
    }
}