use std::env;
use std::fs::File;
use std::io::Read;
use std::path::Path;
use std::process::exit;
use bytes::Bytes;
use crate::upkf::Upkf;

pub fn main() {
	let argv: Vec<String> = env::args().collect();
	let dir_to_compress = Path::new( argv.get(0).unwrap() );

	if !dir_to_compress.exists() {
		eprintln!( "Directory {} doesn't exist!", dir_to_compress.display() );
		exit(1)
	}

	let mut upkf = Upkf::new( dir_to_compress.file_name().unwrap().to_str().unwrap().to_string() );

	for entry in dir_to_compress.read_dir().unwrap() {
		let file = entry.unwrap();
		if file.metadata().unwrap().is_dir() {
			continue
		}
		println!( "Adding {}", file.path().to_str().unwrap() );
		let mut buf = vec![];
		File::open( file.path() ).unwrap().read( buf.as_mut_slice() );
		upkf.add_file(
			file.path().to_str().unwrap().to_string(),
			Bytes::from( buf )
		);
	}
	println!("Done")
}