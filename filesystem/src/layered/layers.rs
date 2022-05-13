use std::fs::File;
use std::io::ErrorKind;
use std::path::{Path, PathBuf};
use std::sync::Arc;
use path_slash::PathBufExt;
use crate::upkf::Upkf;
use crate::layered::{Layer, LayeredFile, LayerMeta};

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
			return Ok( LayeredFile::Rust { file: file.unwrap() } )
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

pub struct UpkfLayer {
	upkf: Upkf
}

impl UpkfLayer {
	pub fn from_buf( path: PathBuf ) -> Self {
		UpkfLayer {
			upkf: Upkf::load( path.as_path(), true )
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
				return Ok( LayeredFile::Upkf { element: Arc::new( &element ) } )
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
