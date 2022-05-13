use std::env;
use std::os::windows::fs::MetadataExt;
use std::path::Path;
use std::process::exit;
use filesystem::upkf::Upkf;

pub fn main() {
	let argv: Vec<String> = env::args().collect();
	let file_to_decompress = Path::new( argv.get(1).unwrap() );
	// let file_to_decompress = Path::new("filesystem.upkf");
	let no_data_text: String = String::from("N/D");

	if !file_to_decompress.exists() {
		eprintln!("File {} doesn't exist!", file_to_decompress.display() );
		exit(1)
	}

	println!( "Loading {}", file_to_decompress.display() );
	let upkf = Upkf::load( file_to_decompress, true ).unwrap();
	println!( "Origin: {}", upkf.get_origin() );
	println!( "File Size: {}", file_to_decompress.metadata().unwrap().file_size() );
	println!( "Entry Count: {}", upkf.count() );
	println!( "Entries:" );
	for entry in upkf.iter() {
		let mut meta = entry.get_meta();
		if meta.is_empty() {
			meta = &no_data_text;
		}
		let crc32 = entry.get_crc32();
		let sha256 = entry.get_sha256();
		let mut crc32d = no_data_text.clone();
		let mut sha256d = no_data_text.clone();
		if crc32.is_some() && sha256.is_some() {
			crc32d = crc32.unwrap().to_string();
			sha256d = sha256.clone().unwrap();
		}
		println!( " - File Path: {}", entry.get_path() );
		println!( "   - Meta: {}", meta );
		println!( "   - Size: {}", entry.get_content().len() );
		println!( "   - Crc32: {}", crc32d );
		println!( "   - Sha256: {}", sha256d );
		println!( "   - Binary: {}", entry.is_bynary() );
		println!( "   - Compression: {}", entry.get_compression() );
	}
	println!( "\nFinished" )
}