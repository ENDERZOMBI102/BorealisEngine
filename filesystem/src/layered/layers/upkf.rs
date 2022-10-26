use std::fs::File;
use std::io::{Error, ErrorKind};
use std::path::PathBuf;
use std::sync::Arc;
use path_slash::PathBufExt;
use crate::layered::*;
use crate::upkf::{Element, Upkf};

pub struct UpkfLayerProvider { }
impl LayerProvider for UpkfLayerProvider {
	fn supports( &self, path: &PathBuf) -> bool {
		if let Some( ext ) = path.extension() {
			return ext.to_str().unwrap() == "upkf" && path.exists();
		}
		false
	}

	fn create<'a>(&self, path: &PathBuf, fs: &'a LayeredFS) -> Result<Arc<dyn Layer + 'a>, LayeredFSError> {
		Ok( Arc::new( UpkfLayer::new( path, fs ) ) )
	}
}


pub struct UpkfLayer<'a> {
	upkf: Upkf,
	fs: &'a LayeredFS,
	uuid: Uuid
}

impl UpkfLayer<'_> {
	pub fn new( path: &PathBuf, fs: &LayeredFS ) -> Self {
		UpkfLayer {
			upkf: Upkf::load( path.as_path(), true ).unwrap(),
			uuid: Uuid::new_v4(),
			fs: fs,
		}
	}
}

impl Layer for UpkfLayer<'_> {
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

	fn get_file( &self, filename: &str ) -> Result<LayeredFile, Error> {
		for element in self.upkf.iter() {
			if element.get_path() == filename {
				return Ok( Box::new( UpkfLayeredFile { element: Arc::new( &element ), layer: self.fs.get_layer_reference( &self.uuid ).unwrap() } ) )
			}
		}
		Err( Error::new(ErrorKind::NotFound, format!("File {filename} was not found") ) )
	}

	fn meta( &self ) -> LayerMeta {
		LayerMeta {
			origin: Some( self.upkf.get_origin().to_string() ),
			filename: self.upkf.get_path().unwrap().to_str().unwrap().to_string(),
			size: Some( File::open( self.upkf.get_path().unwrap() ).unwrap().metadata().unwrap().len() )
		}
	}

	fn uuid(&self) -> &Uuid {
		&self.uuid
	}
}


struct UpkfLayeredFile<'a> {
	element: Arc<&'a Element>,
	layer: Arc<dyn Layer>
}

impl ILayeredFile for UpkfLayeredFile<'_> {
	fn size(&self) -> u64 {
		self.element.get_content().len() as u64
	}

	fn read(&self) -> Result<Vec<u8>, Error> {
		Ok( self.element.get_content().clone().to_vec() )
	}

	fn read_string(&self) -> Result<String, Error> {
		match String::from_utf8( self.element.get_content().clone().to_vec() ) {
			Ok( string ) => Ok( string ),
			Err( err ) => Err( Error::new( ErrorKind::InvalidData, err ) )
		}
	}

	fn layer(&self) -> Arc<dyn Layer> {
		self.layer.clone()
	}

	fn path(&self) -> String {
		self.element.get_path().to_string()
	}
}