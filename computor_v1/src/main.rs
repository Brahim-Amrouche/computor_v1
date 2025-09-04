use parsing::parser;
use std::env;

fn main() {
	let mut args:Vec<String> = env::args().collect();
	if args.len() != 2 {
		println!("I accept 1 argument as such: '5 * X^0 + 4 * X^1 - 9.3 * X^2 = 1 * X^0'");
		return;
	}
	let mut parser = parser::Parser::new(args.remove(1));
	let _ = parser.parse();
}
