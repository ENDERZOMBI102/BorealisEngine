use std::path::Path;
use std::process::exit;
use crate::upkf::Upkf;

pub fn main() {
	// let argv: Vec<String> = env::args().collect();
	// let file_to_decompress = Path::new( argv.get(1).unwrap() );
	let file_to_decompress = Path::new("filesystem.upkf");

	if !file_to_decompress.exists() {
		eprintln!("File {} doesn't exist!", file_to_decompress.display() );
		exit(1)
	}

	println!( "Loading {}", file_to_decompress.display() );
	let upkf = Upkf::load( file_to_decompress );
	println!( "Origin: {}", upkf.get_origin() );
	println!( "Entries:" );
	for entry in upkf.iter() {
		println!( " - file: {}", entry.get_path() );
		println!( "   - meta: {}", entry.get_meta() );
		println!( "   - size: {}", entry.get_content().len() );
		println!( "   - binary: {}", entry.is_bynary() );
		println!( "   - compression: {}", entry.get_compression() );
	}
	println!( "\nFinished" )
}