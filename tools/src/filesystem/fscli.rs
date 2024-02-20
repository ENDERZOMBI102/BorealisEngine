#![feature(slice_pattern)]
#![allow(non_snake_case)]
#![feature(string_remove_matches)]
#![feature(fn_traits)]

extern crate core;

use std::io::Write;

use filesystem::layered::LayeredFS;

use crate::shell::getCommands;

mod shell;

pub fn main() {
	let commands = getCommands();

	let mut fs = LayeredFS::new();
	// FIXME: This errors
	// fs.add_layer( std::env::current_dir().unwrap(), false ).expect("Failed to add current dir as layer.");

	println!( "FileSystem shell v1.4" );

	let mut input = String::new();
	let mut currentDir = "/".to_string();

	'outer: loop {
		// first thing we do is cleaning, as we `break` to here when a command is executed
		input.clear();
		print!( ">>> " );
		std::io::stdout().flush().expect("Failed to flush STDOUT");
		match std::io::stdin().read_line( &mut input ) {
			Ok(_n) => {
				input.clear().extend( input.trim() );
				let command: Vec<&str> = input.split(" ").collect();

				match command.as_slice() {
					[ "help", "help" ] => { // help w/help as parameter command
						println!("help: Prints this message")
					}
					[ "help", cmd ] => { // help w/parameter command
						match commands.get(cmd) {
							Some((_, help)) => println!("{help}"),
							None => eprintln!("help: Unknown command {cmd}")
						}
					}
					[ "help" ] => { // help command
						println!("Available COMMANDS:");
						for ( _, ( _, help ) ) in commands.iter() {
							println!(" - {help}")
						}
					},
					_ => None.unwrap()
				}

				// FIXME: This errors
				// handle command
				for hdlr in commands {
					if hdlr.name == command[0] {
						// registered command to execute
						hdlr.handler( ( &mut fs, &command, &mut currentDir ) );
						break 'outer
					}
				}
				// unknown command
				eprintln!( "ERROR: Unknown command {}", command[0] )
			}
			Err(error) => eprintln!( "ERROR: {error}" )
		}
	}
}
