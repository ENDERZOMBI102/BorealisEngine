pub mod layers;

use std::io::{Error, ErrorKind, Read, Seek};
use std::path::PathBuf;

pub type LayeredFile = Box<dyn ILayeredFile>;

trait ILayeredFile {
	fn size( &self ) -> u64;
	fn read( &self ) -> Result<Vec<u8>, Error>;
	fn read_string( &self ) -> Result<String, Error>;
	fn layer( &self ) -> HeapPtr<dyn Layer>;
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
