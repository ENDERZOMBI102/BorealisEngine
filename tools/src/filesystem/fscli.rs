#![feature(slice_pattern)]
#![allow(non_snake_case)]
#![feature(string_remove_matches)]
#![feature(fn_traits)]

use core::slice::SlicePattern;
use std::fmt::{Debug, Display};
use std::io::Write;

use filesystem::layered::LayeredFS;
use crate::shell::getCommands;

mod shell;

pub fn main() {
	let mut commands = getCommands();
	commands.insert(
		"clear",
		(
			| fs: &mut LayeredFS, mut args: Vec<&str>, currentDir: &mut String | print!("\x1B[2J"),
			"clear: Clears the terminal"
		)
	);

	let helpHandler = | fs: &mut LayeredFS, mut args: Vec<&str>, currentDir: &mut String | {
		match args.as_slice() {
			[ "help", com ] => { // help w/parameter command
				match commands.get(com) {
					Some((_, help)) => println!("{help}"),
					None => eprintln!("help: Unknown command {cmd}")
				}
			}
			[ "help" ] => { // help command
				println!("Available commands:");
				for (_, (_, help)) in &commands {
					println!(" - {help}")
				}
			},
		}
	};
	commands.insert( "help",  ( helpHandler , "help: Prints this message" ) );

	let mut fs = LayeredFS::new();
	fs.add_layer( std::env::current_dir().unwrap(), false ).expect("Failed to add current dir as layer.");

	println!( "FileSystem shell v1.4" );

	let mut input = String::new();
	let mut currentDir = "/".to_string();

	loop {
		print!( ">>> " );
		std::io::stdout().flush().expect("Failed to flush STDOUT");
		match std::io::stdin().read_line( &mut input ) {
			Ok(_n) => {
				input.remove_matches("\n");
				input.remove_matches("\r");
				let command: Vec<&str> = input.split(" ").collect();

				// handle command
				match commands.get( command[0] ) {
					// registered command to execute
					Some( ( command, _ ) ) => command.call( ( &mut fs, command, &mut currentDir ) ),
					// unknown command
					None => eprintln!( "ERROR: Unknown command {}", commands[0] )
				}
			}
			Err(error) => eprintln!( "ERROR: {error}" )
		}
		input.clear()
	}
}
