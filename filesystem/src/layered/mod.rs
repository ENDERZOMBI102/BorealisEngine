use std::io::{Error, ErrorKind};
use std::path::PathBuf;
use std::rc::Rc;
use std::sync::Arc;

use uuid::Uuid;

use crate::layered::layers::folder::FolderLayerProvider;
use crate::layered::layers::upkf::UpkfLayerProvider;
use crate::layered::layers::vpk::VpkLayerProvider;

pub mod layers;

pub type LayeredFile<'a> = Box<dyn ILayeredFile + 'a>;

pub enum LayeredFSError {
	NoExtension,
	Unsupported(String)
}

pub trait LayerProvider: Sync + Send {
	fn supports( &self, path: &PathBuf ) -> bool;
	fn create<'a>( &self, path: PathBuf, fs: Rc<&'a LayeredFS> ) -> Result<Arc<dyn Layer + 'a>, LayeredFSError>;
}

pub trait ILayeredFile {
	fn size( &self ) -> u64;
	fn read( &self ) -> Result<Vec<u8>, Error>;
	fn read_string( &self ) -> Result<String, Error>;
	fn layer( &self ) -> Arc<dyn Layer>;
	fn path( &self ) -> String;
}

pub struct LayerMeta {
	pub origin: Option<String>,
	pub filename: String,
	pub size: Option<u64>
}

#[allow(clippy::needless_lifetimes)]
pub trait Layer {
	fn resolve( &self, filename: &str ) -> PathBuf;
	fn contains( &self, filename: &str ) -> bool;
	fn get_file<'a>( &'a self, filename: &str ) -> Result<LayeredFile<'a>, Error>;
	fn meta( &self ) -> LayerMeta;
	fn uuid( &self ) -> &Uuid;
}

pub struct LayeredFS<'a> {
	providers: Vec< Box<dyn LayerProvider + 'a> >,
	pub layers: Vec< Arc< dyn Layer + 'a> >
}

impl<'a> LayeredFS<'a> {
	pub fn new() -> Self {
		LayeredFS {
			providers: vec![
				Box::new( FolderLayerProvider { } ),
				Box::new( UpkfLayerProvider { } ),
				Box::new( VpkLayerProvider { } )
			],
			layers: Vec::new()
		}
	}

	pub fn contains( &self, filename: &str ) -> bool {
		for layer in &self.layers {
			if layer.contains( filename ) {
				return true;
			}
		}
		false
	}

	pub fn get_file( &self, filename: &str ) -> Result<LayeredFile, Error> {
		for layer in &self.layers {
			if layer.contains( filename ) {
				return layer.get_file( filename );
			}
		}
		Err( Error::new(ErrorKind::NotFound, format!("File {filename} was not found") ) )
	}

	pub fn resolve( &self, filename: &str ) -> Option<PathBuf> {
		for layer in &self.layers {
			if layer.contains( filename ) {
				return Some( layer.resolve( filename ) );
			}
		}
		None
	}

	pub fn add_layer( &'a mut self, path: PathBuf, prepend: bool ) -> Result<(), LayeredFSError> {
		for provider in &self.providers {
			if provider.supports( &path ) {
				let layer = provider.create( path, Rc::new( self) )?;
				if prepend {
					self.layers.insert( 0, layer )
				} else {
					self.layers.push( layer )
				}
				return Ok(())
			}
		}
		match path.extension() {
			None => Err( LayeredFSError::NoExtension ),
			Some( ext ) => Err( LayeredFSError::Unsupported { 0: ext.to_str().unwrap().to_string() } )
		}
	}

	pub fn add_layer_provider( &mut self, provider: Box<dyn LayerProvider> ) {
		self.providers.push( provider )
	}

	pub(crate) fn get_layer_reference(&self, uuid: &Uuid ) -> Option<Arc<dyn Layer + 'a>> {
		for layer in &self.layers {
			if layer.uuid() == uuid {
				return Some(layer.clone())
			}
		}
		None
	}

	pub fn layer_count( &self ) -> usize {
		self.layers.len()
	}
}
