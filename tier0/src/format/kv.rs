#![allow(unused_variables, unused_mut)]

use std::fmt::Debug;

#[derive(Debug)]
pub enum Token {
	Open,
	Close,
	Word { value: String }
}

#[derive(Debug)]
pub enum KeyValues {
	Root { values: Vec<KeyValues> },
	KeyValue { key: String, value: String },
	KeyMap { key: String, values: Vec<KeyValues> },
}

pub fn parse( tokens: Vec<Token> ) -> KeyValues {
	let mut vac: Vec<KeyValues> = Vec::new();
	let mut index = 0;
	
	while index < tokens.len() {
		
	}
	
	KeyValues::Root { values: vac }
}

pub fn tokenize( string: &str ) -> Vec< Token > {
	let mut vac: Vec<Token> = Vec::new();
	let mut word = String::new();
	let stringy = string.to_string();
	let data: Vec<&str> = stringy.lines().collect();
	let mut index = 0usize;
	let mut line = 0usize;

	while index < data.len() {

	}

	for chr in string.to_string().chars() {
		match chr {
			' ' => {
				if !word.is_empty() {
					vac.push( Token::Word { value: word.clone() } );
					word.clear();
				}
			},
			'"' => { },
			'{' => vac.push( Token::Open ),
			'}' => vac.push( Token::Close ),
			_ => word.push( chr )
		}
	}
	vac
}

pub(crate) fn main() {
	// FIXME: There's an infinite loop somewhere...
	let vdfdata = "
		a b c d abcd { \"\" \"\" \"}\" { \"a\" b [$cde] } }
		something \"quoted\" /*and*/ something not/quite\\quoted
		key Value_With-Stuff // single-line \"comment\" /* cmon */ { \"ignore\" \"these\" }
		key /* multi-line
		comment. yep, it can go for
		multiple lines */ value /* and they can go between keys/*values */
		\"{\" { \"}\" \"{\" } /* { \"}\" \"*/\"{}\"\"}\"
	";

	let tokens = tokenize( vdfdata );
	println!( "{:?}", tokens );
	let map = parse( tokens );
	println!( "{:?}", map )
}
