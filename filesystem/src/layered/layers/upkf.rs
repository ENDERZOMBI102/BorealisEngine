use std::fs::File;
use std::io::{Error, ErrorKind};
use std::path::PathBuf;
use std::sync::Arc;
use path_slash::PathBufExt;
use crate::layered::{ILayeredFile, Layer, LayeredFile, LayerMeta};
use crate::upkf::{Element, Upkf};

pub struct UpkfLayer {
	upkf: Upkf
}

impl UpkfLayer {
	pub fn from_buf( path: PathBuf ) -> Self {
		UpkfLayer {
			upkf: Upkf::load( path.as_path(), true ).unwrap()
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
				return Ok( Box::new( UpkfLayeredFile { element: Arc::new( &element ), layer: self } ) )
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


struct UpkfLayeredFile<'a> {
	element: Arc<&'a Element>,
	layer: HeapPtr<UpkfLayer>
}

impl ILayeredFile for UpkfLayeredFile<'_> {
	fn size(&self) -> u64 {
		element.get_content().len() as u64
	}

	fn read(&self) -> Result<Vec<u8>, Error> {
		Ok( element.get_content().clone().to_vec() )
	}

	fn read_string(&self) -> Result<String, Error> {
		match String::from_utf8( element.get_content().clone().to_vec() ) {
			Ok( string ) => Ok( string ),
			Err( err ) => Err( Error::new( ErrorKind::InvalidData, err ) )
		}
	}

	fn layer(&self) -> HeapPtr<dyn Layer> {
		self.layer
	}
}