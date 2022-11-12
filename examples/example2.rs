use fluid_parser::lexer::Lexer;
use fluid_parser::parser::Parser;

fn main() {
    let args: Vec<String> = std::env::args().collect();
    let s = std::fs::read_to_string(&args[1]).expect("Failed to read file!");
    let l = Lexer::new(&s);
    let mut p = Parser::new(l);
    let a = p.parse();
    dbg!(a);
}
