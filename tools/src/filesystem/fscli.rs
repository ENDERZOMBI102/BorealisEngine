#![feature(string_remove_matches)]
#![feature(slice_pattern)]
#![allow(non_snake_case)]
#![feature(type_alias_impl_trait)]

use core::slice::SlicePattern;
use std::collections::HashMap;
use std::fmt::Display;
use std::io::Write;
use std::path::Path;
use filesystem::layered::LayeredFS;
use filesystem::layered::layers::{FolderLayer, UpkfLayer, VpkLayer};
use tier0::format::{e, kv};

pub fn main() {
	let mut fs = LayeredFS {
		layers: vec![
			Box::new( FolderLayer::from_buf( std::env::current_dir().unwrap() ) )
		]
	};

	println!( "FileSystem CLI v1" );

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
						[ "has", path ] => println!("{}", fs.contains( path ) ),
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
					"reverse" => {
						fs.layers.reverse();
						println!("reversed layers order")
					},
					"addLayer" => match command.as_slice() {
						[ "addLayer" ] | [ "addLayer", "--pre" ] => eprintln!("usage: addLayer [--pre] $PATH"),
						[ "addLayer", "--pre", path ] => addLayerHandler( &mut fs, true, path ),
						[ "addLayer", path ] => addLayerHandler( &mut fs, false, path ),
						_ => eprintln!("usage: addLayer [--pre] $PATH")
					},
					"listLayers" => for layer in &fs.layers {
						// println!( "Layer in pos {}: {}", fs.layers.as_slice().index_of( layer ), layer.meta().filename )
					},
					"listFiles" => eprintln!( "TODO: FINISH THIS" ),
					"parse" => parseHandler( &mut fs, command[ 1 .. command.len() - 1 ].as_slice() ),
					"clear" => print!("\x1B[2J"),
					"help" => {
						println!( "Available commands:" );
						println!( " - has $PATH: Prints true if the file exists, false otherwise" );
						println!( " - read $PATH: Prints the contents of the file, if found" );
						println!( " - find $PATH: Prints the full path to the file, if found" );
						println!( " - addlayer $PREPEND $PATH: Adds a layer to the fs, may be a path to a folder or .upkf/.vpk file" );
						println!( " - listLayers: Lists all layers in this LFS" );
						println!( " - listFiles: Lists all files present in all layers" );
						println!( " - reverse: Reverses the order of the layers" );
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

	fn addLayerHandler( fs: &mut LayeredFS, prepend: bool, rawPath: &str ) {
		let path = Path::new( rawPath ).canonicalize().unwrap();
		let success_message = format!( "{} {} as new layer", if prepend { "prepended" } else { "appended" }, path.display() );

		if path.is_dir() {
			// folder
			fs.add_layer( Box::new( FolderLayer::from_buf( path.clone() ) ), prepend );
			println!( "{}", success_message )
		} else {
			match path.extension() {
				None => eprintln!( "ERROR: If adding a file, please make sure it has a valid extension." ),
				Some( ext ) => {
					match ext.to_str().unwrap() {
						// upkf file
						"upkf" => fs.add_layer( Box::new( UpkfLayer::from_buf( path.canonicalize().unwrap() ) ), prepend ),
						// vpk file
						"vpk" => fs.add_layer( Box::new( VpkLayer::from_buf( path.canonicalize().unwrap() ) ), prepend ),
						ext => {
							println!( "ERROR: Unsupported file type: {}", ext );
							return;
						}
					}
					println!( "{}", success_message )
				}
			}
		}
	}

	fn parseHandler( fs: &mut LayeredFS, args: &[&str]) {
		// parse [ --help | [ --detect | --tokenize | --lex | --ugly ] file ]
		type Handler<'a> = &'a impl Fn(&str, &str) -> dyn Display;

		let process = | path, stage, handlers: HashMap<&str, Handler> | match fs.get_file(path) {
			Err(kind) => eprintln!("parse: failed to load file \"{}\": {}", path, kind),
			Ok(layeredFile) => match layeredFile.read_string() {
				Err(kind) => eprintln!("parse: failed to read file \"{}\": {}", path, kind),
				Ok(contents) => {
					let ext =  Path::new(path).extension().unwrap_or("".as_ref() ).to_str().unwrap();
					match handlers.get( ext ) {
						Some( handler ) => println!( "{:?}", handler( contents.as_str(), path ).to_string() ),
						None => eprintln!( "parse: Cannot {stage} file: Unknown file type '{ext}'" )
					}
				}
			}
		};

		let mut map = HashMap::<&str, Handler>::new();
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
				map.insert( "e", &| path, data | e::tokenize( data, path ) );
				process( path, "tokenize", map )
			},
			[ "--lex", path ] => {
				map.insert( "e", &| path, data | e::lex(e::tokenize( data, path ) ) );
				process( path, "lex", map )
			},
			[ "--ugly", path ] | [ path ] => {
				map.insert( "e", &| path, data | e::loads( data, path ) );
				process( path, "parse", map )
			},
			_ => eprintln!( "usage: parse [ --help | [ --detect | --tokenize | --lex | --ugly ] file ]" ),
		}
	}
}
