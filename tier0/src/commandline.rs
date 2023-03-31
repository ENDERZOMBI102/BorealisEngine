use std;
use std::sync::LazyLock;

pub struct CommandLine {
	exec: String,
	argv: Vec<String>
}

impl CommandLine {
	pub fn get() -> &'static CommandLine {
		static CLI: LazyLock<CommandLine> = LazyLock::new(|| {
			let mut args: Vec<String> = std::env::args().collect();
			let argv = args.split_off(1);
			CommandLine { exec: args.remove(0), argv }
		});
		&CLI
	}

	pub fn flag( &mut self, flag: &'static str ) -> bool {
		self.argv.iter()
			.any( |item| item == flag )
	}

	pub fn option( &self, flag: &str ) -> Option<&String> {
		self.argv.iter()
			.enumerate()
			.find( |(_, arg)| arg.as_str() == flag )
			.map( |(index, _)| &self.argv[ index + 1 ] )
	}

	pub fn option_many( &self, flag: &str ) -> Vec<&String> {
		let mut res = Vec::new();
		let mut index = 0;

		while index < self.argv.len() {
			if self.argv[index] == flag {
				res.push( &self.argv[ index + 1 ] );
			} else {
				index += 1;
			}
		}
		res
	}

	pub fn all( &self ) -> &Vec<String> {
		&self.argv
	}
}

#[cfg(test)]
mod testing {
	use super::*;

	#[test]
	pub fn testing() {
		println!( "Executable:\n\t- \"{}\"", CommandLine::get().exec );
		println!( "Arguments:" );
		for arg in &CommandLine::get().argv {
			println!( "\t- \"{}\"", arg );
		}
	}
}
