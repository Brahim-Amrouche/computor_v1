mod tokenizer;

use tokenizer::Tokenizer;

fn main() {
	let equation = "8 * X^0 - 6 * X^1 + 0 * X^2 - 5.6 * X^3 = 3 * X^0";
	let mut token_generator = Tokenizer::new(equation.into());
	token_generator.tokenize();
	println!("tokens are {:?}",token_generator.tokens)
}
