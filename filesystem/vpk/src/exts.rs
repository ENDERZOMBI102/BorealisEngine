use byteorder::ReadBytesExt;

pub trait ReadNullStringExt: std::io::Read {
	#[inline]
	fn read_null_string( &mut self ) -> std::io::Result<String> {
		let mut string = String::new();

		loop {
			match self.read_u8()? {
				0x00 => break,
				cur => string.push( cur as char )
			}
		}

		Ok( string )
	}
}

impl<R: std::io::Read + ?Sized> ReadNullStringExt for R { }
