mod parser;
mod tokenizer;

use parser::Parser;
use tokenizer::Tokenizer;

fn main() {
    let equation = "8 * X^0 - 6 * X^1 + 0 * X^2 - 5.6 * X^3 = 3 * X^0";
    let mut token_generator = Tokenizer::new(equation.into());
    token_generator.tokenize();
    let mut parser = Parser::new(token_generator);
    match parser.parse() {
        Ok(ast) => println!("AST successfully created:\n{}", ast),
        Err(e) => eprintln!("Error parsing: {}", e),
    }
}
