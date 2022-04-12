use std::env;
use std::fs::File;
use std::io::Read;
use std::os::windows::fs::MetadataExt;
use std::path::Path;
use std::process::exit;
use bytes::Bytes;
use crate::upkf::{CompressionType, Upkf};
use crate::upkf::CompressionType::{NONE, LZMA, GZIP, BZIP2, LZMA2};

pub fn main() {
	let argv: Vec<String> = env::args().collect();
	let dir_to_compress = Path::new( argv.get(1).unwrap() );

	let compression: CompressionType;
	if argv.contains( &"--lzma".to_string() ) {
		compression = LZMA;
	} else if argv.contains(&"--lzma2".to_string() ) {
		compression = LZMA2;
	} else if argv.contains( &"--gzip".to_string() ) {
		compression = GZIP;
	} else if argv.contains( &"--bzip2".to_string() ) {
		compression = BZIP2;
	} else {
		compression = NONE;
	}

	if !dir_to_compress.exists() {
		eprintln!( "Directory {} doesn't exist!", dir_to_compress.display() );
		exit(1)
	}

	println!( "Compressing directory \"{}\" with algorithm {:?}", dir_to_compress.display(), compression );

	let mut upkf = Upkf::new( dir_to_compress.file_name().unwrap().to_str().unwrap().to_string() );

	for entry in walkdir::WalkDir::new( dir_to_compress ) {
		let file = entry.unwrap();
		if file.metadata().unwrap().is_dir() {
			continue
		}
		println!( "Adding {}", file.path().to_str().unwrap() );
		let mut buf = vec![ 0; file.metadata().unwrap().file_size() as usize ];
		File::open( file.path() ).unwrap().read( &mut buf );
		upkf.add_binary_file(
			file.path().to_str().unwrap().to_string(),
			Bytes::from( buf ),
			compression
		);
	}
	println!( "Saving into {}.upkf", dir_to_compress.to_str().unwrap() );
	let mut filename = String::new();
	filename = filename + dir_to_compress.file_name().unwrap().to_str().unwrap() + ".upkf";
	upkf.save( Path::new( &filename ) );
	println!("Done")
}