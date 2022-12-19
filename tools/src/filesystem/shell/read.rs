use filesystem::layered::LayeredFS;

pub(crate) fn readHandler(fs: &mut LayeredFS, mut args: Vec<&str>, currentDir: &mut String ) {
	match args.as_slice() {
		[ "read", path ] => match fs.get_file(path) {
			Ok( file ) => match file.read_string() {
				Ok( data ) => println!( "{}", data ),
				Err( err ) => println!( "read: Failed to read file \"{}\": {}", path, err )
			},
			_ => eprintln!( "File {path} was not found" )
		},
		_ => eprintln!( "usage: read $PATH" )
	}
}
