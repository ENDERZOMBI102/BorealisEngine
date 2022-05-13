pub mod layers;

use std::fs::File;
use std::io::{ErrorKind, Read, Seek, SeekFrom, Write};
use std::ops::Index;
use std::path::{Path, PathBuf};
use std::sync::Arc;
use vpk::entry::VPKEntry;
use crate::upkf::Element;
use crate::layered::layers::{FolderLayer, UpkfLayer, VpkLayer};

pub enum LayeredFile<'a> {
	Rust { file: File },
	Upkf { element: Arc<&'a Element> },
	Vpk { path: String, entry: Arc<&'a VPKEntry> },
}

impl LayeredFile<'_> {
	pub fn size( &self ) -> u64 {
		return match self {
			LayeredFile::Rust { file } => file.metadata().unwrap().len(),
			LayeredFile::Upkf { element } => element.get_content().len() as u64,
			LayeredFile::Vpk { path, entry } => entry.get().unwrap().len() as u64
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
			LayeredFile::Upkf { element } => element.get_content().clone().to_vec(),
			LayeredFile::Vpk { path, entry } => {
				let mut buf = vec![ 0u8; entry.dir_entry.file_length as usize ];
				let mut file = File::open( path ).unwrap();
				file.seek( SeekFrom::Start( entry.dir_entry.archive_offset as u64 ) );
				file.take( entry.dir_entry.file_length as u64 ).read( buf.as_mut_slice() );
				buf
			}
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

pub fn main() {
	let mut fs = LayeredFS { layers: vec![ ] };

	let mut input = String::new();
	loop {
		print!( ">>> " );
		std::io::stdout().flush();
		match std::io::stdin().read_line( &mut input ) {
			Ok(_n) => {
				input.remove_matches("\n");
				input.remove_matches("\r");
				let command: Vec<&str> = input.split(" ").collect();
				match command[0] {
					"has" => println!("{}", fs.contains( command[1] ) ),
					"read" => unsafe {
						let file = fs.get_file( command[1] );
						if file.is_ok() {
							println!( "{}", String::from_utf8_unchecked( file.unwrap().read() ) )
						} else {
							println!( "File {} was not found", command[1] )
						}
					},
					"find" => println!( "{}", fs.resolve( command[1] ).unwrap().to_str().unwrap() ),
					"reverse" => {
						fs.layers.reverse();
						println!("reversed layers order")
					},
					"addLayer" => {
						let prepend = command[1] == "pre";
						let path = Path::new( command[2] ).canonicalize().unwrap();
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
										ext => println!( "ERROR: Unsopported file type: {}", ext )
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
					"listFiles" => {
						println!( "TODO: FINISH THIS" )
					},
					"help" => {
						println!( "Available commands:" );
						println!( " - has $PATH: Prints true if the file exists, false otherwise" );
						println!( " - read $PATH: Prints the contents of the file, if found" );
						println!( " - find $PATH: Prints the full path to the file, if found" );
						println!( " - addlayer $PREPEND $PATH: Adds a layer to the fs, may be a path to a folder or .upkf/.vpk file" );
						println!( " - listLayers: Lists all layers in this LFS" );
						println!( " - listFiles: Lists all files present in all layers" );
						println!( " - reverse: Reverses the order of the layers" );
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