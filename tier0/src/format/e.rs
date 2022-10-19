use std::fmt::{Display, Formatter};
use std::fs::read_to_string;
use std::path::Path;

#[derive(Debug, Clone, PartialEq)]
pub struct KeyValue {
	key: String,
	value: E,
}

impl Display for KeyValue {
	fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
		f.write_fmt( format_args!( "{}: {}", self.key, self.value ) )
	}
}

#[derive(Debug, Clone, PartialEq)]
pub enum E {
	None,
	Integer { val: i64 },
	Float { val: f64 },
	String { val: String },
	List { values: Vec<E> },
	Map { values: Vec<KeyValue> },
	Object { class: String, fields: Vec<KeyValue> },
}

#[allow(unused_variables)]
impl Display for E {
	fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
		match self {
			E::None => f.write_str( "None" ),
			E::Integer { val } => f.write_fmt( format_args!( "{}", val ) ),
			E::Float { val } => f.write_fmt( format_args!( "{}", val ) ),
			E::String { val } => f.write_fmt( format_args!("\"{val}\"") ),
			E::List { values } => {
				f.write_str("[")?;
				for value in values {
					f.write_fmt(format_args!("{value}") )?;
					if value != values.last().unwrap() {
						f.write_str("," )?;
					}
				}
				f.write_str("]")
			},
			E::Map { values } => {
				f.write_str("{")?;
				for keyVal in values {
					f.write_fmt(format_args!( "{}: {}", keyVal.key, keyVal.value ) )?;
					if keyVal != values.last().unwrap() {
						f.write_str("," )?;
					}
				}
				f.write_str("}")
			},
			E::Object { class, fields } => {
				f.write_fmt(format_args!("{{class: \"{class}\", fields: {{") )?;
				for field in fields {
					f.write_fmt(format_args!("{}: {}", field.key, field.value ) )?;
					if field != fields.last().unwrap() {
						f.write_str("," )?;
					}
				}
				f.write_str("}}")
			},
		}
	}
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum TokType {
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

#[derive(Debug, Clone, PartialEq)]
pub enum TokValue {
	String { value: String },
	Int { value: i64 },
	Float { value: f64 },
	None
}

#[derive(Debug, Clone)]
pub struct Loc {
	file: String,
	line: usize,
	char: usize
}

#[derive(Debug, Clone)]
pub struct Token {
	typ: TokType,
	value: TokValue,
	padding: usize,
	loc: Loc
}

impl Token {
	fn new( typ: TokType, val: Token ) -> Self {
		Token {
			typ: typ.clone(),
			value: val.value,
			padding: val.padding,
			loc: val.loc
		}
	}

	fn stringValue(&self) -> String {
		match &self.value {
			TokValue::String { value } => value.to_string(),
			TokValue::Int { value } => value.to_string(),
			TokValue::Float { value } => value.to_string(),
			TokValue::None => "None".to_string()
		}
	}
}

struct Tokenizer {
	tokens: Vec<Token>,
	line: usize,
	char: usize,
	index: usize,
	padding: usize,
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
			padding: 0,
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

	fn add( &mut self, typ: TokType, value: TokValue ) -> () {
		self.tokens.push(
			Token {
				typ,
				value,
				padding: self.padding,
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
				    // self.add( TokType::Comment, TokValue::String { value: string.clone() } );
			    }
				'\t' => {
					let mut count = 1usize;
					while self.get_char_o( 1 ) == '\t' {
						count += 1;
						self.index += 1;
					}
					self.add( TokType::Padding, TokValue::Int { value: count as i64 } );
					self.padding = count;
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
					self.padding = 0;
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

					self.add( TokType::Word, TokValue::String { value: string.trim_end().to_string() } );
					self.index += 1;
					self.char += string.len() + 1;
				},
			}
		}
		self.tokens.push( Token {
			typ: TokType::EOF,
			value: TokValue::None,
			padding: 0,
			loc: Loc {
				file: self.file.clone(),
				line: self.line + 1,
				char: 0
			}
		} );

		self.tokens.clone()
	}
}

struct Lexer {
	tok_list: Vec<Token>,
	index: usize
}

impl Lexer {
	fn new( tok_list: Vec<Token> ) -> Self {
		Lexer {
			tok_list: tok_list,
			index: 0
		}
	}

	fn peek(  &mut self, offset: usize ) -> &Token {
		if self.tok_list.len() > self.index + offset {
			&self.tok_list[ self.index + offset ]
		} else {
			self.tok_list.last().unwrap()
		}
	}

	fn consume( &mut self ) -> Token {
		self.index += 1;
		self.tok_list[ self.index - 1 ].clone()
	}

	fn peekType( &mut self ) -> &TokType {
		&self.peek(1).typ
	}

	fn peekIsType( &mut self, offset: usize, typ: TokType ) -> bool {
		&self.peek( offset ).typ == &typ
	}

	fn parse( &mut self ) -> Vec<Token> {
		let mut processed_tokens = vec![];

		while self.tok_list.len() > self.index {
			if self.peekIsType( 0, TokType::Word ) && self.peekIsType( 1, TokType::Semicolon ) {
				// key
				processed_tokens.push( Token::new( TokType::Key, self.consume() ) );
			} else if (
					self.peekIsType( 0, TokType::Semicolon ) ||
					self.peekIsType( 0, TokType::Padding )
				) &&
				self.peekIsType( 1, TokType::Word ) &&
				!self.peekIsType( 2, TokType::Semicolon )
			{
				// value
				self.consume();
				processed_tokens.push( Token::new( TokType::Value, self.consume() ) );
			} else if self.peekIsType( 0, TokType::Dot ) && self.peekIsType( 1, TokType::Word ) && self.peekIsType( 2, TokType::Semicolon ) {
				// class
				self.consume();
				let value = self.consume();
				self.consume();
				processed_tokens.push( Token::new( TokType::Class, value ) );
			} else if vec![ TokType::Newline, TokType::EOF ].contains( self.peekType() ) {
				// newline/eof
				let tokk = self.consume();
				processed_tokens.push( tokk );
			} else {
				// anything else
				self.consume();
			}
		}

		processed_tokens
	}
}

struct Parser {
	tokens: Vec<Token>,
	index: usize
}

impl Parser {
	fn new( tokens: Vec<Token> ) -> Self {
		Parser { tokens: tokens, index: 0 }
	}

	fn peek( &self, offset: usize ) -> &Token {
		&self.tokens[ self.index + offset ]
	}

	fn consume( &mut self ) -> Token {
		self.index += 1;
		self.tokens[ self.index - 1 ].clone()
	}

	fn consumeIfIs( &mut self, typ: TokType ) {
		if self.peek(0).typ == typ {
			self.consume();
		}
	}

	fn key( &mut self ) -> Option<String> {
		if self.peek(0).typ != TokType::Key {
			return None;
		}
		Some( self.consume().stringValue() )
	}

	fn class( &mut self ) -> Option<String> {
		if self.peek(0).typ != TokType::Class {
			return None;
		}
		Some( self.consume().stringValue() )
	}

	fn value( &mut self ) -> E {
		if self.peek(0).typ == TokType::Class {
			return self.object()
		}
		match self.consume().value {
			TokValue::Int { value } => E::Integer { val: value.clone() },
			TokValue::Float { value } => E::Float { val: value.clone() },
			TokValue::String { value } => E::String { val: value.clone() },
			TokValue::None => E::None
		}
	}

	fn keyValue( &mut self ) -> KeyValue {
		if let Some( key ) = self.key() {
			// there's a key
			if self.peek( 0 ).typ == TokType::Semicolon {
				self.consume(); // remove the semicolon

				// its either a map or a list
				if vec![ TokType::Value, TokType::Class ].contains( &self.peek(0).typ ) {
					// its a list
					return KeyValue { key: key, value: self.list() }
				}

				// its a map
				return KeyValue { key: key, value: self.map() }

			} else if self.peek( 0 ).typ == TokType::Value {
				// its a key-value pair
				return KeyValue { key: key, value: self.value() }
			} else if self.peek( 0 ).typ == TokType::Class {
				// its a key-value of an object
				return KeyValue { key: key, value: self.object() }
			}
		}
		panic!( "What the fuck just happened?? \n\t- {:?}\n\t- {:?}", self.peek(0), self.peek(1) )
	}

	fn object( &mut self ) -> E {
		// .ClassName:
		//      field: value

		let class = self.class().unwrap();

		let mut fields = vec![];
		while self.index < self.tokens.len() {
			if self.peek(0).typ != TokType::Key {
				break
			}
			fields.push( self.keyValue() );
		}

		E::Object { class: class, fields: fields }
	}

	fn list( &mut self ) -> E {
		let mut items = vec![];
		while self.index < self.tokens.len() {
			if !vec![ TokType::Value, TokType::Class ].contains( &self.peek(0).typ ) {
				break
			}
			items.push( self.value() );
		}
		E::List { values: items }
	}

	fn map( &mut self ) -> E {
		let mut items = vec![];
		self.consumeIfIs(TokType::Semicolon); // remove the newline

		let firstKey = self.peek(0).clone();
		while self.index < self.tokens.len() {
			if self.peek(0).typ != TokType::Key || self.peek(0).padding != firstKey.padding {
				break
			}
			items.push( self.keyValue() );
		}
		E::Map { values: items }
	}

	fn objectify( &mut self ) -> E {
		if self.peek(0).typ == TokType::Word {
			// root is list
			return self.list()
		}
		// root is map
		self.map()
	}
}

pub fn tokenize( string: &str, file: &str ) -> Vec<Token> {
	Tokenizer::new(string, file).tokenize()
}

pub fn lex(tok_list: Vec<Token> ) -> Vec<Token> {
	Lexer::new(tok_list).parse()
}

pub fn parse(tok_list: Vec<Token> ) -> E {
	Parser::new(tok_list).objectify()
}

pub fn load( path: &Path ) -> E {
	parse(
		lex(
			tokenize(
				read_to_string( path ).unwrap().as_str(),
				path.to_str().unwrap()
			)
		)
	)
}

pub fn loads( data: &str, file: &str ) -> E {
	parse( lex( tokenize(data, file ) ) )
}

pub(crate) fn main() {
	let tokens = tokenize( read_to_string( Path::new("test.e") ).unwrap().as_str(), "test.e" );
	println!( "Tokens: {:?}", tokens );

	let parsed_tokens = lex( tokens );
	println!( "Tokens: {:?}", parsed_tokens );

	let object = parse( parsed_tokens );
	println!( "Object: {}", object )
}
