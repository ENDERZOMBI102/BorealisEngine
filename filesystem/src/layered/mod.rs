pub mod layers;

use std::fs::File;
use std::io::{Error, ErrorKind, Read, Seek, SeekFrom};
use std::path::PathBuf;
use std::sync::Arc;
use vpk::entry::VPKEntry;
use crate::upkf::Element;

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
			LayeredFile::Vpk { path: _path, entry } => entry.get().unwrap().len() as u64
		}
	}

	pub fn read( &self ) -> Result<Vec<u8>, Error> {
		match self {
			LayeredFile::Rust { file } => {
				let mut file2 = file.clone();
				let mut vec: Vec<u8> = Vec::new();
				file2.read_to_end( &mut vec )?;
				Ok( vec )
			},
			LayeredFile::Upkf { element } => Ok( element.get_content().clone().to_vec() ),
			LayeredFile::Vpk { path, entry } => {
				let mut buf = vec![ 0u8; entry.dir_entry.file_length as usize ];
				let mut file = File::open( path ).unwrap();
				file.seek( SeekFrom::Start( entry.dir_entry.archive_offset as u64 ) )?;
				file.take( entry.dir_entry.file_length as u64 ).read( buf.as_mut_slice() )?;
				Ok( buf )
			}
		}
	}

	pub fn read_string( &self ) -> Result<String, Error> {
		match self {
			LayeredFile::Rust { file } => {
				let mut string = String::new();
				file.try_clone().unwrap().read_to_string( &mut string )?;
				Ok( string )
			},
			LayeredFile::Upkf { element } => {
				match String::from_utf8( element.get_content().clone().to_vec() ) {
					Ok( string ) => Ok( string ),
					Err( err ) => Err( Error::new( ErrorKind::InvalidData, err ) )
				}
			},
			LayeredFile::Vpk { path, entry } => {
				let mut string = String::new();
				let mut file = File::open( path ).unwrap();
				file.seek( SeekFrom::Start( entry.dir_entry.archive_offset as u64 ) )?;
				file.take( entry.dir_entry.file_length as u64 ).read_to_string( &mut string )?;
				Ok( string )
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
	pub layers: Vec< Box< dyn Layer > >
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
