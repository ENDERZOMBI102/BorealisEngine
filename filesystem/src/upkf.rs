use std::fs;
use std::fs::File;
use std::io::{Write};
use std::path::Path;
use byteorder::{ByteOrder, LittleEndian, WriteBytesExt};

pub struct UpkfHeader {
	signature: u32,
	version: u16,
	origin_size: u16,
	origin: String,
	entry_count: u64,
}

impl UpkfHeader {
	pub fn new( origin: String ) -> UpkfHeader {
		return UpkfHeader {
			signature: 0x464b5055,
			version: 0,
			origin_size: origin.len() as u16,
			origin: origin,
			entry_count: 0
		}
	}

	pub fn save( &self, path: &Path ) -> Result<(), ()> {
		let mut file = File::create( path ).unwrap();
		file.write_u32::<LittleEndian>( self.signature );
		file.write_u16::<LittleEndian>( self.version );
		file.write_u16::<LittleEndian>(self.origin_size );
		file.write(self.origin.as_bytes() );
		file.write_u64::<LittleEndian>(self.entry_count );
		Ok(())
	}
}

pub fn main() {
	let header = UpkfHeader::new( "UngineTest".to_string() );
	header.save( Path::new("data.upkf") );
}