use filesystem::layered::LayeredFS;

pub(crate) fn lsHandler( fs: &mut LayeredFS, mut args: Vec<&str>, currentDir: &mut String ) {
	args.remove(0); // remove "layer" prefix
	let path = match args.as_slice() {
		[ rawPath ] => rawPath,
		// TODO: This
		_ => None.unwrap()
	};
}

pub(crate) fn cdHandler( fs: &mut LayeredFS, mut args: Vec<&str>, currentDir: &mut String ) {
	match args.as_slice() {
		[ "cd", path ] => {
			// TODO: This
		}
		_ => None.unwrap()
	}
}

pub(crate) fn findHandler( fs: &mut LayeredFS, mut args: Vec<&str>, currentDir: &mut String ) {
	match args.as_slice() {
		[ "find", path ] => match fs.resolve( path ) {
			None => eprintln!( "find: cannot find path \"{}\"", path ),
			Some( path ) => println!( "{}", path.to_str().unwrap() )
		},
		_ => eprintln!("usage: find $PATH")
	}
}

pub(crate) fn hasHandler( fs: &mut LayeredFS, mut args: Vec<&str>, currentDir: &mut String ) {
	match args.as_slice() {
		// FIXME: This errors
		// [ "has", path ] => {
		// 	match fs.get_file( path ) {
		// 		Ok( file ) => println!( "File `{}` found in layer {}", file.path(), fs.find_layer( file.layer() ).unwrap().meta().filename ),
		// 		Err(_) => {}
		// 	}
		// },
		_ => eprintln!("usage: has $PATH")
	}
}
