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

	fn create<'a>(&self, path: PathBuf, fs: Rc<&'a LayeredFS>) -> Result<Arc<dyn Layer + 'a>, LayeredFSError> {
		Ok( Arc::new( VpkLayer::new( path, fs ) ) )
	}
}

pub struct VpkLayer<'a> {
	path: PathBuf,
	vpk: VPK,
	fs: Rc<&'a LayeredFS<'a>>,
	uuid: Uuid,
}

impl VpkLayer<'_> {
	pub fn new<'a>( path: PathBuf, fs: Rc<&'a LayeredFS>, ) -> VpkLayer<'a> {
		VpkLayer {
			vpk: vpk::from_path( path.as_path().to_str().unwrap() ).unwrap(),
			path: path,
			fs: fs,
			uuid: Uuid::new_v4()
		}
	}
}

impl<'a> Layer for VpkLayer<'a> {
	fn resolve( &self, filename: &str ) -> PathBuf {
		let mut path = PathBuf::from( String::from( self.path.to_str().unwrap() ) + "!" );
		path.push( filename );
		path.to_slash().unwrap().parse().unwrap()
	}

	fn contains( &self, filename: &str ) -> bool {
		self.vpk.tree.contains_key( filename )
	}

	fn get_file(&self, filename: &str) -> Result<LayeredFile<'a>, Error> {
		for ( name, entry ) in self.vpk.tree.iter() {
			if name == filename {
				return Ok( Box::new( VpkLayeredFile {
					path: self.path.to_str().unwrap().to_string(),
					entry: Arc::new( &entry ),
					layer: self.fs.get_layer_reference( &self.uuid ).unwrap()
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
	layer: Arc<dyn Layer>
}

impl ILayeredFile for VpkLayeredFile<'_> {
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

	fn layer(&self) -> Arc<dyn Layer> {
		self.layer.clone()
	}

	fn path(&self) -> String {
		self.path.clone()
	}
}
