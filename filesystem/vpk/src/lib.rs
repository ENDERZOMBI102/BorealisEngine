mod exts;

use std::fs::File;
use std::io::Read;
use std::path::Path;

use byteorder::{LittleEndian, ReadBytesExt};
use thiserror::Error;
use crate::exts::ReadNullStringExt;

#[derive(Error, Debug)]
pub enum VpkError {
	#[error("some IO error occurred")]
    IoError(#[from] std::io::Error),
	#[error("unsupported VPK version")]
	UnsupportedVersion(u32),
	#[error("invalid vpk directory entry terminator")]
	InvalidEntryTerminator(u16),
	#[error("unknown data store error")]
	Unknown
}

#[derive(Debug, Eq, PartialEq)]
pub enum VpkHeader {
	V1 {
		signature: u32,
		/// The size, in bytes, of the directory tree
		directory_length: u32
	},
	V2 {
		signature: u32,
		/// The size, in bytes, of the directory tree
		directory_length: u32,
		/// How many bytes of file content are stored in this VPK file (0 in CSGO)
		file_data_section_size: u32,
		/// The size, in bytes, of the section containing MD5 checksums for external archive content
		archive_md5_section_size: u32,
		/// The size, in bytes, of the section containing MD5 checksums for content in this file (should always be 48)
		other_md5_section_size: u32,
		/// The size, in bytes, of the section containing the public key and signature. This is either 0 (CSGO & The Ship) or 296 (HL2, HL2:DM, HL2:EP1, HL2:EP2, HL2:LC, TF2, DOD:S & CS:S)
		signature_section_size: u32
	}
}

#[derive(Debug, Eq, PartialEq)]
pub struct VpkDirEntry {
	pub path: String,
	pub crc: u32,
	pub preload_bytes: u16,
	pub archive_index: u16,
	pub entry_offset: u32,
	pub entry_length: u32,
	pub preloaded_bytes: Vec<u8>
}

pub fn load<P: AsRef<Path>>( file: P ) -> Result<(), VpkError> {
	println!( "Loading from {:?}", file.as_ref() );
    let mut file = File::open( file.as_ref() )?;

	let signature = file.read_u32::<LittleEndian>()?;
	let version = file.read_u32::<LittleEndian>()?;
	let directory_length = file.read_u32::<LittleEndian>()?;

	let header = match version {
		1 => VpkHeader::V1 { signature, directory_length },
		2 => VpkHeader::V2 {
			signature,
			directory_length,
			file_data_section_size: file.read_u32::<LittleEndian>()?,
			archive_md5_section_size: file.read_u32::<LittleEndian>()?,
			other_md5_section_size: file.read_u32::<LittleEndian>()?,
			signature_section_size: file.read_u32::<LittleEndian>()?
		},
		it => return Err( VpkError::UnsupportedVersion( it ) )
	};

	println!( "{:?}", header );
	let mut entries = Vec::new();

	while let Ok( extension ) = file.read_null_string() {
		if extension.is_empty() {
			break;
		}

		while let Ok( folder ) = file.read_null_string() {
			if folder.is_empty() {
				break;
			}

			while let Ok( filename ) = file.read_null_string() {
				if filename.is_empty() {
					break;
				}

				let path = format!( "{}/{}.{}", folder, filename, extension );

				let crc = file.read_u32::<LittleEndian>()?;

				let preload_bytes = file.read_u16::<LittleEndian>()?;

				let archive_index = file.read_u16::<LittleEndian>()?;

				let entry_offset = file.read_u32::<LittleEndian>()?; // 0x7fff == embedded

				let entry_length = file.read_u32::<LittleEndian>()?;

				let terminator = file.read_u16::<LittleEndian>()?; // must be 0xffff
				if terminator != 0xffff {
					return Err( VpkError::InvalidEntryTerminator( terminator ) );
				}

				let mut preloaded_bytes = vec![0 as u8; preload_bytes as usize];
				file.read_exact( &mut preloaded_bytes )?;

				let entry = VpkDirEntry {
					path,
					crc,
					preload_bytes,
					archive_index,
					entry_offset,
					entry_length,
					preloaded_bytes
				};

				entries.push( entry );
				println!( "{:?}", entries.last().unwrap() );
			}
		}
	}

	Ok(())
}

#[cfg(test)]
mod tests {
	use crate::{load, VpkError};

	#[test]
    fn open_vpk_v1() -> Result<(), VpkError> {
		load( "C:/Program Files (x86)/Steam/steamapps/common/Portal 2/portal2_dlc4/pak01_dir.vpk" )
	}

	#[test]
    fn open_vpk_v2() -> Result<(), VpkError> {
		load( "D:/SteamLibrary/steamapps/common/Black Mesa/hl2/hl2_materials_dir.vpk" )
	}
}
