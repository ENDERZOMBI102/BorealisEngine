use std::cell::OnceCell;
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

pub struct Command {
	pub name: &'static str,
	pub handler: fn(fs: &mut LayeredFS<'static>, argv: Vec<&str>, cwd: &mut String),
	pub help: &'static str
}

static COMMANDS: OnceCell<Vec<Command>> = OnceCell::new();

pub fn getCommands() -> &'static Vec<Command> {
	COMMANDS.get_or_init( || {
		vec![
			Command { name: "ls", handler: lsHandler, help: "ls [$PATH]: Lists all files present in the current from all layers" },
			Command { name: "cd", handler: cdHandler, help: "cd [$PATH]: Change current directory to the provided path, or tell the current one" },
			Command { name: "has", handler: hasHandler, help: "has $PATH: Prints true if the file exists, false otherwise" },
			Command { name: "read", handler: readHandler, help: "read $PATH: Prints the contents of the file, if found" },
			Command { name: "find", handler: findHandler, help: "find $PATH: Prints the full path to the file, if found" },
			Command { name: "layer", handler: layerHandler, help: "layer $SUBCOMMAND [$ARGUMENTS]: Manages layers" },
			Command { name: "parse", handler: parseHandler, help: "parse [$OPTIONS] $PATH: Parses a file of a supported format, use `parse --help` for more info" },
			Command { name: "clear", handler: clearHandler, help: "clear: Clears the terminal" },
		]
	})
}
