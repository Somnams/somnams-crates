// * scanning => parsing => static analysis =>
// * Intermediate representations => optimization => code gen => virtual machine
// * => runtime

mod ast;

use std::{
    env,
    fs::{self, File},
    io::{stdin, LineWriter, Write},
};

use ast::{report::Report, scanner::Scanner, token::Token};

enum FileType {
    JavaScript,
    Other,
}

fn file_type_check(file_path: &str) -> FileType {
    if file_path.ends_with(".js") {
        return FileType::JavaScript;
    }
    return FileType::Other;
}

fn generate_ast(tokens: Vec<Token>) {
    let file = File::create(".output/token.json").expect("File create error.");
    let mut file = LineWriter::new(file);
    let c = serde_json::to_string(&tokens).unwrap();
    file.write(c.as_bytes()).expect("write error");
}

fn run(source: &str) {
    let mut scanner = Scanner::new(source);
    let tokens = scanner.scan_tokens();
    // TODO write ast result to text.json
    generate_ast(tokens);
}

fn run_files(file_path: &str) {
    // TODO file path
    match file_type_check(file_path) {
        FileType::JavaScript => {
            let f = fs::read_to_string(file_path).expect("Cannot read this file.");
            println!("{f}");
            run(&f);
        }
        FileType::Other => {
            println!("Sorry, it only supports JavaScript now.");
        }
    }
}

fn run_prompt() {
    println!("Please typing your JS code ...");
    loop {
        println!(">");
        let mut input = String::new();
        if stdin().read_line(&mut input).expect("Failed to read line.") == 0 {
            break;
        };
        run(&input);
        println!("message received: {}", input);
    }
}

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() == 1 {
        run_prompt();
    } else if args.len() == 2 {
        run_files(&args[1]);
    } else {
        println!("Usage: cargo run -- [script]");
    }
}
