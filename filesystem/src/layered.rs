use std::fs::File;
use std::path::{Path, PathBuf};
use bytes::{Buf, Bytes};
use bytes::buf::{Reader, Writer};

pub struct LayerMeta {
	pub origin: Option<String>,
	pub filename: String,
	pub size: Option<u64>
}

pub trait Layer {
	fn contains( &self, filename: &str ) -> bool;
	fn get_file( &self, filename: &str ) -> &File;
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

	fn get_file( &self, filename: &str ) -> Option<&File> {
		for layer in &self.layers {
			if layer.contains( filename ) {
				return Some( layer.get_file( filename ) );
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
	fn contains( &self, filename: &str ) -> bool {
		let mut path = self.path.clone();
		dbg!( filename );
		path.push( filename.split_at( filename.find( "\n" ).unwrap() ).0 );
		dbg!( &path, path.exists() );
		path.exists()
	}

	fn get_file( &self, filename: &str ) -> &File {
		todo!()
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
	let layer0 = FolderLayer { path: Path::new(".").to_path_buf() };
	let fs = LayeredFS { layers: vec![ &layer0 ] };

	let mut input = String::new();
	loop {
		match std::io::stdin().read_line( &mut input ) {
			Ok(n) => {
				let command: Vec<&str> = input.split(" ").collect();
				match command[0] {
					"has" => println!( "{}", fs.contains( command[1] ) ),
					"read" => println!( "{}", fs.get_file( command[1] ).unwrap().metadata().unwrap().len() ),
					&_ => {}
				}
			}
			Err(error) => println!("error: {}", error),
		}
	}
}