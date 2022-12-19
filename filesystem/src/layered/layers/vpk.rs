use std::fs::File;
use std::io::{Error, ErrorKind, Read, Seek, SeekFrom};
use std::path::PathBuf;
use std::sync::Arc;

use path_slash::PathBufExt;
use uuid::Uuid;
use vpk::entry::VPKEntry;
use vpk::VPK;

use crate::layered::*;

pub struct VpkLayerProvider { }
impl LayerProvider for VpkLayerProvider {
	fn supports( &self, path: &PathBuf) -> bool {
		if let Some( ext ) = path.extension() {
			return ext.to_str().unwrap() == "vpk" && path.exists();
		}
		false
	}

	fn create<'a>( &self, path: PathBuf ) -> Result<Arc<dyn Layer + 'a>, LayeredFSError> {
		Ok( Arc::new( VpkLayer::new( path ) ) )
	}
}

pub struct VpkLayer {
	path: PathBuf,
	vpk: VPK,
	uuid: Uuid,
}

impl VpkLayer {
	pub fn new( path: PathBuf ) -> VpkLayer {
		VpkLayer {
			vpk: vpk::from_path( path.as_path().to_str().unwrap() ).unwrap(),
			path: path,
			uuid: Uuid::new_v4()
		}
	}
}

impl<'a> Layer<'a> for VpkLayer {
	fn resolve( &self, filename: &str ) -> PathBuf {
		let mut path = PathBuf::from( String::from( self.path.to_str().unwrap() ) + "!" );
		path.push( filename );
		path.to_slash().unwrap().parse().unwrap()
	}

	fn contains( &self, filename: &str ) -> bool {
		self.vpk.tree.contains_key( filename )
	}

	fn get_file(&'a self, filename: &str) -> Result<LayeredFile<'a>, Error> {
		for ( name, entry ) in self.vpk.tree.iter() {
			if name == filename {
				return Ok( Box::new( VpkLayeredFile {
					path: self.path.to_str().unwrap().to_string(),
					entry: Arc::new( &entry ),
					layer: &self.uuid
				}))
			}
		}
		Err( Error::new(ErrorKind::NotFound, format!("File {filename} was not found") ) )
	}

	fn meta( &self ) -> LayerMeta {
		LayerMeta {
			origin: None,
			filename: self.path.to_str().unwrap().to_string(),
			size: Some( File::open( &self.path ).unwrap().metadata().unwrap().len() )
		}
	}

	fn uuid(&self) -> &Uuid {
		&self.uuid
	}
}


struct VpkLayeredFile<'a> {
	path: String,
	entry: Arc<&'a VPKEntry>,
	layer: &'a Uuid
}

impl<'a> ILayeredFile<'a> for VpkLayeredFile<'a> {
	fn size( &self ) -> u64 {
		self.entry.get().unwrap().len() as u64
	}

	fn read( &self ) -> Result<Vec<u8>, Error> {
		let mut buf = vec![ 0u8; self.entry.dir_entry.file_length as usize ];
		let mut file = File::open( &self.path ).unwrap();
		file.seek( SeekFrom::Start( self.entry.dir_entry.archive_offset as u64 ) )?;
		file.take( self.entry.dir_entry.file_length as u64 ).read( buf.as_mut_slice() )?;
		Ok( buf )
	}

	fn read_string( &self ) -> Result<String, Error> {
		let mut string = String::new();
		let mut file = File::open( &self.path ).unwrap();
		file.seek( SeekFrom::Start( self.entry.dir_entry.archive_offset as u64 ) )?;
		file.take( self.entry.dir_entry.file_length as u64 ).read_to_string( &mut string )?;
		Ok( string )
	}

	fn layer(&self) -> &'a Uuid {
		self.layer
	}

	fn path(&self) -> String {
		self.path.clone()
	}
}
