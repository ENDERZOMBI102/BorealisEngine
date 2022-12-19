use std::collections::HashMap;
use filesystem::layered::LayeredFS;
use crate::shell::builtin::clearHandler;

use crate::shell::layer::layerHandler;
use crate::shell::parse::parseHandler;
use crate::shell::path::{cdHandler, findHandler, hasHandler, lsHandler};
use crate::shell::read::readHandler;

mod path;
mod layer;
mod parse;
mod read;
mod builtin;


pub(crate) fn getCommands() -> HashMap<&'static str, (fn(&mut LayeredFS<'static>, Vec<&str>, &mut String), &'static str)> {
	let mut commands: HashMap<&str, (fn(&mut LayeredFS<'static>, Vec<&str>, &mut String), &str)> = HashMap::new();
	commands.insert( "ls",    ( lsHandler   , "ls [$PATH]: Lists all files present in the current from all layers" ) );
	commands.insert( "cd",    ( cdHandler   , "cd [$PATH]: Change current directory to the provided path, or tell the current one" ) );
	commands.insert( "has",   ( hasHandler  , "has $PATH: Prints true if the file exists, false otherwise" ) );
	commands.insert( "read",  ( readHandler , "read $PATH: Prints the contents of the file, if found" ) );
	commands.insert( "find",  ( findHandler , "find $PATH: Prints the full path to the file, if found" ) );
	commands.insert( "layer", ( layerHandler, "layer $SUBCOMMAND [$ARGUMENTS]: Manages layers" ) );
	commands.insert( "parse", ( parseHandler, "parse [$OPTIONS] $PATH: Parses a file of a supported format, use `parse --help` for more info" ) );
	commands.insert( "clear", ( clearHandler, "clear: Clears the terminal" ) );

	commands
}

