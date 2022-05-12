use std::fs::File;
use std::io::{Read, Write};
use std::path::{Path, PathBuf};
use bytes::{Buf, Bytes};
use bytes::buf::{Reader, Writer};
use json::stringify;

pub struct LayerMeta {
	pub origin: Option<String>,
	pub filename: String,
	pub size: Option<u64>
}

pub trait Layer {
	fn resolve( &self, filename: &str ) -> PathBuf;
	fn contains( &self, filename: &str ) -> bool;
	fn get_file( &self, filename: &str ) -> File;
	fn meta( &self ) -> LayerMeta;
}

pub struct LayeredFS<'a> {
	layers: Vec<&'a dyn Layer>
}

impl LayeredFS<'a> {
	fn contains( &self, filename: &str ) -> bool {
		for layer in &self.layers {
			if layer.contains( filename ) {
				return true;
			}
		}
		false
	}

	fn get_file( &self, filename: &str ) -> Option<File> {
		for layer in &self.layers {
			if layer.contains( filename ) {
				return Some( layer.get_file( filename ) );
			}
		}
		None
	}

	fn resolve( &self, filename: &str ) -> Option<PathBuf> {
		for layer in &self.layers {
			if layer.contains( filename ) {
				return Some( layer.resolve( filename ) );
			}
		}
		None
	}
}

struct FolderLayer {
	path: PathBuf
}

impl FolderLayer {
	fn new( folder: &Path ) -> Self {
		let mut buf = PathBuf::new();
		buf.push( folder.clone() );
		FolderLayer {
			path: buf
		}
	}
}

impl Layer for FolderLayer {
	fn resolve(&self, filename: &str) -> PathBuf {
		let mut path = self.path.clone();
		path.push( filename.split_at( filename.find( "\n" ).unwrap() ).0 );
		path
	}
	
	fn contains( &self, filename: &str ) -> bool {
		self.resolve( filename ).exists()
	}

	fn get_file( &self, filename: &str ) -> File {
		File::open( self.resolve( filename ) ).unwrap()
	}

	fn meta( &self ) -> LayerMeta {
		LayerMeta {
			origin: None,
			filename: self.path.to_str().unwrap().to_string(),
			size: None
		}
	}
}

pub fn main() {
	let layer0 = FolderLayer { path: Path::new(".").canonicalize().unwrap().to_path_buf() };
	let layer1 = FolderLayer { path: Path::new("./../tast").canonicalize().unwrap().to_path_buf() };
	let mut fs = LayeredFS { layers: vec![&layer0, &layer1 ] };

	let mut input = String::new();
	loop {
		match std::io::stdin().read_line( &mut input ) {
			Ok(_n) => {
				let command: Vec<&str> = input.split(" ").collect();
				match command[0] {
					"has" => println!( "{}", fs.contains( command[1] ) ),
					"read" => {
						let mut file = fs.get_file( command[1] ).unwrap();
						let mut string = String::new();
						file.read_to_string( &mut string );
						println!( "{}", string )
					},
					"find" => println!( "{}", fs.resolve( command[1] ).unwrap().to_str().unwrap() ),
					"reverse\n" => {
						fs.layers.reverse();
						println!( "reversed layers order" )
					},
					cmd => println!( "Unknown command {}", cmd )
				}
			}
			Err(error) => println!("error: {}", error),
		}
		input.clear()
	}
}