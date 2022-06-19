use std::fs::read_to_string;
use std::path::Path;

#[derive(Debug, Clone)]
pub struct KeyValue {
	key: String,
	value: E,
}

#[derive(Debug, Clone)]
pub enum E {
	Integer { val: i64 },
	String { val: String },
	List { values: Vec<E> },
	Map { values: Vec<KeyValue> },
	Object { class: String, values: Vec<KeyValue> },
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub(crate) enum TokType {
	Word,
	Semicolon,
	Dot,

	Padding,
	Newline,
	EOF,

	Comment,
	Key,
	Value,
	Class,
}

#[derive(Debug, Clone)]
pub(crate) enum TokValue {
	String { value: String },
	Int { value: i64 },
	Unsigned { value: usize },
	None,
}

#[derive(Debug, Clone)]
pub(crate) struct Loc {
	file: String,
	line: usize,
	char: usize,
}

#[derive(Debug, Clone)]
pub(crate) struct Token {
	typ: TokType,
	value: TokValue,
	loc: Loc,
}

fn eof( last: &Token ) -> Token {
	Token { typ: TokType::EOF, value: TokValue::None, loc: last.loc.clone() }
}

struct Tokenizer {
	tokens: Vec<Token>,
	line: usize,
	char: usize,
	index: usize,
	data: Vec<char>,
	file: String
}

impl Tokenizer {
	fn new( string: &str, file: &str ) -> Self {
		let mut stringy = string.to_string();
		stringy.remove_matches('\r');
		Tokenizer {
			tokens: vec![],
			line: 1,
			char: 1,
			index: 0,
			data: stringy.chars().collect(),
			file: file.to_string(),
		}
	}

	fn get_char_o( &mut self, offset: usize ) -> char {
		self.data[ self.index + offset ]
	}

	fn get_char( &mut self ) -> char {
		self.data[ self.index as usize ]
	}

	fn add_w_loc( &mut self, typ: TokType, value: TokValue, loc: Loc ) -> () {
		self.tokens.push( Token { typ, value, loc } )
	}

	fn add( &mut self, typ: TokType, value: TokValue ) -> () {
		self.tokens.push(
			Token {
				typ,
				value,
				loc: Loc {
					file: self.file.clone(),
					line: self.line,
					char: self.char
				}
			}
		)
	}

	fn tokenize( &mut self ) -> Vec<Token> {
		let delimiters: Vec<char> = vec![':', '\n', '\0', '#' ];
		while self.index < self.data.len() {
			match self.get_char() {
			    '#' => {
				    let mut string = String::new();
				    while self.get_char() != '\n' {
					    string.push( self.get_char() );
					    self.index += 1;
				    }
				    self.add_w_loc(
					    TokType::Comment,
					    TokValue::String { value: string.clone() },
					    Loc {
						    file: self.file.clone(),
						    line: self.line,
						    char: self.char + string.len()
					    }
				    )
			    }
				'\t' => {
					let mut count = 1;
					while self.get_char_o( 1 ) == '\t' {
						count += 1;
						self.index += 1;
					}
					self.add( TokType::Padding, TokValue::Unsigned { value: count } );
					self.char += 1 + count;
					self.index += 1;
				}
				':' => {
					self.add( TokType::Semicolon, TokValue::None );
					self.index += 1;
					self.char += 1;
				}
				'\n' => {
					self.add( TokType::Newline, TokValue::None );
					self.line += 1;
					self.char = 1;
					self.index += 1;
				}
				'.' => {
					self.add( TokType::Dot, TokValue::None );
					self.index += 1;
					self.char += 1;
				}
				' ' => {
					self.index += 1;
					self.char += 1;
				}
				_ => {
					let mut string = String::new();
					string.push( self.get_char() );

					while self.data.len() - 1 > self.index && !delimiters.contains( &self.get_char_o( 1 ) ) {
						string.push( self.get_char_o(1) );
						self.index += 1;
					}

					self.index += 1;
					self.char += string.len() + 1;
					self.add( TokType::Word, TokValue::String { value: string } );
				},
			}
		}
		self.add_w_loc( TokType::EOF, TokValue::None, Loc { file: self.file.clone(), line: self.line + 1, char: 0 } );

		self.tokens.clone()
	}
}

struct Parser {
	tok_list: Vec<Token>,
	processed_tokens: Vec<Token>,
	index: usize
}

impl Parser {
	fn new( tok_list: Vec<Token> ) -> Self {
		Parser {
			tok_list: tok_list,
			processed_tokens: Vec::new(),
			index: 0
		}
	}

	fn peek( &mut self ) -> &Token {
		if self.tok_list.len() > self.index + 1 {
			&self.tok_list[ self.index + 1 ]
		} else {
			self.tok_list.last().unwrap()
		}
	}

	fn peek_o(  &mut self, offset: usize ) -> &Token {
		if self.tok_list.len() > self.index + offset {
			&self.tok_list[ self.index + offset ]
		} else {
			self.tok_list.last().unwrap()
		}
	}

	fn consume( &mut self ) -> &Token {
		self.index += 1;
		&self.tok_list[ self.index - 1 ]
	}


	fn peekType( &mut self ) -> &TokType {
		&self.peek().typ
	}

	fn peek_type_o( &mut self, offset: usize ) -> &TokType {
		&self.peek_o( offset ).typ
	}

	fn peekIsType( &mut self, offset: usize, typ: TokType ) -> bool {
		self.peek_type_o( offset ) == &typ
	}

	fn discard( &mut self ) -> () {
		self.consume();
	}

	fn add( &mut self, typ: TokType, tok: Token ) -> () {
		self.processed_tokens.push( Token { typ: typ.clone(), value: tok.value.clone(), loc: tok.loc.clone() } )
	}

	fn parse( &mut self ) -> Vec<Token> {
		while self.tok_list.len() > self.index {
			let mut tok: Option<Token> = None;
			let mut typ: Option<TokType> = None;

			if self.peekIsType( 0, TokType::Word ) && self.peekIsType( 1, TokType::Semicolon ) {
				tok = Some( self.consume().clone() );
				typ = Some( TokType::Key );
			} else if ( self.peekIsType( 0, TokType::Semicolon ) || self.peekIsType( 0, TokType::Padding ) ) && self.peekIsType( 1, TokType::Word ) && !self.peekIsType( 2, TokType::Semicolon ) {
				let prev = self.consume().clone();
				if prev.typ == TokType::Padding {
					self.processed_tokens.push( prev );
				}
				tok = Some( self.consume().clone() );
				typ = Some( TokType::Value );
			} else if self.peekIsType( 0, TokType::Dot ) && self.peekIsType( 1, TokType::Word ) && self.peekIsType( 2, TokType::Semicolon ) {
				self.discard();
				tok = Some( self.consume().clone() );
				self.discard();
				typ = Some( TokType::Class );
			} else if vec![ TokType::Padding, TokType::Newline, TokType::EOF ].contains( self.peekType() ) {
				let tokk = self.consume().clone();
				self.processed_tokens.push( tokk );
			} else {
				self.discard();
			}

			if tok.is_some() && typ.is_some() {
				self.add( typ.unwrap(), tok.unwrap() );
			}
		}

		self.processed_tokens.clone()
	}
}

pub(crate) fn tokenize( string: &str, file: &str ) -> Vec<Token> {
	Tokenizer::new(string, file).tokenize()
}

pub(crate) fn parse( tok_list: Vec<Token> ) -> Vec<Token> {
	Parser::new(tok_list).parse()
}

pub(crate) fn main() {
	let tokens = tokenize( read_to_string( Path::new("test.e") ).unwrap().as_str(), "test.e" );
	println!( "Tokens: {:?}", tokens );
	let parsed_tokens = parse( tokens );
	println!( "Tokens: {:?}", parsed_tokens )

}
