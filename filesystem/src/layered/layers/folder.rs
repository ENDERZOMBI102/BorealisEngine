use std::fs::File;
use std::io::{Error, Read};
use std::path::PathBuf;

use uuid::Uuid;

use crate::layered::*;

pub struct FolderLayerProvider { }
impl LayerProvider for FolderLayerProvider {
	fn supports( &self, path: &PathBuf ) -> bool {
		path.is_dir()
	}

	fn create( &self, path: &PathBuf, fs: &LayeredFS) -> Result<Arc<dyn Layer>, LayeredFSError> {
		Ok( Arc::new( FolderLayer::new( path, fs ) ) )
	}
}

pub struct FolderLayer<'a> {
	path: PathBuf,
	fs: &'a LayeredFS,
	uuid: Uuid
}

impl FolderLayer<'_> {
	pub(crate) fn new(og: &PathBuf, fs: &LayeredFS ) -> Self {
		FolderLayer { path: og.clone(), uuid: Uuid::new_v4(), fs: fs }
	}
}

impl Layer for FolderLayer<'_> {
	fn resolve( &self, filename: &str ) -> PathBuf {
		let mut path = self.path.clone();
		path.push( filename );
		path
	}

	fn contains( &self, filename: &str ) -> bool {
		self.resolve( filename ).exists()
	}

	fn get_file( &self, filename: &str ) -> Result<LayeredFile, Error> {
		let file = File::open( self.resolve( filename ) )?;
		Ok( Box::new( FolderLayeredFile { file: file, path: filename.to_string(), layer: self.fs.get_layer_reference( &self.uuid ).unwrap() } ) )
	}

	fn meta( &self ) -> LayerMeta {
		LayerMeta {
			origin: None,
			filename: self.path.to_str().unwrap().to_string(),
			size: None
		}
	}

	fn uuid( &self ) -> &Uuid {
		&self.uuid
	}
}


struct FolderLayeredFile {
	file: File,
	path: String,
	layer: Arc<dyn Layer>
}

impl ILayeredFile for FolderLayeredFile {
	fn size(&self) -> u64 {
		self.file.metadata().unwrap().len()
	}

	fn read(&self) -> Result<Vec<u8>, Error> {
		let mut file2 = self.file.try_clone()?;
		let mut vec: Vec<u8> = Vec::new();
		file2.read_to_end( &mut vec )?;
		Ok( vec )
	}

	fn read_string(&self) -> Result<String, Error> {
		let mut string = String::new();
		self.file.try_clone().unwrap().read_to_string( &mut string )?;
		Ok( string )
	}

	fn layer(&self) -> Arc<dyn Layer> {
		self.layer.clone()
	}

	fn path(&self) -> String {
		self.path.clone()
	}
}