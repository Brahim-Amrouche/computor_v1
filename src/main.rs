mod executor;
mod interpreter;
mod parser;
mod tokenizer;

use std::env;
use std::io::{self, BufRead, Write};

use executor::Executor;
use interpreter::Interpreter;
use parser::Parser;
use tokenizer::Tokenizer;

pub struct Computor {
    input: String,
}

impl Computor {
    pub fn new(input: String) -> Self {
        Self { input }
    }

    pub fn run(&self) {
        let mut token_generator = Tokenizer::new(self.input.clone());
        token_generator.tokenize();
        let mut parser = Parser::new(token_generator);

        match parser.parse() {
            Ok(ast) => match Interpreter::evaluate(&ast) {
                Ok(poly) => {
                    println!("Reduced form: {}", poly);
                    if let Err(err) = Executor::execute(&poly) {
                        eprintln!("{}", err);
                    }
                }
                Err(e) => eprintln!("Evaluation Error: {}", e),
            },
            Err(e) => eprintln!("Error parsing: {}", e),
        }
    }
}

fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() > 1 {
        let input = args[1].clone();
        let app = Computor::new(input);
        app.run();
    } else {
        let stdin = io::stdin();
        let mut handle = stdin.lock();
        print!("> ");
        io::stdout().flush().unwrap();

        let mut buffer = String::new();
        match handle.read_line(&mut buffer) {
            Ok(bytes) if bytes > 0 => {
                let input = buffer.trim().to_string();
                if !input.is_empty() {
                    let app = Computor::new(input);
                    app.run();
                }
            }
            Ok(_) => {
                // EOF reached with 0 bytes read
            }
            Err(e) => {
                eprintln!("Error reading standard input: {}", e);
            }
        }
    }
}
