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

				match command.as_slice() {
					[ "help", "help" ] => { // help w/parameter command
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

				// handle command
				match commands.get( command[0] ) {
					// registered command to execute
					Some( ( handler, _ ) ) => handler.call( ( &mut fs, command, &mut currentDir ) ),
					// unknown command
					None => eprintln!( "ERROR: Unknown command {}", command[0] )
				}
			}
			Err(error) => eprintln!( "ERROR: {error}" )
		}
		input.clear()
	}
}
