use std::collections::HashMap;
use std::fmt::Debug;
use std::path::Path;

use filesystem::layered::LayeredFS;
use tier0::format::e;

pub(crate) fn parseHandler( fs: &mut LayeredFS, mut args: Vec<&str>, currentDir: &mut String ) {
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
