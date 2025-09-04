use crate::tokenizer::{self, Tokenizer};

pub struct Polynomial {
	coefficient: f64,
	exponent: u64
}

pub struct Parser {
	tokenizer: Tokenizer
}

impl Parser {
	pub fn new(content: String) -> Self {
		let seperators = "*^+-=";
		Self { tokenizer: Tokenizer::new(content, seperators.chars().collect()) }
	}

	fn tokenize(&mut self){
		self.tokenizer.tokenize();
	}

	pub fn parse(&mut self){
		self.tokenize();
		println!("Tokenizing Done");
		self.tokenizer.print_tokens();
	}
}