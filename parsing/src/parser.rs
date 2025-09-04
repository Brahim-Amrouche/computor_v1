use crate::tokenizer::{Tokenizer};
use core::{f64};
use std::collections::{LinkedList};

#[derive(Debug,PartialEq, Eq, Clone, Copy)]
enum Operation {
	Plus = '+' as isize,
	Minus = '-' as isize,
	Multiplication = '*' as isize,
	Exponent = '^' as isize,
	Equal = '=' as isize
}

#[derive(Debug)]
pub struct Polynomial {
	coefficient: f64,
	exponent: u64
}

impl Polynomial {
	pub fn new(coefficient: f64, exponent:u64) -> Self{
		Polynomial {
			coefficient,
			exponent
		}
	}

	pub fn set_polynomial_parameters(&mut self, coefficient: f64, exponent:u64) {
		self.coefficient = coefficient;
		self.exponent = exponent;
	}
}

#[derive(Debug)]
enum EquationObject{
	Operation(Operation),
	Number(f64),
	Polynomial(Polynomial)
}

pub struct Parser {
	tokenizer:	Tokenizer,
	interpreted_tokens : Vec<EquationObject>,
	equation: Vec<EquationObject>,
}

impl Parser {
	pub fn new(content: String) -> Self {
		let seperators = "*^+-=";
		Self { 
			tokenizer: Tokenizer::new(content, seperators.chars().collect()),
			interpreted_tokens: Vec::new(),
			equation: Vec::new()
		}
	}

	fn tokenize(&mut self){
		self.tokenizer.tokenize();
	}

	fn interpret_tokens(&mut self){
		for token in self.tokenizer.tokens.iter().map(|s| s.as_str()){
			let trimed_token = token.trim();
			let token_interpretation = match trimed_token{
				"+" => EquationObject::Operation(Operation::Plus),
				"-" => EquationObject::Operation(Operation::Minus),
				"*" => EquationObject::Operation(Operation::Multiplication),
				"^" => EquationObject::Operation(Operation::Exponent),
				"=" => EquationObject::Operation(Operation::Equal),
				t if t == "X" || t == "x" => {
					EquationObject::Polynomial(Polynomial::new(0.0, 0))
				},
				_ => {
					
					let number = token.trim().to_string().parse::<f64>().unwrap();
					EquationObject::Number(number as f64)
				} 
			};
			self.interpreted_tokens.push(token_interpretation);
		}
	}

	// fn has_polynomial_coefficient(interpreted_tokens: &Vec<EquationObject>, i: usize) -> Result<f64, &str>{
	// 	if let None = interpreted_tokens.get(i - 2){
	// 		return Err("No coefficient given")
	// 	}
	// 	match &interpreted_tokens[i - 1] {
	// 		EquationObject::Operation(o) if *o == Operation::Multiplication => (),
	// 		_ => return  Err("Wrong Equation format: Missing *")
	// 	};
	// 	let coefficient = match interpreted_tokens[i - 2] {
	// 		EquationObject::Number(n) => n,
	// 		_ => return  Err("Wrong Equation Format: Missing coefficient")
	// 	};
	// 	Ok(coefficient)
	// }

	// fn has_polynomial_exponent(interpreted_tokens: &Vec<EquationObject>, i:usize) -> Result<u64, &str> {
	// 	if let None = interpreted_tokens.get(i + 2) {
	// 		return Err("No exponent given")
	// 	}
	// 	match &interpreted_tokens[i + 1]{
	// 		EquationObject::Operation(o) if *o == Operation::Exponent => (),
	// 		_ => return  Err("Wrong equation format: Missing ^")
	// 	}
	// 	let exponent = match interpreted_tokens[i + 2] {
	// 		EquationObject::Number(n) => n,
	// 		_ => return Err("Wrong equation format: Missing exponent")
	// 	};
	// 	Ok(exponent as u64)
	// }


	// fn is_equation_number(interpreted_tokens: &Vec<EquationObject>, i:&mut usize) -> Result<EquationObject, &str>{
	// 	while *i < interpreted_tokens.len() {
			
	// 		*i += 1;
	// 	}
	// 	Ok(EquationObject::Number(0.0))
	// }

	// fn reduce_token_interpretation(interpreted_tokens: &Vec<EquationObject>) -> Result<Vec<EquationObject>, &str>{
	// 	let mut i  = 0;
	// 	let mut equation = Vec::new();
	// 	while i < interpreted_tokens.len() {
	// 		let equation_object =  match &interpreted_tokens[i] {
	// 			EquationObject::Number(_) => {
					
	// 			},
	// 			EquationObject::Operation(_) =>{

	// 			}
	// 			_ => continue
	// 		};
	// 		equation.push(equation_object);
	// 		i += 1;
	// 	};
	// 	Ok(equation)
	// }

	// fn check_equation_validity(&self) -> Result<(), &str>{
	// 	for i in 0..self.equation.len(){
	// 		if let EquationObject::Operation(o) = &self.equation[i] 
	// 			&& (*o == Operation::Minus || *o == Operation::Plus)
	// 			&& let EquationObject::{

	// 		}
	// 	}
	// 	Ok(())
	// }

	pub fn parse(&mut self) -> Result<(), &str>{
		self.tokenize();
		self.tokenizer.print_tokens();
		self.interpreted_tokens.reserve(self.tokenizer.tokens.len());
		self.interpret_tokens();
		println!("the interepreted tokens are: {:?}", self.interpreted_tokens);
		// self.equation = Parser::reduce_token_interpretation(&self.interpreted_tokens)?;
		// self.check_equation_validity();
		Ok(())
	}
}