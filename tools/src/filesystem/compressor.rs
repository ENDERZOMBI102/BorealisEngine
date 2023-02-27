use std::env;
use std::fs::File;
use std::io::Read;
use std::os::windows::fs::MetadataExt;
use std::path::Path;
use std::process::exit;
use bytes::Bytes;
use upkf::{CompressionType, Upkf, UpkfMeta};
use upkf::CompressionType::{NONE, LZMA, GZIP, LZMA2};

pub fn main() {
	let argv: Vec<String> = env::args().collect();
	let dir_to_compress = Path::new( argv.get(1)? );
	let no_data_text: String = String::from("{}");

	let compression: CompressionType;
	if argv.contains( &"--lzma".to_string() ) {
		compression = LZMA;
	} else if argv.contains(&"--lzma2".to_string() ) {
		compression = LZMA2;
	} else if argv.contains( &"--gzip".to_string() ) {
		compression = GZIP;
	} else {
		compression = NONE;
	}

	if !dir_to_compress.exists() {
		eprintln!( "Directory {} doesn't exist!", dir_to_compress.display() );
		exit(1)
	}

	println!( "Compressing directory \"{}\" with default algorithm {}", dir_to_compress.display(), compression.name() );

	let mut upkf = Upkf::new( dir_to_compress.file_name()?.to_str()?.to_string() );

	for entry in walkdir::WalkDir::new( dir_to_compress ) {
		let file = entry?;
		// ignore directories and metadata files
		if file.metadata()?.is_dir() || String::from( file.file_name().to_str()? ).ends_with( ".upkfmeta" ) {
			continue
		}
		let raw_path = file.path().to_str()?.replace("\\", "/");
		let path = Path::new( raw_path.as_str() );
		println!( "Adding {}", path.to_str()? );
		// check for metadata
		let meta_path = path.parent()?.join( String::from( path.file_name()?.to_str()? ) +  ".upkfmeta" );
		let mut end_meta: Option<UpkfMeta> = None;
		if meta_path.exists() && meta_path.is_file() {
			println!( "\t- Metadata file found, verying it" );
			let meta = UpkfMeta::deserialize( &mut File::open( meta_path )?, compression );
			if meta.is_err() {
				eprintln!( "\t- Metadata is not valid, ignoring" );
			} else {
				let mut text = &meta?.get_string_meta();
				if text.is_empty() {
					text = &no_data_text;
				}
				println!( "\t- Metadata is valid" );
				println!( "\t\t- Compression: {}", &meta2.get_compression().name() );
				println!( "\t\t- Metadata: {}", text );
				println!( "\t\t- Binary: {}", &meta2.is_binary() );
				end_meta = Some( meta2 );
			}
		} else if meta_path.exists() {
			// no need for is_file check, we already declared its not
			eprintln!( "\t- Something named {} exists, but its not a file, ignoring", meta_path.display() );
		} else {
			println!( "\t- No .upkfmeta file found" );
		}
		let mut buf = vec![0; file.metadata()?.file_size() as usize ];
		File::open( &path )?.read( &mut buf )?;
		let meta2 = end_meta.unwrap_or( UpkfMeta::default( compression ) );
		upkf.add_file(
			path.to_str()?.to_string(),
			meta2.get_string_meta(),
			Bytes::from( buf ),
			meta2.is_binary(),
			meta2.get_compression()
		);
	}
	println!( "Saving into {}.upkf", dir_to_compress.to_str()? );
	let mut filename = String::new();
	filename = filename + dir_to_compress.file_name()?.to_str()? + ".upkf";
	upkf.save( Path::new( &filename ) )?;
	println!("Done")
}
