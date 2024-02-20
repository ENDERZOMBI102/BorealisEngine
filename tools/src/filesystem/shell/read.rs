use filesystem::layered::LayeredFS;

pub(crate) fn readHandler(fs: &mut LayeredFS, mut argv: Vec<&str>, cwd: &mut String ) {
	match argv.as_slice() {
		// FIXME: This errors
		// [ "read", path ] => match fs.get_file(path) {
		// 	Ok( file ) => match file.read_string() {
		// 		Ok( data ) => println!( "{}", data ),
		// 		Err( err ) => println!( "read: Failed to read file \"{}\": {}", path, err )
		// 	},
		// 	_ => eprintln!( "File {path} was not found" )
		// },
		_ => eprintln!( "usage: read $PATH" )
	}
}
