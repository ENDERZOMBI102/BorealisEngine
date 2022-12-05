use filesystem::layered::LayeredFS;

pub(crate) fn lsHandler( fs: &mut LayeredFS, mut args: Vec<&str>, currentDir: &mut String ) {
	args.remove(0); // remove "layer" prefix
	let path = match args.as_slice() {
		[ rawPath ] => rawPath,
		// TODO: This
	};
}

pub(crate) fn cdHandler( fs: &mut LayeredFS, mut args: Vec<&str>, currentDir: &mut String ) {
	match command.as_slice() {
		[ "cd", path ] => {
			// TODO: This
		}
	}
}

pub(crate) fn findHandler( fs: &mut LayeredFS, mut args: Vec<&str>, currentDir: &mut String ) {
	match command.as_slice() {
		[ "find", path ] => match fs.resolve( path ) {
			None => eprintln!( "find: cannot find path \"{}\"", path ),
			Some( path ) => println!( "{}", path.to_str().unwrap() )
		},
		_ => eprintln!("usage: find $PATH")
	}
}

pub(crate) fn hasHandler( fs: &mut LayeredFS, mut args: Vec<&str>, currentDir: &mut String ) {
	match command.as_slice() {
		[ "has", path ] => {
			match fs.get_file( path ) {
				Ok( file ) => println!( "File `{}` found in layer {}", file.path(), file.layer().meta().filename ),
				Err(_) => {}
			}
		},
		_ => eprintln!("usage: has $PATH")
	}
}
