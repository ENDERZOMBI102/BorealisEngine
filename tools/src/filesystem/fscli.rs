#![feature(string_remove_matches)]

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
					"has" => {
						if command.len() != 2 {
							eprintln!("usage: has $PATH");
							continue
						}
						println!("{}", fs.contains( command[1] ) )
					},
					"read" => {
						if let Ok( file ) = fs.get_file( command[1] ) {
							match file.read_string() {
								Ok( data ) => println!( "{}", data ),
								Err( err ) => println!( "read: Failed to read file \"{}\": {}", command[1], err )
							}
						} else {
							println!( "File {} was not found", command[1] )
						}
					},
					"find" => {
						if command.len() != 2 {
							eprintln!("usage: find $PATH");
							continue
						}
						match fs.resolve( command[1] ) {
							None => eprintln!( "find: cannot find path \"{}\"", command[1] ),
							Some( path ) => println!( "{}", path.to_str().unwrap() )
						}
					},
					"reverse" => {
						fs.layers.reverse();
						println!("reversed layers order")
					},
					"addLayer" => {
						if command.len() == 1 {
							eprintln!("usage: addLayer [--pre] $PATH");
							continue
						}
						let prepend = command.contains( &"--pre" );
						let path = Path::new( command.last().unwrap() ).canonicalize().unwrap();
						let mut success_message = String::new();
						success_message += if prepend { "prepended " } else { "appended " };
						success_message += path.to_str().unwrap();
						success_message += " as new layer";

						if path.is_dir() {
							// folder
							fs.add_layer( Box::new( FolderLayer::from_buf( path.clone() ) ), prepend );
							println!( "{}", success_message )
						} else {
							match path.extension() {
								None => println!( "ERROR: If adding a file, please make sure it has a valid extension." ),
								Some( ext ) => {
									match ext.to_str().unwrap() {
										// upkf file
										"upkf" => {
											fs.add_layer(
												Box::new( UpkfLayer::from_buf( path.canonicalize().unwrap() ) ),
												prepend
											);
											println!( "{}", success_message )
										},
										// vpk file
										"vpk" => {
											fs.add_layer(
												Box::new( VpkLayer::from_buf( path.canonicalize().unwrap() ) ),
												prepend
											);
											println!( "{}", success_message )
										},
										ext => println!( "ERROR: Unsupported file type: {}", ext )
									}
								}
							}
						}
					},
					"listLayers" => {
						for layer in &fs.layers {
							println!(
								"Layer in pos {}: {}",
								fs.layers.iter().enumerate()
									.find( |&i| i.1.meta().filename == layer.meta().filename )
									.unwrap()
									.0,
								layer.meta().filename
							)
						}
					},
					"listFiles" => println!( "TODO: FINISH THIS" ),
					"parse" => {
						// parse [ --help | [ --detect | --tokenize | --lex | --ugly ] file ]
						if command.len() == 1 {
							eprintln!( "usage: parse [ --help | [ --detect | --tokenize | --lex | --ugly | ] file ]" );
						} else if command.contains(&"--help") {
							println!( "Parsing command line utility v1" );
							println!( "usage: parse [ --help | [ --detect | --tokenize | --lex | --ugly | ] file ]" );
							println!( "\t--help            prints this message" );
							println!( "\t--detect   $PATH  prints the detected format of a file" );
							println!( "\t--tokenize $PATH  tokenize a file and print the stream of tokens" );
							println!( "\t--lex      $PATH  tokenize and lex a file and print the stream of tokens" );
							println!( "\t--ugly     $PATH  parses a file and print the resulting object in a single line" );
							println!( "\t           $PATH  parses a file and pretty print the resulting object" );
						} else {
							let file = command.last().unwrap();
							let ext: &str = file.split(".").collect::<Vec<&str>>().last().unwrap();

							// load the file data as an UTF-8 string
							let data = match fs.get_file( file ) {
								Err( kind ) => {
									eprintln!( "parse: failed to load file \"{}\": {}", file, kind );
									continue
								}
								Ok( layeredFile ) => match layeredFile.read_string() {
									Err( kind ) => {
										eprintln!("parse: failed to read file \"{}\": {}", file, kind );
										continue
									}
									Ok( string ) => string
								}
							};

							if command.contains(&"--tokenize") {
								match ext {
									"kv" | "vdf" => println!( "{:?}", kv::tokenize( data.as_str() ) ),
									"e" => println!( "{:?}", e::tokenize( data.as_str(), file ) ),
									_ => eprintln!( "parse: Cannot tokenize file: Unknown file type" )
								}
							} else if command.contains(&"--lex") {
								match ext {
									"kv" | "vdf" => println!( "{:?}", kv::parse( kv::tokenize( data.as_str() ) ) ),
									"e" => println!( "{:?}", e::parse( e::tokenize( data.as_str(), file ) ) ),
									_ => eprintln!( "parse: Cannot lex file: Unknown file type" )
								}
							} else if command.contains(&"--detect") {
								println!(
									"Detected format: {}",
									match ext {
										"kv" | "vdf" => "Valve's KeyValues 1",
										"e" => "EZ102's E format",
										_ => "Unknown"
									}
								)
							} else if command.contains(&"--ugly") {
								match ext {
									"kv" | "vdf" => println!( "{}", "" ),
									"e" => println!( "{}", e::loads( data.as_str(), file ) ),
									_ => eprintln!( "parse: Cannot parse file: Unknown file type" )
								}
							} else {
								match ext {
									"kv" | "vdf" => println!( "{}", "" ),
									"e" => println!( "{}", e::loads( data.as_str(), file ) ),
									_ => eprintln!( "parse: Cannot parse file: Unknown file type" )
								}
							}
						}
					}
					"clear" => print!("\x1B[2J")
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
}
