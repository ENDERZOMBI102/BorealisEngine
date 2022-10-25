use std::fs::File;
use std::io::{Error, ErrorKind};
use std::path::{Path, PathBuf};

use crate::layered::{ILayeredFile, Layer, LayeredFile, LayerMeta};

pub struct FolderLayer {
	path: PathBuf
}

impl FolderLayer {
	pub fn from_path( folder: &Path ) -> Self {
		FolderLayer {
			path: folder.clone().to_path_buf()
		}
	}

	pub fn from_buf( folder: PathBuf ) -> Self {
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
			return Ok( Box::new( FolderLayeredFile { file: file.unwrap(), layer: &self } ) )
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


struct FolderLayeredFile {
	file: File,
	layer: HeapPtr<FolderLayer>
}

impl ILayeredFile for FolderLayeredFile {
	fn size(&self) -> u64 {
		file.metadata().unwrap().len()
	}

	fn read(&self) -> Result<Vec<u8>, Error> {
		let mut file2 = file.clone();
		let mut vec: Vec<u8> = Vec::new();
		file2.read_to_end( &mut vec )?;
		Ok( vec )
	}

	fn read_string(&self) -> Result<String, Error> {
		let mut string = String::new();
		file.try_clone().unwrap().read_to_string( &mut string )?;
		Ok( string )
	}

	fn layer(&self) -> HeapPtr<dyn Layer> {
		self.layer
	}
}