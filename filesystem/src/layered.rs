use std::fs::File;
use std::io::{ErrorKind, Read, Write};
use std::path::{Path, PathBuf};
use std::sync::Arc;
use bytes::{Buf, Bytes};
use bytes::buf::{Reader, Writer};
use path_slash::PathBufExt;
use crate::upkf::{Element, Upkf};

pub enum LayeredFile<'a> {
	Rust { file: File },
	Upkf { element: Arc<&'a Element> },
}

impl LayeredFile<'_> {
	pub fn size( &self ) -> u64 {
		return match self {
			LayeredFile::Rust { file } => file.metadata().unwrap().len(),
			LayeredFile::Upkf { element } => element.get_content().len() as u64
		}
	}

	pub fn read( &self ) -> Vec<u8> {
		return match self {
			LayeredFile::Rust { file } => {
				let mut file2 = file.clone();
				let mut vec: Vec<u8> = Vec::new();
				file2.read_to_end( &mut vec );
				vec
			},
			LayeredFile::Upkf { element } => element.get_content().clone().to_vec()
		}
	}
}

pub struct LayerMeta {
	pub origin: Option<String>,
	pub filename: String,
	pub size: Option<u64>
}

pub trait Layer {
	fn resolve( &self, filename: &str ) -> PathBuf;
	fn contains( &self, filename: &str ) -> bool;
	fn get_file( &self, filename: &str ) -> Result< LayeredFile, ErrorKind >;
	fn meta( &self ) -> LayerMeta;
}

pub struct LayeredFS {
	layers: Vec< Box< dyn Layer > >
}

impl LayeredFS {
	pub fn contains( &self, filename: &str ) -> bool {
		for layer in &self.layers {
			if layer.contains( filename ) {
				return true;
			}
		}
		false
	}

	pub fn get_file( &self, filename: &str ) -> Result<LayeredFile, ErrorKind> {
		for layer in &self.layers {
			if layer.contains( filename ) {
				return layer.get_file( filename );
			}
		}
		Err( ErrorKind::NotFound )
	}

	pub fn resolve( &self, filename: &str ) -> Option<PathBuf> {
		for layer in &self.layers {
			if layer.contains( filename ) {
				return Some( layer.resolve( filename ) );
			}
		}
		None
	}

	pub fn add_layer( &mut self, layer: Box<dyn Layer>, prepend: bool ) {
		if prepend {
			self.layers.insert( 0, layer )
		} else {
			self.layers.push(layer)
		}
	}

	pub fn layer_count( &self ) -> usize {
		self.layers.len()
	}

}

struct FolderLayer {
	path: PathBuf
}
impl FolderLayer {
	fn from_path( folder: &Path ) -> Self {
		FolderLayer {
			path: folder.clone().to_path_buf()
		}
	}

	fn from_buf( folder: PathBuf ) -> Self {
		FolderLayer {
			path: folder.clone()
		}
	}
}

impl Layer for FolderLayer {
	fn resolve( &self, filename: &str ) -> PathBuf {
		let mut path = self.path.clone();
		path.push( filename );
		path
	}
	
	fn contains( &self, filename: &str ) -> bool {
		self.resolve( filename ).exists()
	}

	fn get_file( &self, filename: &str ) -> Result<LayeredFile, ErrorKind> {
		let file = File::open( self.resolve( filename ) );
		if file.is_ok() {
			return Ok( LayeredFile::Rust { file: file.unwrap() } )
		}
		Err( ErrorKind::Other )
	}

	fn meta( &self ) -> LayerMeta {
		LayerMeta {
			origin: None,
			filename: self.path.to_str().unwrap().to_string(),
			size: None
		}
	}
}

struct UpkfLayer {
	upkf: Upkf
}

impl UpkfLayer {
	fn from_buf( path: PathBuf ) -> Self {
		UpkfLayer {
			upkf: Upkf::load( path.as_path(), true )
		}
	}
}

impl Layer for UpkfLayer {
	fn resolve( &self, filename: &str ) -> PathBuf {
		let mut path = PathBuf::from( String::from( self.upkf.get_path().unwrap().to_str().unwrap() ) + "!" );
		path.push( filename );
		path.to_slash().unwrap().parse().unwrap()
	}

	fn contains( &self, filename: &str ) -> bool {
		for file in self.upkf.iter() {
			if file.get_path() == filename {
				return true
			}
		}
		false
	}

	fn get_file( &self, filename: &str ) -> Result<LayeredFile, ErrorKind> {
		for element in self.upkf.iter() {
			if element.get_path() == filename {
				return Ok( LayeredFile::Upkf { element: Arc::new( &element ) } )
			}
		}
		Err( ErrorKind::NotFound )
	}

	fn meta( &self ) -> LayerMeta {
		LayerMeta {
			origin: Some( self.upkf.get_origin().to_string() ),
			filename: self.upkf.get_path().unwrap().to_str().unwrap().to_string(),
			size: Some( File::open( self.upkf.get_path().unwrap() ).unwrap().metadata().unwrap().len() )
		}
	}
}


pub fn main() {
	let mut fs = LayeredFS { layers: vec![
		Box::new( FolderLayer { path: Path::new(".").canonicalize().unwrap().to_path_buf() } ),
		Box::new( FolderLayer { path: Path::new("./../tast").canonicalize().unwrap().to_path_buf() } ),
		// Box::new( UpkfLayer { upkf: Upkf::load( Path::new("./cfg.upkf"), true ) } )
	] };

	let mut input = String::new();
	loop {
		match std::io::stdin().read_line( &mut input ) {
			Ok(_n) => {
				let command: Vec<&str> = input.split(" ").collect();
				match command[0] {
					"has" => println!("{}", fs.contains( fix_path( command[1] ) ) ),
					"read" => unsafe {
						let mut file = fs.get_file( fix_path( command[1] ) );
						if file.is_ok() {
							println!( "{}", String::from_utf8_unchecked( file.unwrap().read() ) )
						} else {
							println!( "File {} was not found", fix_path( command[1] ) )
						}
					},
					"find" => println!( "{}", fs.resolve( fix_path( command[1] ) ).unwrap().to_str().unwrap() ),
					"reverse\n" => {
						fs.layers.reverse();
						println!("reversed layers order")
					},
					"addLayer" => {
						let prepend = command[1] == "pre";
						let path = Path::new( fix_path( command[2] ) ).canonicalize().unwrap();
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
										ext => println!( "ERROR: Unsopported file type: {}", ext )
									}
								}
							}
						}
					},
					"help\n" => {
						println!( "Available commands:" );
						println!( " - has $PATH: Prints true if the file exists, false otherwise" );
						println!( " - read $PATH: Prints the contents of the file, if found" );
						println!( " - find $PATH: Prints the full path to the file, if found" );
						println!( " - addlayer $PREPEND $PATH: Adds a layer to the fs, may be a path to a folder or .upkf file" );
						println!( " - reverse: Reverses the order of the layers" );
						println!( " - help: Shows the help message" );
					},
					cmd => println!( "ERROR: Unknown command {}", cmd )
				}
			}
			Err(error) => println!("ERROR: {}", error)
		}
		input.clear()
	}
}

fn fix_path( path: &'a str ) -> &'a str {
	path.split_at( path.find( "\n" ).unwrap() ).0
}