mod exts;

use std::collections::HashMap;
use std::fs::File;
use std::io::Read;
use std::mem::size_of;
use std::path::{Path, PathBuf};

use thiserror::Error;
use zerocopy::{FromBytes, AsBytes};
use crate::exts::ReadNullStringExt;

#[derive(Error, Debug)]
pub enum VpkError {
	#[error("some IO error occurred")]
    IoError(#[from] std::io::Error),
	#[error("unsupported VPK version")]
	UnsupportedVersion(u32),
	#[error("invalid vpk directory entry terminator")]
	InvalidEntryTerminator(u16),
	#[error("invalid vpk signature")]
	InvalidSignature(u32),
	#[error("vpk file has no parent directory")]
	NoParentDir,
	#[error("failed to read vpk directory entry")]
	FailedReadDirEntry,
	#[error("unknown data store error")]
	Unknown
}

#[repr(C, packed)]
#[derive(FromBytes, AsBytes)]
pub struct VpkHeaderV1 {
	signature: u32, // 0x55aa1234
	version: u32,   // [1|2]

	/// The size, in bytes, of the directory tree
	tree_length: u32
}

#[repr(C, packed)]
#[derive(FromBytes, AsBytes)]
pub struct VpkHeaderV2 {
	v1: VpkHeaderV1,

	/// How many bytes of file content are stored in this VPK file (0 in CSGO)
	file_data_section_size: u32,
	/// The size, in bytes, of the section containing MD5 checksums for external archive content
	archive_md5_section_size: u32,
	/// The size, in bytes, of the section containing MD5 checksums for content in this file (should always be 48)
	other_md5_section_size: u32,
	/// The size, in bytes, of the section containing the public key and signature. This is either 0 (CSGO & The Ship) or 296 (HL2, HL2:DM, HL2:EP1, HL2:EP2, HL2:LC, TF2, DOD:S & CS:S)
	signature_section_size: u32
}

#[repr(C, packed)]
#[derive(Debug, FromBytes, AsBytes)]
struct VpkDirEntry {
	/// A 32bit CRC of the file's data.
	crc: u32,
	/// The number of bytes contained in the index file.
	preload_bytes: u16,

	/// A zero based index of the archive this file's data is contained in.
	/// If 0x7fff, the data follows the directory.
	archive_index: u16,
	/// If `archive_index` is 0x7fff, the offset of the file data relative to the end of the directory (see the header for more details).
	/// Otherwise, the offset of the data from the start of the specified archive.
	entry_offset: u32,
	/// If zero, the entire file is stored in the preload data.
	/// Otherwise, the number of bytes stored starting at `entry_offset`.
	entry_length: u32,

	terminator: u16 // 0xffff
}

#[derive(Debug)]
pub struct VpkFile {
	entry: VpkDirEntry
}

#[derive(Debug)]
pub struct Vpk {
	version: u8,
	path: PathBuf,
	base_path: PathBuf,
	files: HashMap<PathBuf, VpkFile>,
}

impl Vpk {
	pub fn load<P: AsRef<Path>>( path: P ) -> Result<Self, VpkError> {
		let mut file = File::open( path.as_ref() )?;

		let mut header_bytes = [ 0; size_of::<VpkHeaderV2>() ];
		file.read( &mut header_bytes[ .. size_of::<VpkHeaderV1>() ] )?;

		let v1 = VpkHeaderV1::read_from_prefix( header_bytes.as_slice() ).ok_or( VpkError::Unknown )?;

		if v1.signature != 0x55aa1234 {
			return Err( VpkError::InvalidSignature( v1.signature ) )
		}

		let mut vpk = Vpk {
			version: v1.version as u8,
			path: path.as_ref().to_path_buf(),
			base_path: path.as_ref().parent().ok_or( VpkError::NoParentDir )?.to_path_buf(),
			files: Default::default()
		};

		match v1.version {
			1 => vpk.load_v1( &mut file, v1 )?,
			2 => {
				file.read( &mut header_bytes[ size_of::<VpkHeaderV1>() .. ] )?;

				let v2 = VpkHeaderV2::read_from_prefix( header_bytes.as_slice() ).ok_or( VpkError::Unknown )?;

				vpk.load_v2( &mut file, v2 )?
			},
			it => return Err( VpkError::UnsupportedVersion( it ) )
		}

		Ok( vpk )
	}

	fn load_v1( &mut self, file: &mut File, v1: VpkHeaderV1 ) -> Result<(), VpkError> {
		self.load_tree( file )?;

		Ok(())
	}

	fn load_v2( &mut self, file: &mut File, v2: VpkHeaderV2 ) -> Result<(), VpkError> {
		self.load_tree( file )?;

		Ok(())
	}

	fn load_tree( &mut self, file: &mut File ) -> Result<(), VpkError> {
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

					let mut bytes = [ 0; size_of::<VpkDirEntry>() ];
					file.read( &mut bytes )?;

					let entry = VpkDirEntry::read_from( bytes.as_slice() ).ok_or( VpkError::FailedReadDirEntry )?;

					if entry.terminator != 0xffff {
						return Err( VpkError::InvalidEntryTerminator( entry.terminator ) );
					}

					// let mut preloaded_bytes = vec![0 as u8; entry.preload_bytes as usize];
					// file.read( &mut preloaded_bytes )?;

					self.files.insert( PathBuf::from( format!( "{}/{}.{}", folder, filename, extension ) ), VpkFile { entry } );
				}
			}
		}

		Ok(())
	}
}

#[cfg(test)]
mod tests {
	use crate::{Vpk, VpkError};

	#[test]
    fn open_vpk_v1() -> Result<(), VpkError> {
		Vpk::load( "C:/Program Files (x86)/Steam/steamapps/common/Portal 2/portal2_dlc4/pak01_dir.vpk" )?;

		Ok(())
	}

	#[test]
    fn open_vpk_v2() -> Result<(), VpkError> {
		Vpk::load( "D:/SteamLibrary/steamapps/common/Black Mesa/hl2/hl2_materials_dir.vpk" )?;

		Ok(())
	}
}
