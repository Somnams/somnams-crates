use ast::lexer::Lexer;

mod ast;

fn main() {
    let input = "2 + 1";
    let lexer = Lexer::new(input);    
    println!("lexer: {:#?}", lexer);
}