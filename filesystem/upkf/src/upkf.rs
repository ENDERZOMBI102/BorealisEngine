struct UpkfFileHeader {
	magic: u32, // 0x55_50_4B_46 'UPKF'
	version: u8,
}

struct UpkfHeaderV1 {
	file_header: UpkfFileHeader,
	origin_size: u16,
	origin: String,
	header_crc: u32,
}

struct UpkfExtDecl {
	extension_size: u8,
	extension: String,
	directory_count: u16,
	directories: Vec<UpkfDirDecl>,
}

struct UpkfDirDecl {
	path_size: u16,
	path: String,
	entry_count: u16,
	entries: Vec<UpkfEntryDecl>,
}

struct UpkfEntryDecl {
	name_size: u8,
	name: String,
	compression_type: CompressionType,
	size: u32,
	crc: u32,
	sha: u32,
	data: Data, // if size > u8, this contains the index of the data-only pak file containing the data and its offset
}
#[repr(transparent)]
enum Data {
	Inline( Vec<u8> ),
	External { index: u8, offset: u64 }
}

#[repr(u8)]
enum CompressionType {
	None,
	LZMA,
	LZMA2,
	GZIP
}
