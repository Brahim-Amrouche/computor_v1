mod executor;
mod interpreter;
mod parser;
mod tokenizer;

use executor::Executor;
use interpreter::Interpreter;
use parser::Parser;
use tokenizer::Tokenizer;

fn main() {
    let equation = "1 * X^0 + 2 * X^1 + 5 * X^2 = 0";
    let mut token_generator = Tokenizer::new(equation.into());
    token_generator.tokenize();
    let mut parser = Parser::new(token_generator);
    match parser.parse() {
        Ok(ast) => {
            // println!("AST successfully created:\n{}", ast);
            match Interpreter::evaluate(&ast) {
                Ok(poly) => {
                    println!("Reduced form: {}", poly);
                    if let Err(err) = Executor::execute(&poly) {
                        eprintln!("{}", err);
                    }
                }
                Err(e) => eprintln!("Evaluation Error: {}", e),
            }
        }
        Err(e) => eprintln!("Error parsing: {}", e),
    }
}
