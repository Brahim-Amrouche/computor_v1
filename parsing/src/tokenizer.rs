use std::{clone, collections::LinkedList};

pub(crate) struct Tokenizer {
	content: String,
	seperators: Vec<char>,
	tokens: LinkedList<String>
}

impl Tokenizer {
	pub(crate) fn new(content: String, seperators: Vec<char>) -> Self{
		Self {
			content,
			seperators,
			tokens: LinkedList::new()
		}
	}

	pub(crate) fn tokenize(&mut self){
		let mut start_pos: usize = 0;
		for (pos, char) in self.content.char_indices() {
			if !self.seperators.contains(&char){
				continue;
			}
			let slice = &self.content[start_pos..=pos];
			self.tokens.push_back(slice.to_string());
			start_pos = pos + 1;
		}
	}

	pub(crate) fn print_tokens(&self){
		for (pos, token) in self.tokens.iter().enumerate() {
			println!("the {pos}th token: {token}");
		}
	}
}
