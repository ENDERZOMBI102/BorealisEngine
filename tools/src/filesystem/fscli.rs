#![feature(string_remove_matches)]
#![feature(slice_pattern)]
#![allow(non_snake_case)]
#![feature(type_alias_impl_trait)]
#![feature(fn_traits)]

use core::slice::SlicePattern;
use std::collections::HashMap;
use std::fmt::{Debug, Display};
use std::io::{ErrorKind, Write};
use std::path::Path;

use filesystem::layered::{LayeredFile, LayeredFS};
use filesystem::layered::layers::{FolderLayer, UpkfLayer, VpkLayer};
use tier0::format::e;

pub fn main() {
	let mut fs = LayeredFS {
		layers: vec![
			Box::new( FolderLayer::from_buf( std::env::current_dir().unwrap() ) )
		]
	};

	println!( "FileSystem CLI v1.2" );

	let mut input = String::new();
	loop {
		print!( ">>> " );
		std::io::stdout().flush().expect("Failed to flush STDOUT");
		match std::io::stdin().read_line( &mut input ) {
			Ok(_n) => {
				input.remove_matches("\n");
				input.remove_matches("\r");
				let command: Vec<&str> = input.split(" ").collect();
				match command[0] {
					"has" => match command.as_slice() {
						[ "has", path ] => {
							match fs.get_file() {
								Ok( file ) => println!( "File `{}` found in layer {}", file ),
								Err(_) => {}
							}
						},
						_ => eprintln!("usage: has $PATH")
					},
					"read" => match command.as_slice() {
						[ "read", path ] => match fs.get_file(path) {
							Ok( file ) => match file.read_string() {
								Ok( data ) => println!( "{}", data ),
								Err( err ) => println!( "read: Failed to read file \"{}\": {}", path, err )
							},
							_ => eprintln!( "File {path} was not found" )
						},
						_ => eprintln!( "usage: read $PATH" )
					},
					"find" => match command.as_slice() {
						[ "find", path ] => match fs.resolve( path ) {
							None => eprintln!( "find: cannot find path \"{}\"", path ),
							Some( path ) => println!( "{}", path.to_str().unwrap() )
						},
						_ => eprintln!("usage: find $PATH")
					},
					"layer" => layerHandler( &mut fs, command ),
					"listFiles" => eprintln!( "TODO: FINISH THIS" ),
					"parse" => parseHandler( &mut fs, command ),
					"clear" => print!("\x1B[2J"),
					"help" => {
						println!( "Available commands:" );
						println!( " - has $PATH: Prints true if the file exists, false otherwise" );
						println!( " - read $PATH: Prints the contents of the file, if found" );
						println!( " - find $PATH: Prints the full path to the file, if found" );
						println!( " - layer $SUBCOMMAND [$ARGUMENTS]: Manages layers" );
						println!( " - listFiles: Lists all files present in all layers" );
						println!( " - parse [$OPTIONS] $PATH: Parses a file of a supported format, use `parse --help` for more info" );
						println!( " - clear: Clears the terminal" );
						println!( " - help: Prints this message" );
					},
					cmd => println!( "ERROR: Unknown command {}", cmd )
				}
			}
			Err(error) => println!("ERROR: {}", error)
		}
		input.clear()
	}

	fn layerHandler( fs: &mut LayeredFS, mut args: Vec<&str> ) {
		args.remove(0); // remove "layer" prefix
		match args.as_slice() {
			[ "help" ] => {
				println!( "Layer manager v1.1" );
				println!( "usage: layer ( help | list | reverse | ( append | prepend ) $PATH )" );
				println!( "\thelp              prints this message" );
				println!( "\tappend     $PATH  Adds a layer to the end of the fs, may be a path to a folder or .upkf/.vpk file" );
				println!( "\tprepend    $PATH  Adds a layer to the start of the fs, may be a path to a folder or .upkf/.vpk file" );
				println!( "\tlist              list all available layers" );
				println!( "\treverse           reverse the order of the layer list" );
			}
			// [ "list" ] => println!( "Layer in pos {}: {}", fs.layers.as_slice().index_of( layer ), layer.meta().filename ),
			[ "reverse" ] => {
				fs.layers.reverse();
				println!("Layer list successfully reversed.")
			}
			[ action @ ( "append" | "prepend" ), rawPath ] => {
				let path = Path::new(rawPath).canonicalize().unwrap();

				if path.is_dir() { // folder
					fs.add_layer( Box::new( FolderLayer::from_buf( path.clone() ) ), action == &"prepend" );
				} else { // file
					match path.extension() {
						None => eprintln!("ERROR: If adding a file, please make sure it has a valid extension."),
						Some( ext ) => {
							match ext.to_str().unwrap() {
								// upkf file
								"upkf" => fs.add_layer( Box::new( UpkfLayer::from_buf( path.clone() ) ), action == &"prepend" ),
								// vpk file
								"vpk" => fs.add_layer( Box::new( VpkLayer::from_buf( path.clone() ) ), action == &"prepend" ),
								ext => {
									eprintln!( "ERROR: Unsupported file type: {ext}" );
									return;
								}
							}
						}
					}
				}
				println!( "{action}ded {:?} as new layer", path )
			}
			_ => eprintln!( "usage: layer ( help | list | reverse | ( append | prepend ) $PATH )" )
		}
	}

	fn parseHandler( fs: &mut LayeredFS, mut args: Vec<&str> ) {
		type Handler<'a> = &'a dyn Fn(&str, &str) -> Box<dyn Debug>;
		let mut handlers = HashMap::<&str, Handler>::new();

		let mut process = |path: &str, stage: &str, handlers: HashMap<&str, Handler>| match fs.get_file(path) {
			Err(kind) => eprintln!("parse: failed to load file \"{}\": {}", path, kind),
			Ok(layeredFile) => match layeredFile.read_string() {
				Err(kind) => eprintln!("parse: failed to read file \"{}\": {}", path, kind),
				Ok(contents) => {
					let ext = Path::new(path).extension().unwrap_or("".as_ref()).to_str().unwrap();
					match handlers.get(ext) {
						None => eprintln!("parse: Cannot {stage} file: Unknown file type '{ext}'"),
						Some(handler) => println!("{:?}", (handler.call((contents.as_str(), path)) as Box<dyn Debug>))
					}
				}
			}
		};

		args.remove(0); // remove "parse" prefix
		match args.as_slice() {
			[ "--help" ] => {
				println!( "Parsing command line utility v1" );
				println!( "usage: parse [ --help | [ --detect | --tokenize | --lex | --ugly ] file ]" );
				println!( "\t--help            prints this message" );
				println!( "\t--detect   $PATH  prints the detected format of a file" );
				println!( "\t--tokenize $PATH  tokenize a file and print the stream of tokens" );
				println!( "\t--lex      $PATH  tokenize and lex a file and print the stream of tokens" );
				println!( "\t--ugly     $PATH  parses a file and print the resulting object in a single line" );
				println!( "\t           $PATH  parses a file and pretty print the resulting object" );
			},
			[ "--detect", path ] => println!(
				"Detected format: {}",
				match Path::new(path).extension().unwrap_or("".as_ref()).to_str().unwrap() {
					"kv" | "vdf" => "Valve's KeyValues 1",
					"kv2" => "Valve's KeyValues 2",
					"kv3" => "Valve's KeyValues 3",
					"e" => "EZ102's E format",
					_ => "Unknown"
				}
			),
			[ "--tokenize", path ] => {
				handlers.insert("e", &|path, data | Box::new( e::tokenize(data, path ) ) );
				process( path, "tokenize", handlers )
			},
			[ "--lex", path ] => {
				handlers.insert("e", &|path, data | Box::new( e::lex(e::tokenize(data, path ) ) ) );
				process( path, "lex", handlers )
			},
			[ "--ugly", path ] | [ path ] => {
				handlers.insert("e", &|path, data | Box::new( e::loads(data, path ) ) );
				process( path, "parse", handlers )
			},
			_ => eprintln!( "usage: parse [ --help | [ --detect | --tokenize | --lex | --ugly ] file ]" ),
		}
	}
}
