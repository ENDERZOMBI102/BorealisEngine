pub mod layers;

use tier0::types::HeapPtr;
use std::io::{Error, ErrorKind};
use std::path::PathBuf;
use std::rc::Rc;
use std::sync::Arc;
use uuid::Uuid;

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
	fn uuid( &self ) -> &Uuid;
}

pub struct LayeredFS {
	pub layers: Vec< HeapPtr< dyn Layer > >
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
			self.layers.insert( 0, Arc::new( Rc::new( layer ) ) )
		} else {
			self.layers.push( Arc::new( Rc::new( layer ) ) )
		}
	}

	pub(crate) fn get_layer_reference( layer: &Uuid ) -> HeapPtr<dyn Layer> {

	}

	pub fn layer_count( &self ) -> usize {
		self.layers.len()
	}
}
