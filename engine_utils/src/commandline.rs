use std;

enum ArgType {
	Arg, Value
}

struct RawArg {
	arg_type: ArgType,
	value: String
}

impl RawArg {
	pub(crate) fn is_arg(&self) -> bool {
		return matches!(self.arg_type, ArgType::Arg)
	}
	pub(crate) fn is_value(&self) -> bool {
		return !self.is_arg()
	}
}

pub struct Argument {
	key: String,
	value: Option<String>
}


pub fn main() {
	let args = std::env::args();
	let mut exec: Option<String> = None;
	let mut raw_arguments: Vec<RawArg> = Vec::new();
	let mut arguments: Vec<Argument> = Vec::new();
	let mut i = 0;
	for arg in args {
		if exec.is_none() {
			exec = Some( arg.clone() )
		} else {
			if arg.starts_with("+") || arg.starts_with("-") {
				raw_arguments.push( RawArg { arg_type: ArgType::Arg, value: arg } );
			} else {
				raw_arguments.push( RawArg { arg_type: ArgType::Value, value: arg } )
			}
		}
	}
	while i < raw_arguments.len() {
		if raw_arguments.len() > i + 1 && raw_arguments[i + 1].is_value() {
			arguments.push(
				Argument {
					key: raw_arguments[i].value.clone(),
					value: Some( raw_arguments[i + 1].value.clone() )
				}
			);
			i += 2;
		} else {
			arguments.push(
				Argument {
					key: raw_arguments[i].value.clone(),
					value: None
				}
			);
			i += 1;
		}
	}

	println!( "Executable:\n\t- \"{}\"", exec.unwrap() );
	println!( "Arguments:" );
	for arg in arguments {
		println!( "\t- \"{}\"", arg.key );
		if arg.value.is_some() {
			println!( "\t\t- \"{}\"", arg.value.unwrap() )
		}
	}
}