use std::collections::VecDeque;


#[derive(Debug)]
pub enum PolynomialsToken {
	Space,
	Addition,
	Substraction,
	Multiplication,
	Exponential,
	Equals,
	Expression(String)
}

#[derive(Debug)]
pub struct Tokenizer{
	content: String,
	pub tokens : VecDeque<PolynomialsToken>
}

impl Tokenizer {
	pub fn new(content: String) -> Self {
		Tokenizer {
			content,
			tokens : VecDeque::new()
		}
	}

	fn allocate_expression(&self, start: &mut usize, end: usize) -> PolynomialsToken {
		let slice = &self.content[*start..end];
		*start = end;
		PolynomialsToken::Expression(slice.to_string())
	}

	fn match_char_to_polynome_token(&self, c: char) -> Option<PolynomialsToken> {
		let token = match c {
			'+' => PolynomialsToken::Addition,
			'-' => PolynomialsToken::Substraction,
			'*' => PolynomialsToken::Multiplication,
			'^' => PolynomialsToken::Exponential,
			'=' => PolynomialsToken::Equals,
			_ if c.is_whitespace() => PolynomialsToken::Space,
			_ => return None
		};
		Some(token)
	}

	pub fn tokenize(&mut self) {
		let mut start = 0;
		let indices = self.content.char_indices();
		for (mut curr_pos, c) in indices {
			let token = match self.match_char_to_polynome_token(c){
				Some(t) => t,
				None => {
					if curr_pos + 1 != self.content.len(){
						continue;
					}
					curr_pos += 1;
					PolynomialsToken::Space
				}
			};
			if start != curr_pos {
				let expression = self.allocate_expression(&mut start, curr_pos);
				self.tokens.push_back(expression);
			}
			start += 1;
			if let PolynomialsToken::Space = token {
				continue;
			}
			self.tokens.push_back(token);
		}
	}

}
