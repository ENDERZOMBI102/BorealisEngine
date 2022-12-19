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

	fn create<'a>( &self, path: PathBuf ) -> Result<Arc<dyn Layer + 'a>, LayeredFSError> {
		Ok( Arc::new( FolderLayer::new( path ) ) )
	}
}

pub struct FolderLayer {
	path: PathBuf,
	uuid: Uuid
}

impl FolderLayer {
	pub fn new<'a>( path: PathBuf ) -> FolderLayer {
		FolderLayer { path, uuid: Uuid::new_v4() }
	}
}

impl<'a> Layer<'a> for FolderLayer {
	fn resolve( &self, filename: &str ) -> PathBuf {
		let mut path = self.path.clone();
		path.push( filename );
		path
	}

	fn contains( &self, filename: &str ) -> bool {
		self.resolve( filename ).exists()
	}

	fn get_file(&self, filename: &str ) -> Result<LayeredFile, Error> {
		let file = File::open( self.resolve( filename ) )?;
		Ok( Box::new( FolderLayeredFile {
			file: file,
			path: filename.to_string(),
			layer: &self.uuid
		}))
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


struct FolderLayeredFile<'a> {
	file: File,
	path: String,
	layer: &'a Uuid
}

impl<'a> ILayeredFile<'a> for FolderLayeredFile<'a> {
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

	fn layer(&self) -> &'a Uuid {
		self.layer
	}

	fn path(&self) -> String {
		self.path.clone()
	}
}
