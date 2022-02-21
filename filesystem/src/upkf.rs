use std::fs;
use std::fs::File;
use std::io::{Read, Write};
use std::fmt::{Debug, Display};
use std::path::Path;
use byteorder::{ByteOrder, LittleEndian, ReadBytesExt, WriteBytesExt};
use bytes::{Bytes};
use savefile::IntrospectorNavCommand::Up;

#[derive(Debug)]
enum UpkfError {
	NotAnUpkFileError,
	CorruptedDataError,
	VersionNotSupportedError
}

#[derive(Debug)]
enum ElementType {
	TEXT,
	BYTES
}

pub struct Upkf {
	file: String,  // actual fs path
	origin: String,  // origin of the pak
	entries: Vec<Element>  // entries
}

impl Upkf {
	pub fn load( path: &Path ) -> Self {
		let mut file = File::open( path ).unwrap();
		let header = FileHeader::load( &file ).unwrap();
		println!("{:?}", header);
		Self {
			file: path.to_str().unwrap().to_string(),
			origin: header.origin,
			entries: vec![]
		}
	}

	pub fn get_path( &self ) -> &Path {
		return Path::new( &self.file )
	}
}

struct Element {
	path: String,
	bytes: Bytes
}

#[derive(Debug)]
struct FileHeader {
	signature: u32,
	version: u16,
	origin_size: u16,
	origin: String,
	entry_count: u64
}

impl FileHeader {
	fn new( origin: String ) -> FileHeader {
		return FileHeader {
			signature: 0x464b5055,
			version: 0,
			origin_size: origin.len() as u16,
			origin: origin,
			entry_count: 0
		}
	}

	fn save( &self, path: &Path ) -> Result<(), ()> {
		let mut file = File::create( path ).unwrap();
		file.write_u32::<LittleEndian>( self.signature );
		file.write_u16::<LittleEndian>( self.version );
		file.write_u16::<LittleEndian>(self.origin_size );
		file.write(self.origin.as_bytes() );
		file.write_u64::<LittleEndian>(self.entry_count );
		Ok(())
	}

	fn load(mut file: &File ) -> Result<Self, UpkfError> {
		let signature = file.read_u32::<LittleEndian>().unwrap();
		if signature != 0x464b5055 {
			return Result::Err(UpkfError::NotAnUpkFileError)
		}
		let version = file.read_u16::<LittleEndian>().unwrap();
		if version != 0 {
			return Result::Err(UpkfError::VersionNotSupportedError)
		}
		let origin_size = file.read_u16::<LittleEndian>().unwrap();
		let mut buf = vec![1 as u8; origin_size as usize ];
		file.read_exact( &mut buf );
		let origin = String::from_utf8( buf ).unwrap();
		let entry_count = file.read_u64::<LittleEndian>().unwrap();
		return Ok(
			FileHeader {
				signature: signature,
				version: version,
				origin_size: origin_size,
				origin: origin,
				entry_count: entry_count
			}
		)
	}
}

struct EntryHeader {
	size: u64,
	name_size: u32,
	name: String,
	binary: bool,
	next_entry_offset: u32,
	metadata_size: u32,
	metadata: String
}

impl EntryHeader {
	
}

struct Entry {
	data: Bytes
}

impl Entry {
	
}

pub fn main() {
	Upkf::load( Path::new("./data.upkf") );
}