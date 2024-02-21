module bpak;
/**
 *          BPAK Archive Spec V2           *
 * This file defines how BPAK archives are *
 *   composed on disk, with each section   *
 *                                         *
 * --- Structure of a main archive file -- *
 *  - Versioned Main Header                *
 *  - Ext[]                                *
 *    - Path[]                             *
 *      - Entry[]                          *
 *                                         *
 * --- Structure of an auxiliary file ---- *
 *  - Versioned Auxiliary Header           *
 *  - ....                                 *
 */

// -------- COMMON --------
/**
 * The file header, every BPK archive starts with this.
 */
align(1)
struct BPKFileHeader {
	int  magic; // 0x42_50_4B_46 'BPKF'
	byte ver;   // The version of the format, currently `1`
}

// -------- MAIN ARCHIVE --------
/**
 * The header v1 for the main archive.
 * May be expanded with further iterations, but those would be expanded
 * with ulterior header structs which include the lower ones.
 */
align(1)
struct BPKMainHeaderV1 {
	BPKFileHeader fileHeader; // Include the file header, this is how the compatibility with earlier versions work.
	uint  originSize;         // The size of the "origin" field.
	char[] origin;            // An array of ASCII characters with size $originSize; The origin is usually
	                          // either the game's or mod's name.
	ushort metadataSize;      // The size of the "metadata" field.
	char[] metadata;          // Used to contain additional information with this archive, when added,
	                          // it was thought this could be used for json data with mod/game information.
	uint   headerCrc;         // The CRC32 of the header, this this doesn't match impls should treat the file as illegal.
}

/**
 * Represents an extension seen in the archive.
 * Paths are grouped by extension, to ease loading of multiple files
 * with the same extension, useful, for example, for loading scripts,
 * which all share the same extension and most probably will be loaded
 * at the same time.
 */
align(1)
struct BPKExtDecl {
	ubyte  extensionSize; // The size of the "extension" field.
	char[] extension;     // An array of ASCII characters with size $extensionSize;
	                      // it represents the common extension of the contained entries.
	ushort pathsCount;    // How many paths are expected to be present.
	BPKPathDecl[] paths;  // Paths, which in turn store the entries.
}

/**
 * Represents a "directory hierarchy", aka a path.
 * Entries are grouped by path, to avoid gross repetition of long names.
 * This also has the same benefit as highlighted in the BPKExtDecl comment.
 */
align(1)
struct BPKPathDecl {
	ushort pathSize;        // The size of the "path" field.
	char[] path;            // The path of this "directory", including its name.
	ushort entryCount;      // How many entries are expected to be present.
	BPKEntryDecl[] entries; // Entries, which in turn store either the data, or its location.
}

/**
 * Algorithm which was used to compress an entry's data.
 */
enum CType {
	None = 0,
	LZHAM,
	GZIP
}

/**
 * Rerepresents either the contents of the entry itself,
 * or its location in the auxiliary archive files.
 */
union Content {
	ubyte[]  bytes;    // Contents of the entry.
	Location location; // Location of the data.
}

/**
 * Represents the location of an entry's contents
 * in the auxiliary files.
 */
align(1)
struct Location {
	ubyte index;  // Index of the auxiliary file.
	ulong offset; // Offset from the start of the file where the data is located.
}

/**
 * Represents a file, which may be loaded from disk.
 * If either of the crc32 or sha256 don't match the one which may be calculated
 * for the data on disk implementation should not accept to return the contents,
 * and avoid loading anything that comes after this entry, given that the contents
 * are all in the same auxiliary file.
 */
align(1)
struct BPKEntryDecl {
	ubyte  nameSize;         // The size of the "name" field.
	char[] name;             // The name of the file.
	uint   size;             // The size in bytes of the contents on disk.
 bool   inlined;          // Wether the data has been inlined in the main archive or is in an auxiliary file.
	CType  compressionType;  // The compression method which was used for this file's contents.
	ubyte  compressionLevel; // The compression level this file's contents have been compressed with,
	                         // the method may not use this, and in which case it should be set to 255.
	uint   decompSize;       // The size in bytes of the uncompressed contents. May be used to
	                         // pre-allocate a buffer with enough space for all at once.
	uint   crc32;            // The crc32 of the entry's contents.
	char[64] sha256;         // The sha256 of the entry's contents.
	Content content;         // The data or location of the contents.
}

// -------- AUXILIARY FILE --------
/**
 * The header v1 for an auxiliary file.
 * May be expanded with further iterations, but those would be expanded
 * with ulterior header structs which include the lower ones.
 */
align(1)
struct BPKAuxiliaryHeaderV1 {
	BPKFileHeader fileHeader; // Include the file header, this is how the compatibility with earlier versions work.
	ushort pakNameSize;       // The size of the "pakName" field.
	char[] pakName;           // An array of ASCII characters with size $originSize; Contains the name of the
	                          // archive this file contains the data of.
	uint headerCrc;           // The CRC32 of the header, this this doesn't match impls should treat the file as illegal.
}

// Needed by the D compiler, which is used to check for errors,
// please ignore.
void main() { }

